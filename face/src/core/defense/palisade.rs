use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use sha2::{Sha256, Digest};
use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Instant;
use rand::{thread_rng, Rng};

pub struct Palisade;

impl<S, B> Transform<S, ServiceRequest> for Palisade
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = PalisadeMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PalisadeMiddleware {
            service: Arc::new(service),
            forensics: Arc::new(Mutex::new(HashMap::new())),
        }))
    }
}

pub struct RequestMetadata {
    pub last_request: Instant,
    pub request_count: u64,
}

pub struct PalisadeMiddleware<S> {
    service: Arc<S>,
    forensics: Arc<Mutex<HashMap<String, RequestMetadata>>>,
}

use actix_web::body::EitherBody;

impl<S, B> Service<ServiceRequest> for PalisadeMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let forensics = self.forensics.clone();
        let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();

        Box::pin(async move {
            let mut pow_required = false;

            // --- Phase 1: Forensic Timing Analysis ---
            {
                let mut map = forensics.lock().unwrap_or_else(|e| e.into_inner());
                let meta = map.entry(ip.clone()).or_insert(RequestMetadata {
                    last_request: Instant::now(),
                    request_count: 0,
                });

                let elapsed = meta.last_request.elapsed().as_millis();
                if elapsed < 500 {
                    meta.request_count += 1;
                    if meta.request_count > 5 {
                        pow_required = true;
                    }
                } else {
                    meta.request_count = 0;
                }
                meta.last_request = Instant::now();
            }

            // --- Phase 2: PoW Gating ---
            if pow_required {
                let pow_header = req.headers().get("X-Palisade-PoW");
                let mut verified = false;
                
                if let Some(val) = pow_header {
                    if let Ok(pow_str) = val.to_str() {
                        let parts: Vec<&str> = pow_str.split(':').collect();
                        if parts.len() == 2 {
                            let (nonce, solution) = (parts[0], parts[1]);
                            let mut hasher = Sha256::new();
                            hasher.update(nonce.as_bytes());
                            hasher.update(solution.as_bytes());
                            let hash = hasher.finalize();
                            if hash[0] == 0 && hash[1] == 0 {
                                verified = true;
                            }
                        }
                    }
                }

                if !verified {
                    println!("   [PALISADE] 🛡️ IP {} triggered PoW Gate. Challenge issued.", ip);
                    let challenge_nonce: u64 = thread_rng().gen();
                    let res = HttpResponse::PreconditionRequired()
                        .insert_header(("X-Palisade-Challenge", challenge_nonce.to_string()))
                        .body("Sovereign PoW Verification Required. Solve SHA256(nonce + solution) with 2 leading zero bytes.");
                    
                    return Ok(req.into_response(res).map_into_right_body());
                }
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
