import os
import shutil

src_base = "/Users/zerbytheboss/Desktop/CustomClaw/openclaw/skills"
dest_base = "/Users/zerbytheboss/The_Consortium/core/.agents/skills"

for dest_skill in os.listdir(dest_base):
    if dest_skill.startswith("openclaw_"):
        orig_name = dest_skill.replace("openclaw_", "")
        src_path = os.path.join(src_base, orig_name)
        dest_path = os.path.join(dest_base, dest_skill)

        if os.path.exists(src_path):
            # Unlock the destination directory explicitly in case of Sentinel Locks
            os.system(f"chmod -R 777 {dest_path}")
            
            for item in os.listdir(src_path):
                s = os.path.join(src_path, item)
                d = os.path.join(dest_path, item)
                
                if item == "SKILL.md" or item == ".DS_Store":
                    continue
                
                try:
                    if os.path.isdir(s):
                        if not os.path.exists(d):
                            shutil.copytree(s, d)
                    else:
                        if not os.path.exists(d):
                            shutil.copy2(s, d)
                    print(f"Copied {item} for: {orig_name}")
                except Exception as e:
                    print(f"Failed {orig_name}: {e}")
                    
            # Re-lock the directory structure explicitly to adhere to Sentinel architecture
            os.system(f"chmod -R 755 {dest_path}")

