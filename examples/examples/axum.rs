use axum::{
    handler::{get, Handler},
    Router,
};
use tower::limit::ConcurrencyLimitLayer;
use std::{net::SocketAddr, time::Duration};
use hitbox_axum::CacheLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route(
        "/",
        get(handler.layer(ConcurrencyLimitLayer::new(100))),
    );

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() {}