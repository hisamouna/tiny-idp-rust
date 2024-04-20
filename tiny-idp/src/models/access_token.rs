use super::auth_code::gen_rand;
use crate::models::context::USERS;
use chrono::{Duration, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: NaiveDate,
    pub user_id: String,
}

impl AccessToken {
    pub fn new(token: String, expires_at: NaiveDate, user_id: String) -> Self {
        AccessToken {
            token,
            expires_at,
            user_id,
        }
    }
    pub fn build(user_id: String) -> AccessToken {
        let token = gen_rand();
        let expires_at = gen_one_day_after_now();
        AccessToken::new(token, expires_at, user_id)
    }

    pub fn save(self) {
        if USERS
            .lock()
            .unwrap()
            .access_tokens
            .iter()
            .any(|ac| ac.user_id == self.user_id)
        {
            println!("Access_token.update");
            let index = USERS
                .lock()
                .unwrap()
                .access_tokens
                .iter()
                .position(|db| db.user_id == self.user_id)
                .unwrap();
            USERS.lock().unwrap().access_tokens[index] = self;
        } else {
            println!("Access_token.save");
            USERS.lock().unwrap().access_tokens.push(self);
        }
    }

    pub fn is_valid(self) -> bool {
        self.expires_at > Utc::now().date_naive()
    }
}

fn gen_one_day_after_now() -> NaiveDate {
    let now = Utc::now();
    let one_min_later = now + Duration::days(1);
    one_min_later.date_naive()
}
