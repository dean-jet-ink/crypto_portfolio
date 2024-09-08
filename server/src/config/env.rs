pub struct Env {}

impl Env {
    pub fn init() {
        dotenv::dotenv().ok();
    }

    pub fn api_domain() -> String {
        dotenv::var("API_DOMAIN").unwrap()
    }

    pub fn port() -> String {
        dotenv::var("PORT").unwrap()
    }

    pub fn rust_log() -> String {
        dotenv::var("RUST_LOG").unwrap_or(String::from("info"))
    }

    pub fn database_url() -> String {
        dotenv::var("DATABASE_URL").unwrap()
    }

    pub fn test_database_url() -> String {
        dotenv::var("TEST_DATABASE_URL").unwrap()
    }

    pub fn graphql_schema_path() -> String {
        dotenv::var("GRAPHQL_SCHEMA_PATH").unwrap_or(String::from("./src/graphql/schema.graphql"))
    }
}
