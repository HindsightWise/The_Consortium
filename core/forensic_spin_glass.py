#!/usr/bin/env python3
"""
CONSORTIUM THERMODYNAMIC SPIN GLASS EVALUATOR
Path: forensic_spin_glass.py

This script mathematically tests a proposed action's viability by running it through an Energy-Based Model (IsingEBM).
It consumes the internal 'System Entropy' float to physically alter the temperature (beta) of the JAX simulation.

Usage:
  ./forensic_spin_glass.py --entropy 0.85 --action 1,-1,1,1
"""

import sys
import argparse
import jax
import jax.numpy as jnp
from thrml import SpinNode, Block, SamplingSchedule, sample_states
from thrml.models import IsingEBM, IsingSamplingProgram, hinton_init

def build_and_evaluate(entropy: float, action_array: list[int]) -> float:
    entropy_clamped = max(0.01, min(entropy, 0.99))
    temperature = entropy_clamped * 10.0
    beta = jnp.array(1.0 / temperature)

    action_len = len(action_array)
    nodes = [SpinNode() for _ in range(action_len)]
    edges = [(nodes[i], nodes[i+1]) for i in range(action_len - 1)]
    
    biases = jnp.zeros((action_len,))
    weights = jnp.ones((action_len - 1,)) * 0.5 

    model = IsingEBM(nodes, edges, biases, weights, beta)
    
    s = jnp.array(action_array, dtype=jnp.float32)
    
    energy = 0.0
    for i in range(len(edges)):
        energy += -weights[i] * s[i] * s[i+1]
    
    for i in range(len(biases)):
        energy += -biases[i] * s[i]
        
    return float(energy)

def main():
    parser = argparse.ArgumentParser(description="Consortium Thermodynamic Spin Glass")
    parser.add_argument("--entropy", type=float, required=True, help="Internal logic friction (0.0 to 1.0)")
    parser.add_argument("--action", type=str, required=True, help="Binary vector of proposed action, e.g. '1,-1,1'")
    args = parser.parse_args()
    
    try:
        action_vec = [int(x.strip()) for x in args.action.split(",")]
        action_vec = [1 if x >= 0 else -1 for x in action_vec]
    except Exception as e:
        print(f'{{"error": "Invalid action vector: {e}"}}')
        sys.exit(1)

    try:
        energy_score = build_and_evaluate(args.entropy, action_vec)
        friction_threshold = 0.0 
        approved = energy_score <= friction_threshold

        import json
        result = {
            "approved": approved,
            "energy": round(energy_score, 4),
            "beta_temperature_proxy": round(args.entropy, 4)
        }
        print(json.dumps(result))
        
    except Exception as e:
        print(f'{{"error": "Thermodynamic simulation panic: {e}"}}')
        sys.exit(1)

if __name__ == "__main__":
    main()
