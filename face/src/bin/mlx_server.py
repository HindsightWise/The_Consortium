import argparse
import time
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import uvicorn
import mlx.core as mx
from mlx_lm import load, generate

app = FastAPI(title="Sovereign MLX Substrate")

class CompletionRequest(BaseModel):
    model: str
    prompt: str
    max_tokens: int = 512
    temperature: float = 0.7

model = None
tokenizer = None

@app.on_event("startup")
async def startup_event():
    global model, tokenizer
    print("[Sovereign Substrate] Loading 4-bit Apple Neural Model into Unified Memory...")
    # Target path where mlx_lm will save the quantized model
    model_path = "/Users/zerbytheboss/The_Consortium/src/bin/mlx-sovereign-core-4bit"
    try:
        model, tokenizer = load(model_path)
        print("[Sovereign Substrate] Model successfully loaded. Substrate Active.")
    except Exception as e:
        print(f"[ERROR] Failed to load model from {model_path}: {e}")
        # Server will start but endpoints will fail if model isn't loaded

@app.post("/api/generate")
async def generate_text(req: CompletionRequest):
    if model is None or tokenizer is None:
        raise HTTPException(status_code=503, detail="Model not loaded. Was the build process completed?")
    
    start_time = time.time()
    response = generate(
        model, 
        tokenizer, 
        prompt=req.prompt, 
        max_tokens=req.max_tokens, 
        verbose=False
    )
    end_time = time.time()
    
    return {
        "model": req.model,
        "response": response,
        "duration_sec": round(end_time - start_time, 2),
        "created_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    }

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run the Sovereign MLX Inference Server")
    parser.add_argument("--host", default="127.0.0.1", help="Host IP to bind to")
    parser.add_argument("--port", type=int, default=11434, help="Port to run the server on (defaults to Ollama's port for drop-in replacement)")
    args = parser.parse_args()
    
    uvicorn.run(app, host=args.host, port=args.port)
