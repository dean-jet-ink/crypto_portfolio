use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::response::{Html, IntoResponse};

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
