use serde::Deserialize;

pub mod login;
pub mod signup;


const AUTH_ENPOINT: &'static str = "http://127.0.0.1:8000";
const SIGNUP_URL: &'static str = "http://127.0.0.1:8000";


#[derive(Deserialize)]
struct BackendHttpResponse {
    data: Option<serde_json::Value>,
    error: Option<String>
}