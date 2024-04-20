use crate::models::context::USERS;
use chrono::{Duration, NaiveDate, Utc};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct AuthCode {
    pub code: String,
    pub user_id: String,
    pub client_id: String,
    pub expires_at: NaiveDate,
    pub used_at: Option<NaiveDate>,
    pub redirect_uri: String,
}

impl AuthCode {
    pub fn new(
        code: String,
        user_id: String,
        client_id: String,
        expires_at: NaiveDate,
        redirect_uri: String,
    ) -> Self {
        AuthCode {
            code,
            user_id,
            client_id,
            expires_at,
            used_at: None,
            redirect_uri,
        }
    }
    pub fn save(self) {
        if USERS
            .lock()
            .unwrap()
            .auth_codes
            .iter()
            .any(|auth_code| auth_code.code == self.code)
        {
            println!("AuthCode.update");
            let index = USERS
                .lock()
                .unwrap()
                .auth_codes
                .iter()
                .position(|auth_code| auth_code.code == self.code)
                .unwrap();
            USERS.lock().unwrap().auth_codes[index] = self;
        } else {
            println!("AuthCode.save");
            USERS.lock().unwrap().auth_codes.push(self);
        }
    }
}

pub fn build(user_id: String, client_id: String, redirect_uri: &String) -> AuthCode {
    let code = gen_rand();
    let expires_at = gen_one_min_after_now();
    AuthCode::new(code, user_id, client_id, expires_at, redirect_uri.clone())
}

pub fn gen_rand() -> String {
    // 乱数生成器を初期化
    let mut rng = rand::thread_rng();
    // u32の乱数を生成
    let random_number = rng.gen::<u32>();
    // 36進数に変換
    let base36_encoded = base36_encode(random_number);
    // 最後の8文字を取得（存在する場合）
    base36_encoded
        .chars()
        .rev()
        .take(8)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}

fn gen_one_min_after_now() -> NaiveDate {
    let now = Utc::now();
    let one_min_later = now + Duration::minutes(1);
    one_min_later.date_naive()
}

fn base36_encode(mut num: u32) -> String {
    let mut chars = Vec::new();
    let base_chars = "0123456789abcdefghijklmnopqrstuvwxyz";

    while num > 0 {
        let remainder = (num % 36) as usize;
        chars.push(base_chars.chars().nth(remainder).unwrap());
        num /= 36;
    }

    chars.iter().rev().collect()
}
