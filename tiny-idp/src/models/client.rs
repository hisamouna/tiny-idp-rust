#[derive(Debug)]
pub struct Client {
    pub client_id: String,
    pub client_secret: String,
}

impl Client {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }
}
