mod api;
mod services;
mod models;
mod db;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    println!("starting application");
    let app = api::init().layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    let addr = "0.0.0.0:3000";
    println!("🚀 Backend running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    )
    .await
    .unwrap();
}
