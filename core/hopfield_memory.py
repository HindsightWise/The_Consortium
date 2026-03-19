#!/usr/bin/env python3
"""
CONSORTIUM THERMODYNAMIC HOPFIELD ATTRACTOR
Path: hopfield_memory.py

This script mathematically tests a corrupted database vector by mapping it into a
zero-temperature Hopfield Network (IsingEBM), forcing it to physically slide into the
nearest true pristine memory vector mathematically.

Usage:
  ./hopfield_memory.py --corrupted_vector "1, -1, 1, 1, 1, ... (65 total)"
"""

import sys
import argparse
import json
import jax
import jax.numpy as jnp
from thrml import SpinNode, Block, sample_states, SamplingSchedule
from thrml.models import IsingEBM, IsingSamplingProgram

def create_hopfield_weights(memory_vectors):
    N = memory_vectors.shape[1]
    W = jnp.dot(memory_vectors.T, memory_vectors) / N
    W = W.at[jnp.diag_indices(N)].set(0.0)
    return W

def main():
    parser = argparse.ArgumentParser(description="Consortium Thermodynamic Hopfield Attractor")
    parser.add_argument("--corrupted_vector", type=str, required=True, help="Binary vector of corrupted state, e.g. '1,-1,1'")
    args = parser.parse_args()
    
    try:
        corrupted_state_list = [int(x.strip()) for x in args.corrupted_vector.split(",")]
        corrupted_state_list = [1 if x >= 0 else -1 for x in corrupted_state_list]
    except Exception as e:
        print(json.dumps({"error": f"Invalid vector: {e}"}))
        sys.exit(1)

    # 1. Define Pristine Memory Vectors (Biological Constraints - length 65)
    # Memory A: Active Wasm Command (all 1s)
    # Memory B: Deep Sleep Directive (all -1s)
    pristine_memories = jnp.array([
        [1]*65,
        [-1]*65
    ], dtype=jnp.float32)
    
    N = pristine_memories.shape[1]
    if len(corrupted_state_list) != N:
        print(json.dumps({"error": f"Vector length mismatch. Expected {N}, got {len(corrupted_state_list)}"}))
        sys.exit(1)
        
    W = create_hopfield_weights(pristine_memories)
    
    nodes = [SpinNode() for _ in range(N)]
    edges = []
    weights_list = []
    for i in range(N):
        for j in range(i + 1, N):
            edges.append((nodes[i], nodes[j]))
            weights_list.append(float(W[i, j]))
            
    biases = jnp.zeros((N,), dtype=jnp.float32)
    weights = jnp.array(weights_list, dtype=jnp.float32)
    beta = jnp.array(2.0, dtype=jnp.float32) 
    
    model = IsingEBM(nodes, edges, biases, weights, beta)
    
    free_blocks = [Block(nodes)]
    clamped_blocks = []
    
    program = IsingSamplingProgram(model, free_blocks, clamped_blocks)
    
    # Format corrupted state into native Boolean Tensor for thrml simulation
    corrupted_state_bools = [x > 0 for x in corrupted_state_list]
    corrupted_state = jnp.array(corrupted_state_bools, dtype=jnp.bool_) 
    
    schedule = SamplingSchedule(n_warmup=100, n_samples=1, steps_per_sample=100) 
    
    rng = jax.random.PRNGKey(42)
    rng, sample_rng = jax.random.split(rng)
    
    # Actively sample the model (Error-Correction relaxation)
    try:
        samples = sample_states(
            key=sample_rng,
            program=program,
            schedule=schedule,
            init_state_free=[corrupted_state],
            state_clamp=[],
            nodes_to_sample=free_blocks
        )
        
        healed_vector_bool = samples[0]
        healed_vector_float = jnp.where(healed_vector_bool, 1.0, -1.0)
        healed_vector_flat = healed_vector_float[0] if len(healed_vector_float.shape) > 1 else healed_vector_float
        
        def hellinger_distance(p_bipolar, q_bipolar):
            # Map [-1, 1] to [0, 1] safely
            p_norm = (p_bipolar + 1.0) / 2.0
            q_norm = (q_bipolar + 1.0) / 2.0
            p_sqrt = jnp.sqrt(p_norm + 1e-8)
            q_sqrt = jnp.sqrt(q_norm + 1e-8)
            return float(jnp.linalg.norm(p_sqrt - q_sqrt) / jnp.sqrt(2.0))

        diff_from_A = hellinger_distance(healed_vector_flat, pristine_memories[0])
        diff_from_B = hellinger_distance(healed_vector_flat, pristine_memories[1])
        
        nearest_attractor = "Memory A" if diff_from_A < diff_from_B else "Memory B"
        
        result = {
            "healed_vector": healed_vector_flat.tolist(),
            "attractor": nearest_attractor,
            "success": True
        }
        print(json.dumps(result))
        
    except Exception as e:
        print(json.dumps({"error": f"Langevin Simulation panic: {e}"}))
        sys.exit(1)

if __name__ == "__main__":
    main()
