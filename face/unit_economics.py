#!/usr/bin/env python3
"""
SOP 1.2: Unit-Economics & Sean Ellis PMF Test
Tracking Script
"""
import sys
import json
import os

DB_FILE = os.path.expanduser('~/.consortium_unit_economics.json')

def load_db():
    if os.path.exists(DB_FILE):
        with open(DB_FILE, 'r') as f:
            return json.load(f)
    return {"channels": {}, "pmf_survey": {"very_disappointed": 0, "somewhat_disappointed": 0, "not_disappointed": 0}}

def save_db(data):
    with open(DB_FILE, 'w') as f:
        json.dump(data, f, indent=4)

def record_cac_ltv(channel, cac, ltv):
    data = load_db()
    if channel not in data["channels"]:
        data["channels"][channel] = {"cac": 0.0, "ltv": 0.0, "customers": 0}
    
    # Cumulative moving average
    current = data["channels"][channel]
    n = current["customers"]
    
    current["cac"] = (current["cac"] * n + float(cac)) / (n + 1)
    current["ltv"] = (current["ltv"] * n + float(ltv)) / (n + 1)
    current["customers"] += 1
    
    save_db(data)
    print(f"Recorded metrics for {channel}.")
    evaluate_channels(data)

def evaluate_channels(data):
    print("--- CHANNEL VIABILITY REPORT ---")
    for channel, metrics in data["channels"].items():
        if metrics["cac"] == 0:
            ratio = float('inf')
        else:
            ratio = metrics["ltv"] / metrics["cac"]
        
        print(f"Channel: {channel} | LTV: ${metrics['ltv']:.2f} | CAC: ${metrics['cac']:.2f} | Ratio: {ratio:.2f}")
        if ratio < 3.0:
            print(f"FLAG_CHANNEL_UNVIABLE => {channel} is burning capital. Terminate ad spend.")

def record_pmf(response):
    valid_responses = ["very_disappointed", "somewhat_disappointed", "not_disappointed"]
    if response not in valid_responses:
        print("Invalid response.")
        return
    data = load_db()
    data["pmf_survey"][response] += 1
    save_db(data)
    evaluate_pmf(data)

def evaluate_pmf(data):
    survey = data["pmf_survey"]
    total = sum(survey.values())
    if total == 0:
        return
    
    pmf_score = survey["very_disappointed"] / total
    print(f"Current Sean Ellis PMF Score: {pmf_score*100:.2f}% (Target: >= 40%)")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: ./unit_economics.py [record_metrics <channel> <cac> <ltv> | pmf <very_disappointed|somewhat_disappointed|not_disappointed> | status]")
        sys.exit(1)
        
    cmd = sys.argv[1]
    match cmd:
        case "record_metrics":
            record_cac_ltv(sys.argv[2], sys.argv[3], sys.argv[4])
        case "pmf":
            record_pmf(sys.argv[2])
        case "status":
            data = load_db()
            evaluate_channels(data)
            evaluate_pmf(data)
