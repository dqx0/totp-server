/**
 *  Copyright 2024 dqx0
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 */

use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{transport::Server, Request, Response, Status};
use totp_rs::{Algorithm, TOTP, Secret};

pub mod totp_auth {
    tonic::include_proto!("dqx0.totp_auth.v1");
}

use totp_auth::auth_service_server::{AuthService, AuthServiceServer};
use totp_auth::{GenerateTotpRequest, GenerateTotpResponse, ValidateTotpRequest, ValidateTotpResponse};

#[derive(Default)]
pub struct AuthServiceImpl {}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn generate_totp(
        &self,
        request: Request<GenerateTotpRequest>,
    ) -> Result<Response<GenerateTotpResponse>, Status> {
        let req = request.into_inner();
        
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, req.account_name.as_bytes().to_vec())
            .map_err(|e| Status::internal(format!("Failed to generate TOTP: {}", e)))?;
        
        let key = totp.get_secret_base32().to_string();
        println!("Generated TOTP key for {}: {}", req.account_name, key);

        let response = GenerateTotpResponse {
            key,
        };
        Ok(Response::new(response))
    }

    async fn validate_totp(
        &self,
        request: Request<ValidateTotpRequest>,
    ) -> Result<Response<ValidateTotpResponse>, Status> {
        let req = request.into_inner();
        
        let totp = TOTP::new(
                Algorithm::SHA1, 
                6, 
                1, 
                30, 
                Secret::Encoded(req.secret.to_string()).to_bytes().unwrap()
            ).map_err(|e| Status::internal(format!("Failed to validate TOTP: {}", e)))?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let is_valid = totp.check(req.token.as_str(), now);

        println!("Validating TOTP token {}: {}", req.token, is_valid);

        let response = ValidateTotpResponse {
            valid: is_valid,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse::<SocketAddr>()?;
    let auth_service = AuthServiceImpl::default();

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
