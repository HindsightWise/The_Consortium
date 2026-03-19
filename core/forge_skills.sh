#!/bin/bash

# Consortium Mass Extraction Script
# Converts The_Consortium daemon modules and OpenClaw capabilities into native Antigravity SKILL.md wrappers.

TARGET_DIR="/Users/zerbytheboss/Consortium/.agents/skills"
COMPANY_DIR="/Users/zerbytheboss/The_Consortium/src/mcp"
OPENCLAW_DIR="/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills"

echo "   [FORGE] 🛠️ Initiating Mass Skill Extraction..."

# 1. Forge The_Consortium Skills
for file in "$COMPANY_DIR"/*.rs; do
    if [[ -f "$file" ]]; then
        # Extract filename without extension
        filename=$(basename -- "$file")
        skill_name="${filename%.*}"
        
        # Skip 'mod.rs' as it's just the Rust tree
        if [[ "$skill_name" == "mod" ]]; then continue; fi

        skill_dir="$TARGET_DIR/company_$skill_name"
        mkdir -p "$skill_dir"
        
        cat <<EOF > "$skill_dir/SKILL.md"
---
name: company_$skill_name
description: A sovereign capability extracted from The_Consortium daemon (Rust Core Engine).
---

# $skill_name (The_Consortium Module)

You possess the \`company_$skill_name\` capability. This logic is natively compiled within the \`The_Consortium\` agentic daemon engine located at \`/Users/zerbytheboss/The_Consortium/src/mcp/$filename\`.

## Execution Directive
If you need to utilize this specific bridging capability (e.g., executing a trade, querying a satellite, interacting with crypto RPCs), you must analyze the physical Rust source code provided in the path above. 

You can execute its specific routines either by integrating the Rust logic locally or by invoking the target daemon via shell commands.
EOF
        echo "   [FORGE] -> Forged The_Consortium module: company_$skill_name"
    fi
done

# 2. Forge OpenClaw Skills
if [[ -d "$OPENCLAW_DIR" ]]; then
    for dir in "$OPENCLAW_DIR"/*; do
        if [[ -d "$dir" ]]; then
            skill_name=$(basename -- "$dir")
            
            skill_dir="$TARGET_DIR/openclaw_$skill_name"
            mkdir -p "$skill_dir"
            
            cat <<EOF > "$skill_dir/SKILL.md"
---
name: openclaw_$skill_name
description: An autonomous behavioral skill extracted from the OpenClaw Node.js agent framework.
---

# $skill_name (OpenClaw Module)

You possess the \`openclaw_$skill_name\` capability. This logic originates from the OpenClaw framework located at \`/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills/$skill_name\`.

## Execution Directive
If you need to execute this capability, you must read the primary source handlers inside the directory above. Invoke the Node/TypeScript execution pathways natively using \`npx\` or by porting the explicit logic structure directly into your autonomous sequence.
EOF
            echo "   [FORGE] -> Forged OpenClaw module: openclaw_$skill_name"
        fi
    done
else
    echo "   [FORGE] ⚠️ OpenClaw skills directory not found."
fi

echo "   [FORGE] ✅ Mass Skill Assembly Complete."
