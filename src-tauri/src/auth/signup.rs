use serde::{Deserialize, Serialize};

use crate::auth::{BackendHttpResponse, AUTH_ENPOINT, SIGNUP_URL};


const UNRECOGNIZED_ERROR: &'static str = "Unrecognized error";


#[derive(Serialize, Deserialize)]
pub struct SignUpSchema {
    username: String
}


#[derive(Deserialize)]
pub struct SignUpResponse {
    pub username: String,
    pub jwt: String,
    pub credential: Vec<u8>
}


pub async fn signup(signup_schema: SignUpSchema) -> Result<SignUpResponse, String> {
    let client = reqwest::Client::new();
    let backend_response = client.post(&format!("{}/{}", AUTH_ENPOINT, SIGNUP_URL))
        .json(&signup_schema)
        .send()
        .await
        .unwrap();

    let response: BackendHttpResponse = backend_response.json().await.unwrap();

    if let Some(err) = response.error {
        return Err(err)
    };

    let signup_response: SignUpResponse = serde_json::from_value(
            response.data.unwrap()
        ).unwrap();

    Ok(signup_response)
}