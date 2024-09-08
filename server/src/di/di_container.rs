use std::sync::Arc;

use shaku::module;

use crate::app::{
    application::use_cases::user::login_use_case::LoginUseCase,
    domain::repositories::user::user_repository::UserRepository,
};

use super::{repository_provider::RepositoryProvider, use_case_provider::UseCaseProvider};

module! {
  pub DiContainer {
    components = [],
    providers = [],

    use RepositoryProvider {
      components = [dyn UserRepository],
      providers = [],
    },

    use UseCaseProvider {
      components = [dyn LoginUseCase],
      providers = [],
    }
  }
}

impl DiContainer {
    pub async fn new() -> Self {
        let repository_provider = Arc::new(RepositoryProvider::new().await);
        let use_case_provider = Arc::new(UseCaseProvider::new().await);

        Self::builder(repository_provider, use_case_provider).build()
    }
}

impl Clone for DiContainer {
    fn clone(&self) -> Self {
        Self::builder(self.__di_submodule_0.clone(), self.__di_submodule_1.clone()).build()
    }
}
