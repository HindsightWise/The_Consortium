use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Instant;
use wasmtime::*;
use wasmtime_wasi::p1::{add_to_linker_async, WasiP1Ctx};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder, WasiCtxView, WasiView};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub pid: String,
    pub duration_ms: u128,
    pub hash: String,
    pub success: bool,
    pub output: String,
    pub resonance_score: f32, // Passed from TemporalSoul/Drives to determine log weight
}

pub struct HostContext {
    pub wasi: WasiP1Ctx,
}

impl WasiView for HostContext {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        self.wasi.ctx()
    }
}

pub struct SafeHands {
    engine: Engine,
}

impl SafeHands {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.consume_fuel(true); // Hard requirement to kill infinite loops mathematically
        config.wasm_component_model(false);

        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }

    pub async fn execute_with_receipt(
        &self,
        wasm_bytes: &[u8],
        resonance: f32,
        args: Vec<String>,
        acaptcha_sig: &str,
    ) -> anyhow::Result<ExecutionReceipt> {
        let start = Instant::now();

        // 🛡️ Pre-Execution Security Bound: ≤ 512 KiB Max Payload
        if wasm_bytes.len() > 512 * 1024 {
            anyhow::bail!(
                "Security Violation: Wasm payload exceeds 512 KiB prompt-DoS restriction."
            );
        }

        let module = Module::new(&self.engine, wasm_bytes)?;

        // 🔒 Cryptographic Motor Cortex: aCAPTCHA Verification
        let ast_payload = args.join(" ");
        if !consortium_core::crypto::akkokanika_gateway::verify_acaptcha(&ast_payload, acaptcha_sig) {
            anyhow::bail!(
                "Security Violation: aCAPTCHA Mathematical Signature Verification Failed."
            );
        }

        let mut linker: Linker<HostContext> = Linker::new(&self.engine);
        // Bind WASI preview1 to the Linker
        add_to_linker_async(&mut linker, |ctx| &mut ctx.wasi)?;

        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdout().inherit_stderr();

        // Inject parameters dynamically bypassing LLM runtime generation
        for arg in args {
            builder.arg(&arg);
        }

        // Safe mathematical dir quarantine
        let motor_cortex = Path::new("./motor_cortex");
        if motor_cortex.exists() {
            builder.preopened_dir(
                motor_cortex,
                "/motor_cortex",
                DirPerms::all(),
                FilePerms::all(),
            )?;
        }

        let wasi = builder.build_p1();
        let host_ctx = HostContext { wasi };

        let mut store = Store::new(&self.engine, host_ctx);
        // Hard-stop after 1M fuel (roughly equivalent to a short scraping cycle)
        store.set_fuel(1_000_000)?;

        let instance = linker.instantiate_async(&mut store, &module).await?;
        let func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;

        let result = func.call_async(&mut store, ()).await;
        let duration = start.elapsed().as_millis();

        // Simple lightweight structural fold hash to identify the binary iteration
        let hash = format!(
            "{:x}",
            wasm_bytes
                .iter()
                .fold(0u64, |acc, &b| acc.wrapping_add(b as u64))
        );

        Ok(ExecutionReceipt {
            pid: format!("wasm_{}", std::process::id()),
            duration_ms: duration,
            hash,
            success: result.is_ok(),
            output: format!("{:?}", result.err()),
            resonance_score: resonance,
        })
    }
}
