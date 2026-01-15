use env_logger::Env;
use log::{error, info};
use actix_web::{middleware::Logger, App, HttpServer};
use actix_cors::Cors;
use backend::{configuration::Config, db::Database, services::Service, api};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Launching backend application...");
    let conf = match Config::load(){
        Ok(c) => c,
        Err(err) => {
            error!("{}",err);
            return Ok(());
        }
    };

    let server = conf.get_server_properties();
    let mut db = Database::new();

    if let Err(err) = db.init().await{
        error!("Error initialising database, {}",err)
    };
    let db_ops = db.database.unwrap();
    info!("Database initialized successfully.");

    let mut service_ins = Service::new(db_ops);
    if let Err(err) = service_ins.init().await{
        error!("Error initialising services, {}",err);
    };
    info!("Services initialized successfully.");

    let auth_service = service_ins.auth_service.unwrap();
    let post_service = service_ins.post_service.unwrap();
    let get_service = service_ins.get_service.unwrap();
    let put_service = service_ins.put_service.unwrap();
    let jwt_service = service_ins.jwt_service.unwrap();
    let mut api_ins = api::API::new(auth_service,jwt_service,post_service, get_service,put_service);
    api_ins.init().await;
    info!("API initialized successfully.");

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
