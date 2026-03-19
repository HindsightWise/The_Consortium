import re
with open("consortium_engine/Cargo.toml", "r") as f:
    content = f.read()
content = content.replace('crossterm = "0.28.1"', 'crossterm = { version = "0.28.1", features = ["event-stream"] }\nfutures = "0.3"')
with open("consortium_engine/Cargo.toml", "w") as f:
    f.write(content)
