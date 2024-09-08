use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use shaku::Component;
use sqlx::MySqlPool;

use crate::app::domain::{
    entities::user::user_entity::UserEntity, repositories::user::user_repository::UserRepository,
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct MySqlUserRepository {
    pool: Arc<MySqlPool>,
}

impl MySqlUserRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<UserEntity> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<UserEntity> {
        todo!()
    }

    async fn save(&self, user: UserEntity) -> Result<UserEntity> {
        todo!()
    }

    async fn update(&self, user: UserEntity) -> Result<UserEntity> {
        todo!()
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        todo!()
    }
}
