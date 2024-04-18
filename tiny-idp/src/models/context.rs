use crate::models::auth_code;
use crate::models::user;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Context {
    pub users: Vec<user::User>,
    pub auth_codes: Vec<auth_code::AuthCode>,
}

pub static USERS: Lazy<Mutex<Context>> = Lazy::new(|| {
    Mutex::new(Context {
        users: vec![user::User {
            id: 1,
            email: "tiny-idp@asmsuechan.com".to_string(),
            password: "p@ssw0rd".to_string(),
            client_id: "tiny-client".to_string(),
        }],
        auth_codes: vec![],
    })
});
