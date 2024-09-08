use crate::{config::env::Env, routes::create_app};

pub async fn boot() {
    let app = create_app().await;

    let addr = format!("{}:{}", Env::api_domain(), Env::port());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("listening on {}", &addr);

    axum::serve(listener, app).await.unwrap();
}
