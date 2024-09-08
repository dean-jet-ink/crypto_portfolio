use bcrypt::{hash, DEFAULT_COST};
use ddd_macros::ValueObject;
use serde::{Deserialize, Serialize};

#[derive(ValueObject, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn from_plain_text(value: String) -> Self {
        let cipher_text = hash(value, DEFAULT_COST).unwrap();
        Self { value: cipher_text }
    }
}
