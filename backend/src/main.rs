mod api;
mod services;
mod db;
mod configuration;
use env_logger::Env;
use tower_http::cors::{CorsLayer, Any};
use log::{error, info};

use crate::{configuration::Config, db::database};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("Launching backend application...");
    let conf = match Config::load(){
        Ok(c) => c,
        Err(err) => {
            error!("{}",err);
            return;
        }
    };
    let server = conf.get_server_properties();
    let app = api::init().layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );
        
    // let addr = "0.0.0.0:3000";
    let handle = tokio::spawn(async move{
        let addr = format!("{}:{}",server.ip,server.port);
        info!("🚀 Backend running on http://{}", addr);
    
        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
            app
        )
        .await
        .unwrap();
    });
    let mut db = database::Database::new();
    db.init().await;
    if let Err(err) = db.create_tables().await{
        error!("{}",err);
    };
    let _ = handle.await;
}
