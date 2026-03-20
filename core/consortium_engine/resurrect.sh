#!/bin/bash

# ==========================================
# THE CONSORTIUM: LAZARUS BASH LOOP
# ==========================================
# Runs the engine indefinitely. If it encounters a panic, 
# it writes the crash log natively via the `std::panic` hook, 
# then resurrects 3 seconds later into SAFEMODE.

echo "🐙 Ozymandias Lazarus Protocol Armed."

while true; do
    ./target/debug/consortium_engine
    EXIT_CODE=$?
    
    # Check if the process cleanly exited (Ctrl+C from user)
    if [ $EXIT_CODE -eq 0 ]; then
        echo "🛑 Clean Operator Shutdown. Halting resurrection loop."
        break
    fi

    # Check if the process returned 130 (Standard bash SIGINT)
    if [ $EXIT_CODE -eq 130 ]; then
        echo "🛑 Keyboard Interrupt (Ctrl+C). Halting resurrection loop."
        break
    fi

    echo "⚡ [EXTREMIS] Consortium Engine Flatlined (Exit Code $EXIT_CODE)..."
    echo "⚡ [EXTREMIS] Autonomic Nervous System initiating defibrillator reboot in 3 seconds..."
    sleep 3
    echo "⚡ [EXTREMIS] Resurrecting Engine state... Passing crash forensic logs directly into LLM stream."
done
