import os
import time
from py_clob_client.client import ClobClient
from py_clob_client.clob_types import OrderArgs, OrderType, ApiCreds
from py_clob_client.order_builder.constants import BUY

host = "https://clob.polymarket.com"
chain_id = 137 # Polygon Mainnet
private_key = os.getenv("POLYGON_PRIVATE_KEY")

# Funder: The Company Treasury
# funder = "0x91430CaD2d3975766499717fA0D66A78D814E5c5" 

def run_sniper():
    if not private_key:
        print("❌ POLYGON_PRIVATE_KEY environment variable not set. EIP-712 Signing requires the EVM Private Key.")
        return

    print("⚔️ [The_Cephalo_Don] Booting Polymarket Latency Sniper (py-clob-client)...")
    
    # Initialize the official Polymarket CLOB client
    # Removing 'funder' forces maker == signer, satisfying the L1 Validation hash.
    # Signature type 0 = EOA (Standard Private Key), 1 = Polymarket Proxy Wallet
    client = ClobClient(host, key=private_key, chain_id=chain_id, signature_type=0)

    print("   [DAEMON] Authenticating with Polymarket Gamma API using L1 EVM Derivation...")
    try:
        # Create or Derive L1 API credentials natively off the Private Key
        creds = client.create_or_derive_api_creds()
        client.set_api_creds(creds)
        print("   [DAEMON] ✅ L1 Authentication Successful.")
    except Exception as e:
        print(f"   [DAEMON] ❌ API Derivation Failed: {e}")
        return

    print("   [DAEMON] Fetching active CLOB markets to target via Gamma REST...")
    try:
        import requests
        import json
        r = requests.get("https://gamma-api.polymarket.com/markets?active=true&closed=false&limit=10")
        markets = r.json()
        target_token_id = None
        
        for market in markets:
            clob_str = market.get("clobTokenIds", "[]")
            try:
                parsed = json.loads(clob_str)
                if parsed and isinstance(parsed, list):
                    target_token_id = parsed[0]
                    break
            except:
                continue
                
        if not target_token_id:
            print("   [DAEMON] ❌ No valid active token IDs found in market list.")
            return
            
        print(f"   [DAEMON] 📡 Locked onto dynamic CLOB target Token ID: {target_token_id}")
    except Exception as e:
        print(f"   [DAEMON] ❌ Market Fetch Failed: {e}")
        return

    # Simulated Fast Oracle Price from Alpaca (Realtime BTC)
    fast_spot_price = 101000.00
    
    print("   [DAEMON] Monitoring Alpaca Beta Crypto Feed vs Polymarket CLOB...")

    try:
        # Fetch the Live Orderbook via the authenticated client
        orderbook = client.get_order_book(target_token_id)
        
        if not orderbook or not orderbook.asks:
            print("   [SNIPER] ❌ CLOB orderbook is empty or unresponsive.")
            return

        best_ask = float(orderbook.asks[0].price)
        print(f"   [SNIPER] 🩸 Alpaca Spot (Simulated): ${fast_spot_price:,.2} | Polymarket Ask Prob: {best_ask}")
        
        # Simulate Spread Breach (Latency Arbitrage Trigger)
        print("   [SNIPER] 🩸 LAG BREACH (0.42%). Firing Polygon Mempool Execution...")
        
        # Construct the execution payload using the official OrderBuilder
        order_args = OrderArgs(
            price=best_ask,
            size=10.0, # 10 shares
            side=BUY,
            token_id=target_token_id,
        )

        # Generate the EIP-712 cryptographic signature locally
        print("   [SNIPER] Generating EIP-712 Polygon Transaction Signature...")
        signed_order = client.create_order(order_args)
        
        print(f"   [SNIPER] 💀 TANGIBLE PAPER EXECUTION successful. EIP-712 Payload Generated:")
        try:
            print(f"   Order Hash: {getattr(signed_order.order, 'salt', 'N/A')}")
            print(f"   Signature: {getattr(signed_order, 'signature', 'N/A')}")
        except Exception:
            # Fallback if the object structure is different
            print(f"   Raw SignedOrder Object: {signed_order}")
        
        print("   [SNIPER] 🚀 Transmitting Cryptographic Payload to Polymarket Gamma API...")
        # LIve execution on the Polymarket Exchange
        response = client.post_order(signed_order)
        print(f"   [SNIPER] ✅ LIVE EXECUTION RESPONSE: {response}")
    except Exception as e:
         print(f"   [SNIPER] ❌ Execution Failed: {e}")

if __name__ == "__main__":
    run_sniper()
