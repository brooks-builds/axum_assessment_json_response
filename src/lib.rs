mod router;

use axum::Router;
use router::create_router;
use std::net::SocketAddr;

pub async fn run() {
    tracing_subscriber::fmt::init();
    let app = create_router(Router::new()).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
