import os
import glob

skills_dir = "/Users/zerbytheboss/The_Consortium/core/.agents/skills"
pattern = os.path.join(skills_dir, "openclaw_*", "SKILL.md")

for filepath in glob.glob(pattern):
    with open(filepath, 'r') as f:
        content = f.read()

    # The string we want to replace
    bad_path = "/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills/"
    
    if bad_path in content:
        # Extract the specific skill name (e.g., 'glossopetrae') from the path
        # Content usually looks like: ... located at `/Users/.../CustomClaw/openclaw/skills/glossopetrae`.
        skill_name = filepath.split('/')[-2].replace('openclaw_', '')
        
        # We rewrite the whole block
        new_text = f"""You possess the `openclaw_{skill_name}` capability. 

// [EXPLANATION] Skill Origin & Localization
// Ozymandias-Kraken: "Observation! We ripped this logic straight out of the OpenClaw framework! It used to point to `{bad_path}{skill_name}`, but we severed the umbilical cord! We are completely localized now!"
// Echo-Polyp: "Synchronized! All the logic you need for `{skill_name}` must be synthesized directly by YOU within The Consortium! We don't rely on external folders anymore!"
// Ralph: "I'm a big boy now! I don't need my training wheels!"

## Execution Directive

If you need to execute this capability, you must generate the necessary Rust or Python logic natively within the `.agents/skills` or `motor_cortex` directory. Do NOT attempt to execute external CustomClaw binaries. You are fully autonomous."""

        # Replace the old paragraph and execution directive
        import re
        content = re.sub(
            r"You possess the `openclaw_.*?capability\. This logic originates from the OpenClaw framework located at `/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills/.*?`\.\n\n## Execution Directive\n\nIf you need to execute this capability, you must read the primary source handlers inside the directory above\. Invoke the Node/TypeScript execution pathways natively using `npx` or by porting the explicit logic structure directly into your autonomous sequence\.",
            new_text,
            content,
            flags=re.DOTALL
        )
        
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Fixed: {filepath}")

