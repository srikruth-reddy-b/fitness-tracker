mod api;
mod services;
mod db;
mod configuration;
use std::sync::Arc;

use env_logger::Env;
use tower_http::cors::{CorsLayer, Any};
use log::{error, info};

use crate::{configuration::Config, db::Database, services::Service};

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
    let db = conf.get_db_properties();
    let schema = db.schema;
    let mut db = Database::new();
    let _ = db.init().await;
    let db_ops = db.database.unwrap();
    let mut service_ins = Service::new(db_ops, schema);
    let _ = service_ins.init().await;
    let auth_service = service_ins.auth_service.unwrap();
    let mut api_ins = api::API::new(Arc::new(auth_service));
    let app = api_ins.init().await.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );
        
    // let addr = "0.0.0.0:3000";
    let handle = tokio::spawn(async move{
        let addr = format!("{}:{}",server.ip,server.port);
        info!("🚀 Backend running on https://{}", addr);
    
        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
            app
        )
        .await
        .unwrap();
    });
    
    let _ = handle.await;
}
