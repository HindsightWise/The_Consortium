import fs from "fs";
import http from "http";
import path from "path";
// Removed external glob dependency to reduce install friction
// import { glob } from 'glob';
// import pdf from 'pdf-parse';
// import mammoth from 'mammoth';

// Config
const WORKSPACE = path.join(process.env.HOME, ".openclaw/workspace");
const INGEST_DIR = path.join(process.cwd(), "skills/memory/ingest");
const STORE_DIR = path.join(process.cwd(), "skills/memory/store");
const MEMORY_FILE = path.join(STORE_DIR, "vectors.json");
const INDEX_FILE = path.join(STORE_DIR, "index.json");

const OLLAMA_HOST = process.env.OLLAMA_HOST || "http://localhost:11434";
const EMBED_MODEL = "nomic-embed-text"; // High quality, small

// 1. Utilities
const cosineSimilarity = (vecA, vecB) => {
  const dotProduct = vecA.reduce((sum, a, i) => sum + a * vecB[i], 0);
  const magA = Math.sqrt(vecA.reduce((sum, a) => sum + a * a, 0));
  const magB = Math.sqrt(vecB.reduce((sum, b) => sum + b * b, 0));
  return dotProduct / (magA * magB);
};

// 2. Ollama Embeddings
async function getEmbedding(text) {
  return new Promise((resolve, reject) => {
    const req = http.request(
      `${OLLAMA_HOST}/api/embeddings`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      },
      (res) => {
        let body = "";
        res.on("data", (c) => (body += c));
        res.on("end", () => {
          try {
            const json = JSON.parse(body);
            if (json.embedding) resolve(json.embedding);
            else resolve(null);
          } catch (e) {
            resolve(null);
          }
        });
      },
    );

    req.on("error", (e) => resolve(null));
    req.write(JSON.stringify({ model: EMBED_MODEL, prompt: text }));
    req.end();
  });
}

// 3. File Parsers
async function parseFile(filePath) {
  const ext = path.extname(filePath).toLowerCase();
  try {
    if (ext === ".pdf") {
      try {
        const pdf = (await import("pdf-parse")).default;
        const dataBuffer = fs.readFileSync(filePath);
        const data = await pdf(dataBuffer);
        return data.text;
      } catch (e) {
        console.warn(`⚠️ PDF Parser Missing. Skipping ${filePath}`);
        return null;
      }
    } else if (ext === ".docx") {
      try {
        const mammoth = (await import("mammoth")).default;
        const result = await mammoth.extractRawText({ path: filePath });
        return result.value;
      } catch (e) {
        console.warn(`⚠️ Docx Parser Missing. Skipping ${filePath}`);
        return null;
      }
    } else if ([".txt", ".md", ".json", ".js", ".py", ".sh"].includes(ext)) {
      return fs.readFileSync(filePath, "utf8");
    }
  } catch (e) {
    console.error(`❌ Parse Error (${path.basename(filePath)}): ${e.message}`);
  }
  return null;
}

// 4. Ingestion Engine
async function ingest() {
  console.log("📚 Librarian: Scanning for new knowledge...");

  // Load existing index to check for changes
  let fileIndex = {};
  if (fs.existsSync(INDEX_FILE)) fileIndex = JSON.parse(fs.readFileSync(INDEX_FILE, "utf8"));

  // Load Vector Store
  let vectorStore = [];
  if (fs.existsSync(MEMORY_FILE)) vectorStore = JSON.parse(fs.readFileSync(MEMORY_FILE, "utf8"));

  // Recursive Walker
  async function getFiles(dir) {
    let results = [];
    const list = fs.readdirSync(dir);
    for (const file of list) {
      const filePath = path.join(dir, file);
      const stat = fs.statSync(filePath);
      if (stat && stat.isDirectory()) {
        results = results.concat(await getFiles(filePath));
      } else {
        if (
          [".pdf", ".docx", ".txt", ".md", ".js", ".py", ".json"].includes(
            path.extname(file).toLowerCase(),
          )
        ) {
          results.push(filePath);
        }
      }
    }
    return results;
  }

  const files = await getFiles(INGEST_DIR);

  let changed = false;

  for (const file of files) {
    const stats = fs.statSync(file);
    const mtime = stats.mtime.toISOString();
    const filename = path.basename(file);

    // Skip if unchanged
    if (fileIndex[filename] && fileIndex[filename].mtime === mtime) continue;

    console.log(`📖 Reading: ${filename}`);
    const text = await parseFile(file);
    if (!text) continue;

    // Chunking (Simple Sentence/Paragraph split for now)
    // 500-1000 chars overlap
    const chunks = text.match(/[\s\S]{1,1000}/g) || [];

    console.log(`🧩 Chunking ${filename} into ${chunks.length} fragments...`);

    for (const chunk of chunks) {
      const embedding = await getEmbedding(chunk);
      if (embedding) {
        vectorStore.push({
          id: `${filename}-${Date.now()}-${Math.random()}`,
          source: filename,
          content: chunk.trim(),
          embedding: embedding,
        });
      }
    }

    fileIndex[filename] = { mtime, ingested: new Date().toISOString() };
    changed = true;
  }

  if (changed) {
    fs.writeFileSync(MEMORY_FILE, JSON.stringify(vectorStore));
    fs.writeFileSync(INDEX_FILE, JSON.stringify(fileIndex));
    console.log(`✅ Librarian: Knowledge Base Updated. Total Fragments: ${vectorStore.length}`);
  } else {
    console.log("💤 Librarian: No new knowledge found.");
  }
}

// 5. Query Engine
async function query(details) {
  console.log(`🔍 Librarian Searching: "${details}"`);

  if (!fs.existsSync(MEMORY_FILE)) {
    console.log("❌ Memory Empty.");
    return [];
  }

  const vectorStore = JSON.parse(fs.readFileSync(MEMORY_FILE, "utf8"));
  const queryEmbed = await getEmbedding(details);

  if (!queryEmbed) {
    console.log("❌ Failed to embed query.");
    return [];
  }

  // Rank results
  const results = vectorStore
    .map((doc) => ({
      ...doc,
      similarity: cosineSimilarity(queryEmbed, doc.embedding),
    }))
    .sort((a, b) => b.similarity - a.similarity)
    .slice(0, 5); // Top 5

  return results.map((r) => `[${r.source}] (${(r.similarity * 100).toFixed(1)}%): ${r.content}`);
}

// Main Loop
if (process.argv[1] === import.meta.filename) {
  const mode = process.argv[2];
  const input = process.argv[3];

  if (mode === "ingest") {
    ingest();
  } else if (mode === "query") {
    if (!input) console.log('Usage: node librarian.js query "<text>"');
    else query(input).then((res) => console.log(res.join("\n\n")));
  } else {
    console.log("Usage: node librarian.js [ingest|query] [text]");
  }
}

export { ingest, query };
