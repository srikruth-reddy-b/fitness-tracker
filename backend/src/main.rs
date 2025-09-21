mod api;
mod services;
mod db;
mod configuration;
use std::sync::Arc;
use env_logger::Env;
use log::{error, info};

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_cors::Cors;
use crate::{api::API, configuration::Config, db::Database, services::{jwt_service::JwtService, Service}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("Launching backend application...");
    let conf = match Config::load(){
        Ok(c) => c,
        Err(err) => {
            error!("{}",err);
            return Ok(());
        }
    };
    let server = conf.get_server_properties();
    let db = conf.get_db_properties();
    let schema = db.schema;
    let mut db = Database::new(schema.clone());
    let _ = db.init().await;
    let _ = db.init_instances().await;
    let db_ops = db.database.unwrap();
    let mut service_ins = Service::new(db_ops, schema);
    let _ = service_ins.init().await;
    let auth_service = service_ins.auth_service.unwrap();
    let auth_service_clone = auth_service.clone();
    // api_ins.init().await;
    
    let jwt_service = JwtService::new();
    let jwt_service_arc = Arc::new(jwt_service.clone());
    let mut api_ins = api::API::new(auth_service,jwt_service_arc);
    api_ins.init().await;
    // let addr = "0.0.0.0:3000";
    let addr = format!("{}:{}",server.ip,server.port);
    info!("🚀 Backend running on https://{}", addr);
    HttpServer::new(move || {
        let cors = Cors::default()
            // .allow_any_origin()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .configure(|cfg| api_ins.configure(cfg))
    })
    .bind(addr)?
    .run()
    .await
}
