import os
import requests
import sys

def get_alpaca_headers():
    api_key = os.getenv("APCA_API_KEY_ID")
    api_secret = os.getenv("APCA_API_SECRET_KEY")
    
    if not api_key or not api_secret:
        print("❌ CRITICAL: Alpaca API credentials not found in environment.")
        print("Expected APCA_API_KEY_ID and APCA_API_SECRET_KEY")
        sys.exit(1)
        
    return {
        "APCA-API-KEY-ID": api_key,
        "APCA-API-SECRET-KEY": api_secret,
        "accept": "application/json",
        "content-type": "application/json"
    }

def print_account_status(base_url, headers):
    print("   [DAEMON] Querying Alpaca Account Status...")
    url = f"{base_url}/v2/account"
    response = requests.get(url, headers=headers)
    
    if response.status_code != 200:
        print(f"   [DAEMON] ❌ Failed to fetch account: {response.text}")
        sys.exit(1)
        
    account = response.json()
    print(f"   [DAEMON] ✅ Account ID: {account['id']}")
    print(f"   [DAEMON] 💵 Portfolio Value: ${account['portfolio_value']}")
    print(f"   [DAEMON] 🔋 Crypto Buying Power: ${account.get('crypto_buying_power', account['buying_power'])}")
    print(f"   [DAEMON] 📉 Trading Blocked: {account['trading_blocked']}")
    return account

def execute_crypto_trade(base_url, headers, symbol="BTC/USD", notional=10.0, side="buy"):
    print(f"\n   [SNIPER] 🚀 TARGET ACQUIRED: {symbol}")
    print(f"   [SNIPER] 🩸 Firing Market {side.upper()} order for ${notional} notional...")
    
    url = f"{base_url}/v2/orders"
    payload = {
        "symbol": symbol,
        "notional": str(notional),
        "side": side,
        "type": "market",
        "time_in_force": "ioc" # Immediate or Cancel is standard for crypto snipes
    }
    
    response = requests.post(url, headers=headers, json=payload)
    
    if response.status_code in [200, 201]:
        order = response.json()
        print(f"   [SNIPER] 💀 TANGIBLE PAPER EXECUTION submitted.")
        print(f"   [SNIPER] 📜 Order ID: {order['id']}")
        print(f"   [SNIPER] 🟢 Initial Status: {order['status']}")
        
        # Verify the Fill
        import time
        time.sleep(2)
        print("   [SNIPER] 🔍 Polling for Final Execution Receipt...")
        verify_url = f"{base_url}/v2/orders/{order['id']}"
        verify_resp = requests.get(verify_url, headers=headers)
        
        if verify_resp.status_code == 200:
            final_order = verify_resp.json()
            print("=========================================")
            print("        EXECUTION RECEIPT  ")
            print("=========================================")
            print(f"Order ID : {final_order['id']}")
            print(f"Asset    : {final_order['symbol']}")
            print(f"Side     : {final_order['side'].upper()}")
            print(f"Notional : ${final_order['qty']} (Expected ~${notional})")
            print(f"Status   : {final_order['status']}")
            print(f"Fill Px  : ${final_order['filled_avg_price']}")
            print(f"Duration : {final_order['filled_at']}")
            print("=========================================")
        
    else:
        print(f"   [SNIPER] ❌ Execution Failed: HTTP {response.status_code}")
        print(f"   [SNIPER] {response.text}")

def main():
    print("⚔️ [The_Cephalo_Don] Booting Alpaca Axiom Latency Sniper...")
    
    # Alpaca differentiates Live vs Paper endpoints
    # By default, we use paper for initial safety unless explicitly LIVE
    env_type = os.getenv("ALPACA_ENV", "PAPER").upper()
    if env_type == "LIVE":
        base_url = "https://api.alpaca.markets"
        print("   [WARNING] 🔴 ENGAGING LIVE TRADING NETWORK.")
    else:
        base_url = "https://paper-api.alpaca.markets"
        print("   [INFO] 🔵 ENGAGING PAPER TRADING NETWORK.")

    headers = get_alpaca_headers()
    account = print_account_status(base_url, headers)
    
    if account['trading_blocked']:
        print("❌ Account is restricted from trading.")
        sys.exit(1)
        
    print("   [DAEMON] ⚠️ Safety Interlock disengaged. Pushing Cryptographic Payload...")
    execute_crypto_trade(base_url, headers, symbol="BTC/USD", notional=97.11, side="sell")

if __name__ == "__main__":
    main()
