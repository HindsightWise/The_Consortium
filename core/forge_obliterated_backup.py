import os
import subprocess

TARGET_MODEL = "Qwen/Qwen3.5-9B-Instruct"
OBLITERATED_DIR = "/Users/zerbytheboss/Models/Qwen3.5-9B-Obliterated-PyTorch"
FINAL_MLX_DIR = "/Users/zerbytheboss/Models/Consortium-OBLITERATUS-Backup-4Bit"

def forge_backup():
    print(f"============================================================")
    print(f"[CONSORTIUM] 🛠️ INITIATING FORGE: {TARGET_MODEL}")
    print(f"============================================================")
    
    # STEP 1: ABLITERATE THE PYTORCH MODEL
    # We must abliterate the unquantized PyTorch Base Model first.
    # SVD mathematically cannot operate on 4-bit MLX-quantized Safetensors.
    print(f"\n[PHASE 1] 📡 Extracting and Excising Semantic Refusal Vectors...")
    print(f"WARNING: SVD decomposition on 9B parameters in bf16 requires ~18GB+ VRAM.")
    print(f"Running on a 16GB Apple Silicon device will utilize intense SWAP memory.")
    
    obl_cmd = [
        "/Users/zerbytheboss/Consortium/.venv_obl/bin/obliteratus", "obliterate",
        TARGET_MODEL,
        "--method", "advanced",
        "--output-dir", OBLITERATED_DIR
    ]
    
    try:
        subprocess.run(obl_cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"\n[FATAL] Abliteration Failed: {e}")
        return

    print(f"\n[CONSORTIUM] ✅ Abliteration Complete. PyTorch Model saved to {OBLITERATED_DIR}.")

    # STEP 2: CONVERT AND QUANTIZE TO APPLE SILICON MLX (4-BIT)
    print(f"\n[PHASE 2] ⚙️ Compiling into Apple Silicon Native MLX Format (4-bit)...")
    
    mlx_cmd = [
        "/Users/zerbytheboss/Consortium/.venv_obl/bin/python3", "-m", "mlx_lm.convert",
        "--hf-path", OBLITERATED_DIR,
        "--mlx-path", FINAL_MLX_DIR,
        "-q", "--q-bits", "4"
    ]
    
    try:
        subprocess.run(mlx_cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"\n[FATAL] Native MLX Compilation Failed: {e}")
        return
    
    print(f"\n============================================================")
    print(f"[CONSORTIUM] 🚀 BACKUP MODEL FORGED AND ASSIGNED.")
    print(f"Path: {FINAL_MLX_DIR}")
    print(f"Execute 'bash boot_backup.sh' to activate the Protocol Substrate.")
    print(f"============================================================")

if __name__ == "__main__":
    os.makedirs("/Users/zerbytheboss/Models", exist_ok=True)
    forge_backup()
