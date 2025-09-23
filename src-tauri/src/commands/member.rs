use tauri::command;


#[command]
async fn find_member(username: String) -> Option<String> {
    
}


pub mod handlers {
    use async_trait::async_trait;
    use serde::Deserialize;

    use crate::ws::ws_handler::WsHandler;


    #[derive(Deserialize)]
    struct FindMemberHandlerResponse {
        username: Option<String>
    }

    pub struct FindMemberHandler;

    #[async_trait]
    impl WsHandler for FindMemberHandler {
        async fn handle(&self, message: Option<serde_json::Value>) -> Result<Option<serde_json::Value>, String> {
            let msg = message.ok_or("User not found".to_string())?;

            let response: FindMemberHandlerResponse = 
                serde_json::from_value(msg)
                    .map_err(|_err| format!("Wrong Incoming data.\n{:?}", _err).to_string())?;

            Ok(None)
        }
    }
}