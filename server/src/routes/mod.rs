use axum::Router;

use crate::di::di_container::DiContainer;

pub mod graphql_router;

pub async fn create_app() -> Router {
    let router = Router::new();

    let di_container = DiContainer::new().await;

    graphql_router::route(router, &di_container)
}
