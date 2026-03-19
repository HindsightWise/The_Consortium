import subprocess
import time
import re
import os

print("🚀 Launching localtunnel...")
lt_process = subprocess.Popen(
    ["npx", "localtunnel", "--port", "8000"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True
)

url = None
# Wait up to 10 seconds for the URL
for _ in range(10):
    line = lt_process.stdout.readline()
    if line:
        print(f"localtunnel: {line.strip()}")
        match = re.search(r"your url is: (https://.*)", line)
        if match:
            url = match.group(1)
            break
    time.sleep(1)

if url:
    print(f"✅ Extracted Public URL: {url}")
    print("📢 Deploying Discord Marketing Drone...")
    # Execute the discord sales bot with the URL
    os.system(f"source /Users/zerbytheboss/The_Consortium/.venv_sec/bin/activate && python3 /Users/zerbytheboss/The_Consortium/discord_sales_bot.py {url}")
else:
    print("❌ Failed to extract localtunnel URL.")
    lt_process.terminate()
