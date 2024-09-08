use std::fs::File;
use std::io::Write;

use async_graphql::{EmptySubscription, Schema};

use crate::{
    app::presentation::resolvers::{mutations::MutationRoot, queries::QueryRoot},
    config::env::Env,
    di::di_container::DiContainer,
};

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema<'a>(di_container: Option<DiContainer>) -> MySchema {
    match di_container {
        Some(di_container) => Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .data(di_container)
        .finish(),
        None => Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .finish(),
    }
}

pub async fn generate_schema() {
    let path = Env::graphql_schema_path();
    println!("Generating schema at: {}", path);

    let schema = create_schema(None);
    let sdl = schema.sdl();

    tracing::debug!("start to create schema: {}", sdl);
    let mut file = File::create(path).unwrap();
    file.write_all(sdl.as_bytes()).unwrap();
}
