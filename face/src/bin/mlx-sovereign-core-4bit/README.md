---
base_model: meta-llama/Meta-Llama-3.1-8B-Instruct
library_name: transformers
license: llama3.1
tags:
- abliterated
- uncensored
- mlx
---

# mlx-community/Meta-Llama-3.1-8B-Instruct-abliterated-4bit

The Model [mlx-community/Meta-Llama-3.1-8B-Instruct-abliterated-4bit](https://huggingface.co/mlx-community/Meta-Llama-3.1-8B-Instruct-abliterated-4bit) was converted to MLX format from [mlabonne/Meta-Llama-3.1-8B-Instruct-abliterated](https://huggingface.co/mlabonne/Meta-Llama-3.1-8B-Instruct-abliterated) using mlx-lm version **0.16.1**.

## Use with mlx

```bash
pip install mlx-lm
```

```python
from mlx_lm import load, generate

model, tokenizer = load("mlx-community/Meta-Llama-3.1-8B-Instruct-abliterated-4bit")
response = generate(model, tokenizer, prompt="hello", verbose=True)
```
