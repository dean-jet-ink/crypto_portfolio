use serde::{Deserialize, Serialize};

use crate::app::domain::value_objects::user::{password::Password, user_id::UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    user_id: UserId,
    name: String,
    email: String,
    password: Password,
}

impl UserEntity {
    pub fn new(user_id: UserId, name: String, email: String, password: Password) -> Self {
        Self {
            user_id,
            name,
            email,
            password,
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
