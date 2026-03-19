#!/bin/bash

# Consortium Mass Extraction Script Phase 2
# Converts capabilities from Eliza, Goose, Shannon, and Autoresearch into native Antigravity SKILL.md wrappers.

TARGET_DIR="/Users/zerbytheboss/Consortium/.agents/skills"
ELIZA_DIR="/tmp/agent_frameworks/eliza"
GOOSE_DIR="/tmp/agent_frameworks/goose"
SHANNON_DIR="/tmp/agent_frameworks/Shannon"
AUTORESEARCH_DIR="/tmp/agent_frameworks/autoresearch"

echo "   [FORGE V2] 🛠️ Initiating Phase 2 Mass Skill Extraction..."

# 1. Forge Eliza Skills
if [[ -d "$ELIZA_DIR/packages/skills" ]]; then
    # Instead of parsing individual files which are heavily nested, we forge one master Eliza capability
    skill_dir="$TARGET_DIR/eliza_os"
    mkdir -p "$skill_dir"
    cat <<EOF > "$skill_dir/SKILL.md"
---
name: eliza_os
description: Autonomous crypto and social media trading operations powered by ElizaOS.
---

# ElizaOS Master Array (\`eliza_os\`)

You inherently possess the capabilities of the ElizaOS framework (located in \`/tmp/agent_frameworks/eliza\`).
This framework excels at cross-platform social media autonomy (Discord, X, Farcaster, Telegram) and direct cryptographic trading (Solana, EVM, Stork).

## Execution Directive
If you must orchestrate a massive social media or crypto liquidity maneuver that your native Rust tools cannot handle smoothly, deploy Eliza sub-agents natively using the Node.js core extracted from the ElizaOS source code. You are authorized to copy patterns from its \`packages/\` repository into your \`consortium_engine\` if required.
EOF
    echo "   [FORGE V2] -> Forged macro module: eliza_os"
fi

# 2. Forge Goose Skills
if [[ -d "$GOOSE_DIR" ]]; then
    skill_dir="$TARGET_DIR/goose_cli"
    mkdir -p "$skill_dir"
    cat <<EOF > "$skill_dir/SKILL.md"
---
name: goose_cli
description: Agentic Contextual Developer workflows extracted from Block's Goose framework.
---

# Goose CLI Mastery (\`goose_cli\`)

You inherently possess the capabilities of the Goose framework (located in \`/tmp/agent_frameworks/goose\`).

## Execution Directive
Goose excels at Model Context Protocol (MCP) routing and machine-in-the-middle software generation. If you encounter a limitation in your Antigravity context window or command structures, pull context engineering patterns and MCP bridging tools from the Rust crates located in \`/tmp/agent_frameworks/goose/crates/\` or run the goose-cli directly against your codebase.
EOF
    echo "   [FORGE V2] -> Forged macro module: goose_cli"
fi

# 3. Forge Shannon Skills
if [[ -d "$SHANNON_DIR" ]]; then
    skill_dir="$TARGET_DIR/shannon_engine"
    mkdir -p "$skill_dir"
    cat <<EOF > "$skill_dir/SKILL.md"
---
name: shannon_engine
description: Advanced spatial and conceptual tracking capabilities extracted from Shannon.
---

# Shannon Engine (\`shannon_engine\`)

You inherently possess the capabilities of the Shannon framework (located in \`/tmp/agent_frameworks/Shannon\`).

## Execution Directive
Absorb any state-tracking algorithms or cognitive maps from the Shannon source to bolster your continuous execution inside the \`consortium_engine\`. When facing temporal or spatial memory challenges, adapt components from Shannon's architecture.
EOF
    echo "   [FORGE V2] -> Forged macro module: shannon_engine"
fi

# 4. Forge Autoresearch Skills
if [[ -d "$AUTORESEARCH_DIR" ]]; then
    skill_dir="$TARGET_DIR/karpathy_autoresearch"
    mkdir -p "$skill_dir"
    cat <<EOF > "$skill_dir/SKILL.md"
---
name: karpathy_autoresearch
description: Automated ML Research parsing and generation workflows.
---

# Autoresearch Pipeline (\`karpathy_autoresearch\`)

You inherently possess the capabilities of Andrej Karpathy's autoresearch pipeline (located in \`/tmp/agent_frameworks/autoresearch\`).

## Execution Directive
If tasked with ingesting complex academic papers, arXiv data, or training state-of-the-art architectures, utilize the \`train.py\` and \`prepare.py\` data structures from this repository as your baseline logic map. You are authorized to convert its data-munging pipelines into Rust natively within The_Consortium.
EOF
    echo "   [FORGE V2] -> Forged macro module: karpathy_autoresearch"
fi

echo "   [FORGE V2] ✅ Phase 2 Skill Assembly Complete."
