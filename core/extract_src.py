import os
import shutil

src_dir = "/Users/zerbytheboss/Consortium"
dest_dir = "/tmp/Consortium_api"

# Remove dest if exists
if os.path.exists(dest_dir):
    shutil.rmtree(dest_dir)

os.makedirs(dest_dir, exist_ok=True)

allowed_exts = {".rs", ".toml", ".c", ".h", ".cpp", ".md", ".m", ".mm"}

for root, dirs, files in os.walk(src_dir):
    # Skip target and virtualenvs completely
    dirs[:] = [d for d in dirs if d not in ["target", "node_modules", ".git", ".venv_thrml", ".venv_obl", ".next"]]
    
    for file in files:
        if file.startswith("."):
            continue
        
        _, ext = os.path.splitext(file)
        if ext in allowed_exts:
            # Construct paths
            rel_path = os.path.relpath(root, src_dir)
            target_path = os.path.normpath(os.path.join(dest_dir, rel_path))
            
            src_file = os.path.join(root, file)
            dest_file = os.path.join(target_path, file)
            
            try:
                os.makedirs(target_path, exist_ok=True)
                shutil.copy2(src_file, dest_file)
            except Exception as e:
                print(f"Skipping {src_file}: {e}")

print("✅ Physical Extraction Complete.")
