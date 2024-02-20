pub struct Account {
    pub mail: String,
    pub username: String,
    pub password: String,
}

impl Account {
    pub fn new (mail: String, username: String, password: String) -> Self {
        Self {
            mail,
            username,
            password,
        }
    }
}