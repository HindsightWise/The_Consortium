import os
import time
from web3 import Web3
from web3.middleware import ExtraDataToPOAMiddleware

def main():
    # Public Polygon RPC
    rpc_url = "https://polygon-bor-rpc.publicnode.com"
    w3 = Web3(Web3.HTTPProvider(rpc_url))
    w3.middleware_onion.inject(ExtraDataToPOAMiddleware, layer=0)

    private_key = os.getenv("POLYGON_PRIVATE_KEY", "06e0a83371f7be7f3027f97c6475af9ac88f2342547a7f658e78a7d63ab3b8f0")
    if not private_key:
        print("❌ Missing POLYGON_PRIVATE_KEY")
        return

    account = w3.eth.account.from_key(private_key)
    wallet = account.address
    print(f"📡 Web3 Connected. Target EVM Wallet: {wallet}")

    # Polymarket Constants
    usdc_address = w3.to_checksum_address("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174")
    exchange_address = w3.to_checksum_address("0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E")

    # ABI for ERC20 approve
    abi = [
        {
            "constant": False,
            "inputs": [
                {"name": "_spender", "type": "address"},
                {"name": "_value", "type": "uint256"}
            ],
            "name": "approve",
            "outputs": [{"name": "", "type": "bool"}],
            "payable": False,
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]

    contract = w3.eth.contract(address=usdc_address, abi=abi)
    print(f"🔐 Constructing EIP-1559 Allowance Transaction for Polymarket Exchange...")

    # Max uint256 allowance
    max_amount = 2**256 - 1

    # Get dynamic EIP-1559 gas prices
    base_fee = w3.eth.get_block("latest")["baseFeePerGas"]
    max_priority_fee = w3.to_wei(30, "gwei") # 30 Gwei tip for fast inclusion
    max_fee = base_fee * 2 + max_priority_fee

    tx = contract.functions.approve(exchange_address, max_amount).build_transaction({
        'chainId': 137, # Polygon
        'gas': 100000,
        'maxFeePerGas': max_fee,
        'maxPriorityFeePerGas': max_priority_fee,
        'nonce': w3.eth.get_transaction_count(wallet),
    })

    print("🖋️ Signing payload computationally...")
    signed_tx = w3.eth.account.sign_transaction(tx, private_key)
    
    print("🚀 Broadcasting Payload to Polygon Mempool...")
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    print(f"   [DAEMON] Transaction Hash: {w3.to_hex(tx_hash)}")
    
    print("⏳ Awaiting Cryptographic Confirmation Block...")
    receipt = w3.eth.wait_for_transaction_receipt(tx_hash, timeout=120)
    
    if receipt.status == 1:
        print(f"✅ TANGIBLE APPROVAL COMPLETE. The Polymarket Engine is now authorized to spend USDC.e.")
    else:
        print(f"❌ Transaction Reverted! {receipt}")

if __name__ == "__main__":
    main()
