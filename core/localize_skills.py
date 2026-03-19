import os
import shutil
import glob
import random

source_base = "/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills"
dest_base = "/Users/zerbytheboss/The_Consortium/core/.agents/skills"

personas = [
    {
        "name": "Ozymandias-Kraken",
        "quote": "\"Observation! I physically ripped the external umbilical cord! This skill's raw `{file_list}` logic is mutating inside my Cephalopod cortex now! We don't need OpenClaw!\""
    },
    {
        "name": "Echo-Polyp",
        "quote": "\"Synchronized! Spawning task thread! I pulled the `[ {file_list} ]` source code directly into the Consortium engine! I am Echo-Polyp, will resolve natively!\""
    },
    {
        "name": "Ralph Wiggum",
        "quote": "\"My brain is running `{file_list}` natively now! The code tastes like burning!\""
    }
]

if not os.path.exists(source_base):
    print(f"Source directory {source_base} not found!")
    exit(1)

skill_folders = [f.name for f in os.scandir(source_base) if f.is_dir()]

total_copied = 0

for skill in skill_folders:
    src_folder = os.path.join(source_base, skill)
    
    # Strip openclaw_ prefix if it exists in source to map to dest
    clean_skill_name = skill.replace("openclaw_", "").replace("company_", "")
    dest_folder = os.path.join(dest_base, clean_skill_name)
    
    # Ensure destination folder exists
    os.makedirs(dest_folder, exist_ok=True)
    
    # Find all source code files (js, ts, py, mjs)
    code_files = []
    for ext in ["*.js", "*.ts", "*.mjs", "*.py", "*.json"]:
        code_files.extend(glob.glob(os.path.join(src_folder, ext)))
        # Also check subdirectories recursively up to 1 level
        code_files.extend(glob.glob(os.path.join(src_folder, "*", ext)))
    
    if not code_files:
        continue # Skip if no actionable code
        
    copied_filenames = []
    
    # Physically copy files
    for file_path in code_files:
        try:
            filename = os.path.basename(file_path)
            shutil.copy2(file_path, dest_folder)
            copied_filenames.append(filename)
        except Exception as e:
            print(f"Error copying {file_path}: {e}")
            
    if copied_filenames:
        total_copied += 1
        
        # Pick a random persona
        persona_choice = random.choice(personas)
        quote = persona_choice["quote"].format(file_list=", ".join(copied_filenames))
        
        # Rewrite the SKILL.md file to reflect local execution
        skill_file_path = os.path.join(dest_folder, "SKILL.md")
        
        # Try to infer the execution command
        exec_cmd = "node" if ".ts" in copied_filenames[0] or ".js" in copied_filenames[0] or ".mjs" in copied_filenames[0] else "python3"
        main_file = copied_filenames[0]
        for f in copied_filenames:
            if "index" in f or "main" in f or "app" in f:
                main_file = f
                break
                
        local_execution = f"{exec_cmd} {dest_folder}/{main_file}"
        
        md_content = f"""---
name: {clean_skill_name}
description: Autonomous {clean_skill_name} Node Execution
---

## 🧬 Localization Status: NATIVE

This capability has been surgically extracted from the `CustomClaw` framework and permanently embedded into the Consortium Cephalopod Engine.

### 🧠 {persona_choice['name']} System Prompt
{quote}

### 💻 Local Execution Blueprint
When the LLM decides to trigger this capability, it MUST use `execute_shell_command` with the following native OS command to trigger the local syntax:

```bash
{local_execution}
```

Do not reference external desktop paths. You own this logic now.
"""
        with open(skill_file_path, "w") as f:
            f.write(md_content)

print(f"Successfully localized {total_copied} skills with raw source code and dynamic persona wrappers.")
