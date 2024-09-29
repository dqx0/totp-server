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

use totp_rs::{Algorithm, TOTP, Secret};
use tonic::Request;

pub mod totp_auth {
    tonic::include_proto!("dqx0.totp_auth.v1");
}

use totp_auth::auth_service_client::AuthServiceClient;
use totp_auth::{GenerateTotpRequest, ValidateTotpRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthServiceClient::connect("http://127.0.0.1:50051").await?;

    let generate_request = GenerateTotpRequest {
        account_name: "user@example.com".into(),
    };

    let response = client.generate_totp(Request::new(generate_request)).await?;
    let secret = response.into_inner().key;

    // TOTPインスタンスを作成
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret.to_string()).to_bytes().unwrap()
    ).unwrap();

    // 現在のタイムスタンプを取得してTOTPコードを生成
    let token = totp.generate_current().unwrap();

    // TOTPの検証リクエスト
    let validate_request = ValidateTotpRequest {
        token,
        secret,
    };

    // TOTPの検証
    let validate_response = client.validate_totp(Request::new(validate_request)).await?;
    let is_valid = validate_response.into_inner().valid;

    println!("TOTP is valid: {}", result(is_valid));

    Ok(())
}
fn result(valid: bool) -> String {
    if valid {
        return "\x1b[1;32mTRUE\x1b[0m".to_string();
    }
    "\x1b[1;31mFALSE\x1b[0m".to_string()
}