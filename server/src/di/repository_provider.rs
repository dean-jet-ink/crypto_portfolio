use shaku::module;

use crate::app::infrastructure::mysql::{
    connection::MySqlPoolProvider,
    repositories::mysql_user_repository::{MySqlUserRepository, MySqlUserRepositoryParameters},
};

module! {
    pub RepositoryProvider {
        components = [MySqlUserRepository],
        providers = []
    }
}

impl RepositoryProvider {
    pub async fn new() -> Self {
        let pool_provider = MySqlPoolProvider::new().await;

        Self::builder()
            .with_component_parameters::<MySqlUserRepository>(MySqlUserRepositoryParameters {
                pool: pool_provider.pool(),
            })
            .build()
    }
}
