use std::sync::Arc;

use anyhow::Result;
use shaku::Component;

use crate::app::{
    application::{
        dtos::user::login_dto::LoginInput, use_cases::user::login_use_case::LoginUseCase,
    },
    domain::repositories::user::user_repository::UserRepository,
};

#[derive(Component)]
#[shaku(interface = LoginUseCase)]
pub struct LoginInteractor {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl LoginUseCase for LoginInteractor {
    fn handle(&self, login_input: LoginInput) -> Result<()> {
        todo!()
    }
}
