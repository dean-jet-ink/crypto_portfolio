use anyhow::Result;
use axum::async_trait;
use shaku::Interface;

use crate::app::domain::entities::user::user_entity::UserEntity;

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_by_id(&self, id: &str) -> Result<UserEntity>;
    async fn find_by_email(&self, email: &str) -> Result<UserEntity>;
    async fn save(&self, user: UserEntity) -> Result<UserEntity>;
    async fn update(&self, user: UserEntity) -> Result<UserEntity>;
    async fn delete(&self, id: &str) -> Result<bool>;
}
