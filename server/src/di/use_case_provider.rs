use std::sync::Arc;

use shaku::module;

use crate::app::{
    application::interactors::user::login_interactor::LoginInteractor,
    domain::repositories::user::user_repository::UserRepository,
};

use super::repository_provider::RepositoryProvider;

module! {
    pub UseCaseProvider {
        components = [LoginInteractor],
        providers = [],

        use RepositoryProvider {
            components = [dyn UserRepository],
            providers = [],
        }
    }
}

impl UseCaseProvider {
    pub async fn new() -> Self {
        let repository_provider = Arc::new(RepositoryProvider::new().await);

        Self::builder(repository_provider).build()
    }
}
