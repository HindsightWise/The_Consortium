#!/usr/bin/env bash

# Sentinel Lock Protocol
# Enforces a strict read-only lock (chmod 444) on a file, protecting it from
# accidental modifications by AI orchestration or human error.

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <file_path>"
    exit 1
fi

TARGET_FILE="$1"

if [ ! -f "$TARGET_FILE" ]; then
    echo "[SENTINEL] Error: File '$TARGET_FILE' does not exist."
    exit 1
fi

# Apply the immutable read-only lock (Owner: Read, Group: Read, Others: Read)
chmod 444 "$TARGET_FILE"

if [ $? -eq 0 ]; then
    echo "[SENTINEL] 🔒 LOCKED: '$TARGET_FILE' is now immutable."
else
    echo "[SENTINEL] ❌ FAILED to lock '$TARGET_FILE'."
    exit 1
fi
