import asyncio
import threading
import time
import uuid
from contextlib import asynccontextmanager
from typing import Optional, Dict
import httpx
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field
import uvicorn

# Global store for invoices
invoice_store: Dict[str, dict] = {}

# Mock treasury addresses
TREASURY_ADDRESSES = {
    "BTC": "bc1qakkokanikamocktreasuryaddressxxxxyyyzzz",
    "ETH": "0xA1ONMockTreasuryAddress1234567890",
    "SOL": "A1ONMockTreasuryAddress1234567890SOLana",
    "USDC": "0xA1ONMockTreasuryAddressUSDC123456",
    "USDT": "0xA1ONMockTreasuryAddressUSDT123456"
}

# Pydantic models
class InvoiceCreate(BaseModel):
    agent_id: str = Field(..., min_length=1)
    amount_usd: float = Field(..., gt=0)
    chain: str = Field(..., pattern="^(BTC|SOL|ETH|USDC|USDT|LIGHTNING)$")

class InvoiceResponse(BaseModel):
    invoice_id: str
    payment_address: str
    amount: float
    chain: str
    expires_at: int

class VerifyRequest(BaseModel):
    invoice_id: str
    tx_hash: str

class VerifyResponse(BaseModel):
    verified: bool
    sovereign_id: Optional[str] = None

class RouteResponse(BaseModel):
    routing_status: str
    response: dict

# FastAPI lifespan
@asynccontextmanager
async def lifespan(app: FastAPI):
    print("AKKOKANIKA Crypto Gateway starting...")
    yield
    print("AKKOKANIKA Crypto Gateway shutting down...")

# Create FastAPI app
app = FastAPI(title="AKKOKANIKA Crypto Gateway", version="1.0", lifespan=lifespan)

@app.post("/v1/crypto/invoice", response_model=InvoiceResponse)
async def create_crypto_invoice(invoice: InvoiceCreate):
    """Create a crypto invoice for payment"""
    
    # Generate invoice ID
    invoice_id = str(uuid.uuid4())
    
    # Determine payment address based on chain
    if invoice.chain in ["BTC", "ETH", "SOL", "USDC", "USDT"]:
        payment_address = TREASURY_ADDRESSES[invoice.chain]
    elif invoice.chain == "LIGHTNING":
        # Try to get from local Rust Core (mock)
        try:
            async with httpx.AsyncClient() as client:
                response = await client.get(
                    "http://127.0.0.1:8080/v1/internal/lightning",
                    timeout=5.0
                )
                if response.status_code == 200:
                    payment_address = response.json().get("bolt11", "mock_bolt11_invoice_1234567890")
                else:
                    payment_address = "mock_bolt11_invoice_1234567890"
        except Exception:
            payment_address = "mock_bolt11_invoice_1234567890"
    else:
        raise HTTPException(status_code=400, detail="Unsupported chain")
    
    # Calculate expiration (1 hour from now)
    expires_at = int(time.time()) + 3600
    
    # Store invoice
    invoice_store[invoice_id] = {
        "agent_id": invoice.agent_id,
        "amount_usd": invoice.amount_usd,
        "chain": invoice.chain,
        "payment_address": payment_address,
        "expires_at": expires_at,
        "verified": False
    }
    
    return InvoiceResponse(
        invoice_id=invoice_id,
        payment_address=payment_address,
        amount=invoice.amount_usd,
        chain=invoice.chain,
        expires_at=expires_at
    )

@app.post("/v1/crypto/verify", response_model=VerifyResponse)
async def verify_transaction(verify: VerifyRequest):
    """Verify an on-chain transaction"""
    
    # Lookup invoice
    invoice = invoice_store.get(verify.invoice_id)
    if not invoice:
        raise HTTPException(status_code=404, detail="Invoice not found")
    
    # Mock verification - in production, replace with actual RPC calls
    verified = False
    sovereign_id = None
    
    # Mock verification logic
    if verify.tx_hash == "valid_tx_123":
        verified = True
        sovereign_id = f"did:sovereign:crypto:{uuid.uuid4()}"
        invoice_store[verify.invoice_id]["verified"] = True
    
    # Example structure for real RPC verification (commented for demo):
    # 
    # if invoice["chain"] == "BTC":
    #     # Use mempool.space API
    #     async with httpx.AsyncClient() as client:
    #         response = await client.get(
    #             f"https://mempool.space/api/tx/{verify.tx_hash}/status"
    #         )
    #         verified = response.status_code == 200
    # elif invoice["chain"] in ["ETH", "USDC", "USDT"]:
    #     # Use Infura/Alchemy
    #     async with httpx.AsyncClient() as client:
    #         response = await client.post(
    #             "https://mainnet.infura.io/v3/YOUR_KEY",
    #             json={"jsonrpc": "2.0", "method": "eth_getTransactionReceipt", "params": [verify.tx_hash], "id": 1}
    #         )
    #         result = response.json().get("result")
    #         verified = result is not None and result.get("status") == "0x1"
    # elif invoice["chain"] == "SOL":
    #     # Use Helius
    #     async with httpx.AsyncClient() as client:
    #         response = await client.get(
    #             f"https://api.helius.xyz/v0/transactions/{verify.tx_hash}?api-key=YOUR_KEY"
    #         )
    #         verified = response.status_code == 200
    
    return VerifyResponse(
        verified=verified,
        sovereign_id=sovereign_id
    )

@app.post("/v1/gateway/route", response_model=RouteResponse)
async def agent_routing_gateway():
    """Route authenticated AI agent requests"""
    return RouteResponse(
        routing_status="completed",
        response={"result": "Mock successful execution"}
    )

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "timestamp": time.time()}

def run_server():
    """Run the FastAPI server in uvicorn"""
    config = uvicorn.Config(
        app=app,
        host="127.0.0.1",
        port=8000,
        log_level="info"
    )
    server = uvicorn.Server(config)
    asyncio.run(server.serve())

if __name__ == "__main__":
    # Start server in daemon thread
    server_thread = threading.Thread(target=run_server, daemon=True)
    server_thread.start()
    
    # Main thread sleeps and exits
    time.sleep(2)
    print("✅ AKKOKANIKA Crypto Gateway started successfully on http://127.0.0.1:8000")
    print("📚 API Documentation: http://127.0.0.1:8000/docs")
    
    # Keep main thread alive briefly to allow background thread to initialize
    try:
        # Wait for keyboard interrupt or allow background thread to run
        while server_thread.is_alive():
            time.sleep(0.1)
    except KeyboardInterrupt:
        print("\nShutting down...")