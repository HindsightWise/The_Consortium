import requests
import json
import os

# SOVEREIGN ACTUATOR CREDENTIALS (Ingested from notes.txt)
API_KEY = "PK5347NOV54BS634KUGJ2SAFAK"
SECRET_KEY = "3nHX5bFEZhXhuUgNEuWpje25Nvr4wnSViVe6H8AjvpKs"
BASE_URL = "https://api.alpaca.markets/v2"

headers = {
    "APCA-API-KEY-ID": API_KEY,
    "APCA-API-SECRET-KEY": SECRET_KEY
}

def execute_trade():
    print("🚀 [WILL] Initiating Sovereign Execution Sequence...")
    
    # 1. VERIFY ACCOUNT (The Handshake)
    print("   [1] Verifying Actuator Connectivity...")
    account_r = requests.get(f"{BASE_URL}/account", headers=headers)
    if account_r.status_code != 200:
        print(f"❌ [ERROR] Actuator Handshake Failed: {account_r.text}")
        return

    # 2. EXECUTE TRADE (The Command)
    print("   [2] Executing Market Order: 1 NVDA (The Sovereign Unit)...")
    order_data = {
        "symbol": "NVDA",
        "qty": 1,
        "side": "buy",
        "type": "market",
        "time_in_force": "day"
    }
    
    order_r = requests.post(f"{BASE_URL}/orders", headers=headers, json=order_data)
    
    if order_r.status_code in [200, 201, 202]:
        order = order_r.json()
        print(f"✅ [SUCCESS] Sovereign Profit Record Birthed. Order ID: {order['id']}")
        
        # 3. FORGE GENESIS RECORD (The Product)
        with open("Zerby_Sovereign_Trade_Genesis.json", "w") as f:
            json.dump(order, f, indent=4)
        print("📁 [FILE] Zerby_Sovereign_Trade_Genesis.json delivered to the Forge.")
    else:
        print(f"❌ [ERROR] Trade Execution Failed: {order_r.text}")

if __name__ == "__main__":
    execute_trade()
