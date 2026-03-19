#!/usr/bin/env bash

# Sentinel Unlock Protocol
# Reverses the read-only lock (chmod 644) on a file, allowing it to be edited again.

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <file_path>"
    exit 1
fi

TARGET_FILE="$1"

if [ ! -f "$TARGET_FILE" ]; then
    echo "[SENTINEL] Error: File '$TARGET_FILE' does not exist."
    exit 1
fi

# Restore standard read/write permissions (Owner: Read/Write, Group: Read, Others: Read)
chmod 644 "$TARGET_FILE"

if [ $? -eq 0 ]; then
    echo "[SENTINEL] 🔓 UNLOCKED: '$TARGET_FILE' is now ready for modification."
else
    echo "[SENTINEL] ❌ FAILED to unlock '$TARGET_FILE'."
    exit 1
fi
