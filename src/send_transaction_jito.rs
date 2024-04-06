use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    signature::Signature,
    transaction::Transaction,
};
use std::{str::FromStr, time::Duration};
use bincode::serialize;
use bs58; // Ensure bs58 is added to your dependencies in Cargo.toml

#[derive(Serialize)]
struct JitoPayload {
    jsonrpc: &'static str,
    id: u8,
    method: &'static str,
    params: Vec<String>,
}

#[derive(Deserialize)]
struct JitoResponse {
    result: Option<String>,
    error: Option<JitoError>,
}

#[derive(Deserialize)]
struct JitoError {
    code: i32,
    message: String,
}

// Change the return type to `Result<Signature, Box<dyn std::error::Error>>`
pub async fn send_transaction_jito(tx: &Transaction) -> Result<Signature, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let jito_url = "https://mainnet.block-engine.jito.wtf/api/v1/transactions";

    // Serialize the transaction using bincode
    let serialized_tx = serialize(tx)?;
    
    // Encode the serialized transaction using bs58
    let encoded_tx = bs58::encode(serialized_tx).into_string();

    let payload = JitoPayload {
        jsonrpc: "2.0",
        id: 1,
        method: "sendTransaction",
        params: vec![encoded_tx],
    };

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client
        .post(jito_url)
        .json(&payload)
        .headers(headers)
        .send()
        .await?;

    let response_body: JitoResponse = response.json().await?;

    // Handle the response by parsing the Base58 string into a Signature
    match response_body.result {
        Some(sig) => {
            let signature = Signature::from_str(&sig)?;
            Ok(signature)
        },
        None => {
            if let Some(error) = response_body.error {
                Err(format!("Error sending transaction: {} - {}", error.code, error.message).into())
            } else {
                Err("Unknown error sending transaction".into())
            }
        },
    }
}
