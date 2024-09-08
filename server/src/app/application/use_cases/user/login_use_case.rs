use anyhow::Result;
use shaku::Interface;

use crate::app::application::dtos::user::login_dto::LoginInput;

pub trait LoginUseCase: Interface {
    fn handle(&self, login_dto: LoginInput) -> Result<()>;
}
