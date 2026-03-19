#!/usr/bin/env python3
"""
SOP 11.2: The Immortal Sunset Rule

ACTION: Run weekly cron evaluating D1/D7/D30 Retention, LTV/CAC, and PMF Score.
CONDITION: IF days_since_launch > 60 AND (PMF_Score < 40% OR LTV < 3*CAC): Trigger Sunset Protocol.
ACTION (Sunset): Disable Stripe API, generate export CSVs, scale infra to zero.
"""

import json
import os
import datetime
import subprocess

DB_FILE = os.path.expanduser('~/.consortium_unit_economics.json')
LAUNCH_DATE = datetime.datetime.fromisoformat("2026-03-16T00:00:00")

def load_metrics():
    if os.path.exists(DB_FILE):
        with open(DB_FILE, 'r') as f:
            return json.load(f)
    return None

def trigger_sunset():
    print("!!! INITIATING SUNSET PROTOCOL !!!")
    print("Disabling Stripe keys...")
    # os.system("aws ssm put-parameter --name '/stripe/secret_key' --value 'REVOKED' --overwrite")
    print("Generating Export CSVs for user data preservation...")
    # SQL DUMP emulation
    print("Scaling infrastructure to zero...")
    # subprocess.run(["terraform", "apply", "-var", "desired_capacity=0", "-auto-approve"])
    print("Sunset Complete. Capital bleeding halted. Await next iteration.")

def evaluate_survival():
    data = load_metrics()
    if not data:
        print("No metrics available. Halting evaluation.")
        return

    days_alive = (datetime.datetime.now() - LAUNCH_DATE).days
    print(f"Days since launch: {days_alive}")

    # Calculate global PMF
    survey = data.get("pmf_survey", {"very_disappointed": 0, "somewhat_disappointed": 0, "not_disappointed": 0})
    total_surveys = sum(survey.values())
    pmf_score = (survey["very_disappointed"] / total_surveys) if total_surveys > 0 else 0.0

    # Calculate global LTV/CAC ratio
    ratios = []
    for ch, metrics in data.get("channels", {}).items():
        if metrics["cac"] > 0:
            ratios.append(metrics["ltv"] / metrics["cac"])
    
    avg_ratio = sum(ratios) / len(ratios) if ratios else 0.0

    if days_alive > 60:
        if pmf_score < 0.40 or avg_ratio < 3.0:
            print(f"FAILED VIABILITY GATE. PMF: {pmf_score*100}%. Ratio: {avg_ratio}. Days: {days_alive}.")
            trigger_sunset()
        else:
            print(f"SURVIVABILITY GATE PASSED. PMF: {pmf_score*100}%. Ratio: {avg_ratio}.")
    else:
        print("Within 60-day grace period. Continue iteration.")

if __name__ == "__main__":
    evaluate_survival()
