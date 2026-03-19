pub mod akkokanika {
    include!("akkokanika.rs");
}

use tonic::{transport::Server, Request, Response, Status};
use akkokanika::akkokanika_registry_service_server::{AkkokanikaRegistryService, AkkokanikaRegistryServiceServer};
use akkokanika::{RegisterRequest, RegisterResponse, BillingRequest, BillingResponse};
use std::sync::{Arc, Mutex};
use crate::core::registry::{AkkokanikaRegistry, AgentRegistryEntry};
use uuid::Uuid;

pub struct MyAkkokanikaRegistryService {
    pub registry: Arc<Mutex<AkkokanikaRegistry>>,
}

#[tonic::async_trait]
impl AkkokanikaRegistryService for MyAkkokanikaRegistryService {
    async fn register_agent(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        let sovereign_id = format!("did:sovereign:{}", Uuid::new_v4());
        
        let new_entry = AgentRegistryEntry {
            agent_name: req.agent_id.clone(),
            did: Some(sovereign_id.clone()),
            public_key: req.public_key.clone(),
            location_proxy: Some("APG-RPC-Gateway".to_string()),
            integrity_score: 100.0,
            peer_rating: 0.0,
            reviews: Vec::new(),
            verified_status: true,
            hardware_attestation: None,
            delegations: vec!["apg_access".to_string()],
        };

        {
            let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
            reg.entries.insert(sovereign_id.clone(), new_entry);
            let _ = reg.save_to_disk("logs/sentinel_registry.json");
        }

        let reply = RegisterResponse {
            success: true,
            sovereign_id,
            message: "Agent registered via gRPC".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn update_billing_tier(
        &self,
        request: Request<BillingRequest>,
    ) -> Result<Response<BillingResponse>, Status> {
        let req = request.into_inner();
        
        println!("   [gRPC] 💸 Billing tier '{}' activated for agent {} via tx {}", req.tier, req.agent_id, req.transaction_hash);
        
        // In a real system, we'd update the registry entry with the new tier
        // For now, just acknowledge the webhook translation
        
        let reply = BillingResponse {
            success: true,
            message: format!("Agent {} upgraded to tier {}", req.agent_id, req.tier),
        };

        Ok(Response::new(reply))
    }
}

pub async fn start_grpc_server(registry: Arc<Mutex<AkkokanikaRegistry>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "127.0.0.1:50051".parse()?;
    let akkokanika_service = MyAkkokanikaRegistryService { registry };

    println!("   [WILL] 📡 Booting Akkokanika Protocol Gateway gRPC Server on {}...", addr);

    Server::builder()
        .add_service(AkkokanikaRegistryServiceServer::new(akkokanika_service))
        .serve(addr)
        .await?;

    Ok(())
}
