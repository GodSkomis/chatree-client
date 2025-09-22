use tauri::command;

use crate::auth::{signup::SignUpSchema, state::UserState};


#[command]
pub async fn signup(schema: &str) -> Result<String, String> { // Result<username: Str, err: Str>
    let signup_schema: SignUpSchema = serde_json::from_str(schema).map_err(|e| e.to_string())?;
    let result = crate::auth::signup::signup(signup_schema).await?;
    let username = result.username.clone();
    UserState::update(result);

    Ok(username)
}
