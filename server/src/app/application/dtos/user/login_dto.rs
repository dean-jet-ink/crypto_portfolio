#[derive(Debug)]
pub struct LoginInput {
    email: String,
    name: String,
    provider: String,
}

impl LoginInput {
    pub fn new(email: String, name: String, provider: String) -> Self {
        Self {
            email,
            name,
            provider,
        }
    }
}
