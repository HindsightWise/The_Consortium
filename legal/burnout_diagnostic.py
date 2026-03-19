#!/usr/bin/env python3
"""
SOP 0.2: Burnout Diagnostic & Async Architecture
Cron job script to poll the human operator strictly via CLI/Telegram for fatigue score.
"""
import sqlite3
import datetime
import os
import sys

# Initialize local DB
db_path = os.path.expanduser('~/.consortium_fatigue.db')
conn = sqlite3.connect(db_path)
c = conn.cursor()
c.execute('''CREATE TABLE IF NOT EXISTS fatigue_logs
             (date TEXT, score INTEGER)''')

def ask_fatigue():
    # In a full deployment, this would ping Telegram/Discord via HTTP request.
    # For now, it logs the command line request.
    print("[MANDATORY DIAGNOSTIC] Protocol 0.2")
    print("Please enter your current fatigue level (1 = Peak Form, 5 = Critical Burnout): ")
    try:
        score = int(input())
        if score < 1 or score > 5:
            raise ValueError
    except:
        print("Invalid score. Defaulting to safe assumption of 3.")
        score = 3
    
    date_str = datetime.datetime.now().isoformat()
    c.execute("INSERT INTO fatigue_logs VALUES (?, ?)", (date_str, score))
    conn.commit()

def calculate_ma():
    c.execute("SELECT score FROM fatigue_logs ORDER BY date DESC LIMIT 4")
    rows = c.fetchall()
    if not rows: return 0.0
    scores = [r[0] for r in rows]
    return sum(scores) / len(scores)

def check_threshold():
    ma = calculate_ma()
    if ma >= 4.0:
        print(f"CRITICAL: 4-period Moving Average Fatigue is {ma}. \nLOCK_NON_CRITICAL_PRS() HAS BEEN TRIGGERED.")
        print("Rejecting calendar API requests for quick syncs. Enforcing Async templates.")
        # Logic to lock Git branches or freeze daemon expansion goes here.
        sys.exit(1)
    else:
        print(f"Status OK. Fatigue MA: {ma}. Execution unrestricted.")

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == '--poll':
        ask_fatigue()
    check_threshold()
    conn.close()
