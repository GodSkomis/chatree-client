use std::sync::Arc;

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::auth::signup::SignUpResponse;


static USER_STATE: Lazy<Arc<Mutex<UserState>>> = 
    Lazy::new(|| Arc::new(Mutex::new(UserState::default())));


#[derive(Default, Debug, Clone)]
pub struct UserState {
    pub username: String,
    pub jwt: String,
    pub credential: Vec<u8>,
}


impl UserState {
    pub fn update(schema: SignUpResponse) {
        let mut user_state = USER_STATE.lock();
        user_state.username = schema.username;
        user_state.jwt = schema.jwt;
        user_state.credential = schema.credential;
    }

    pub fn snapshot() -> Self {
        let user_state = USER_STATE.lock();
        user_state.clone()
    }
}
