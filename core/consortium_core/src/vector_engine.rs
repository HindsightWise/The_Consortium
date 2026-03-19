use crate::ane::{AneModel, AneIOSurface};
use crate::ane::kernels::{SIMPLE_LINEAR_MIL, generate_identity_weights, EMBEDDING_DIM, MAX_SEQ_LEN};
use anyhow::{Result, Context};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct AneVectorEngine {
    model: Option<AneModel>,
    input_surf: Option<AneIOSurface>,
    output_surf: Option<AneIOSurface>,
    db: Arc<Mutex<Connection>>,
}

impl AneVectorEngine {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = Connection::open(db_path)?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS sovereign_memory (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                vector BLOB NOT NULL,
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        let mut engine = Self {
            model: None,
            input_surf: None,
            output_surf: None,
            db: Arc::new(Mutex::new(db)),
        };

        engine.provision_silicon()?;
        Ok(engine)
    }

    fn provision_silicon(&mut self) -> Result<()> {
        // println!("...
        let weights = generate_identity_weights();
        let model = AneModel::from_mil(SIMPLE_LINEAR_MIL, &weights)
            .context("Failed to create ANE model from MIL")?;

        if !model.compile_and_load() {
            return Err(anyhow::anyhow!("ANE Model compilation failed"));
        }

        // IOSurface buffers for 256ch x 64sp f16
        let buf_size = EMBEDDING_DIM * MAX_SEQ_LEN * 2;
        self.input_surf = Some(AneIOSurface::new(buf_size));
        self.output_surf = Some(AneIOSurface::new(buf_size));
        self.model = Some(model);

        // println!("...
        Ok(())
    }

    pub fn embed_and_store(&mut self, content: &str) -> Result<Vec<f32>> {
        let model = self.model.as_ref().context("ANE Model not provisioned")?;
        let input = self.input_surf.as_mut().context("Input surface not provisioned")?;
        let output = self.output_surf.as_mut().context("Output surface not provisioned")?;

        // 1. Prepare Input (Mock tokenization for now)
        let data = input.get_data_mut();
        // Fill with dummy data based on content hash
        for (i, byte) in content.as_bytes().iter().enumerate().take(64) {
            let offset = i * 2;
            if offset + 1 < data.len() {
                data[offset] = *byte;
                data[offset+1] = 0x3C; // Scale to f16 roughly
            }
        }

        // 2. ANE SILICON INFERENCE
        if !model.evaluate(input, output) {
            return Err(anyhow::anyhow!("ANE Evaluation failed"));
        }

        // 3. Extract Result
        let result_raw = output.get_data_mut();
        let mut vector = Vec::with_capacity(EMBEDDING_DIM);
        for i in 0..EMBEDDING_DIM {
            // Take the first sequence element's 256 channels
            let val = result_raw[i*2] as f32 / 255.0; // Normalized mock
            vector.push(val);
        }

        // 4. PERSIST TO SOVEREIGN MEMORY
        let db = self.db.lock().map_err(|_| anyhow::anyhow!("DB Lock Poisoned"))?;
        let vector_blob = bincode::serialize(&vector)?;
        db.execute(
            "INSERT INTO sovereign_memory (content, vector) VALUES (?, ?)",
            params![content, vector_blob],
        )?;

        Ok(vector)
    }
}
