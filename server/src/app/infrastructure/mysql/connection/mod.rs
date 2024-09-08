use std::{sync::Arc, time::Duration};

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use crate::config::env::Env;

pub struct MySqlPoolProvider {
    pool: Arc<MySqlPool>,
}

impl MySqlPoolProvider {
    pub async fn new() -> Self {
        let db_url = Env::database_url();

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&db_url)
            .await
            .unwrap();

        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn pool(&self) -> Arc<MySqlPool> {
        self.pool.clone()
    }
}
