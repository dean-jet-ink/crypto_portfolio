use async_graphql_axum::GraphQL;
use axum::{routing::get, Router};

use crate::{
    app::presentation::handlers::graphql_handler, di::di_container::DiContainer,
    graphql::create_schema,
};

pub fn route(router: Router, di_container: &DiContainer) -> Router {
    let schema = create_schema(Some(di_container.clone()));

    router.route(
        "/graphql",
        get(graphql_handler::graphql_playground).post_service(GraphQL::new(schema)),
    )
}
