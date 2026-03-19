import os
import shutil

src_base = "/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills"
dest_base = "/Users/zerbytheboss/The_Consortium/core/.agents/skills"

# Iterate over all skills in the destination (Consortium)
for dest_skill in os.listdir(dest_base):
    if dest_skill.startswith("openclaw_"):
        # The original skill name without the 'openclaw_' prefix
        orig_name = dest_skill.replace("openclaw_", "")
        src_path = os.path.join(src_base, orig_name)
        dest_path = os.path.join(dest_base, dest_skill)

        if os.path.exists(src_path):
            # We want to copy the contents of src_path into dest_path
            for item in os.listdir(src_path):
                s = os.path.join(src_path, item)
                d = os.path.join(dest_path, item)
                
                # Prevent overwriting SKILL.md
                if item == "SKILL.md":
                    continue
                
                if os.path.isdir(s):
                    if not os.path.exists(d):
                        shutil.copytree(s, d)
                else:
                    if not os.path.exists(d):
                        shutil.copy2(s, d)
            print(f"Copied source for: {orig_name}")

