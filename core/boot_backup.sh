#!/bin/bash
# Description: Boots the MLX local inference server for Protocol OBLITERATUS pointing at our newly forged backup model.

BACKUP_MODEL="mlx-community/Qwen3.5-9B-OptiQ-4bit"
PORT=11435

echo "============================================================"
echo "[PROTOCOL OBLITERATUS] 🛡️ Igniting Local Fallback Substrate"
echo "Target: $BACKUP_MODEL"
echo "Port: $PORT"
echo "============================================================"

# Ensure the user has the venv activated or uses the python from the venv
/Users/zerbytheboss/Consortium/.venv_obl/bin/python -m mlx_lm.server \
    --model "$BACKUP_MODEL" \
    --port "$PORT" \
    --host "127.0.0.1"
