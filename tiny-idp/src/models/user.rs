#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub client_id: String,
}

impl User {
    pub fn new(id: u64, email: String, password: String, client_id: String) -> Self {
        Self {
            id,
            email,
            password,
            client_id,
        }
    }

    pub fn find_by_email<'a>(db: &'a Vec<User>, email: &str) -> Option<&'a User> {
        db.iter().find(|user| user.email == email)
    }

    pub fn login(db: &Vec<User>, email: &str, password: &str) -> bool {
        if let Some(_) = db
            .iter()
            .find(|user| user.email == email && user.password == password)
        {
            true
        } else {
            false
        }
    }
}
