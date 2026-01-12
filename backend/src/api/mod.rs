pub mod login;
pub mod workouts;
pub mod dashboard;
pub mod middleware;
use std::sync::Arc;
use log::error;
use actix_web::{web, App};
use crate::{api::{login::Login, workouts::Workouts}, services::{auth_service::AuthService, jwt_service::JwtService, post_service::PostService, get_service::GetService}};

#[derive(Clone)]
pub struct API{
    auth_service: Arc<AuthService>,
    jwt_service: Arc<JwtService>,
    post_service: Arc<PostService>,
    get_service: Arc<GetService>,
    login_api : Option<Login>,
    workouts_api: Option<Workouts>
}
impl API{
    pub fn new(auth_service: Arc<AuthService>, jwt_service: Arc<JwtService>, post_service: Arc<PostService>, get_service: Arc<GetService>) -> Self{
        API{
            auth_service,
            jwt_service,
            post_service,
            get_service,
            login_api: None,
            workouts_api: None
        }
    }

    pub async  fn init(&mut self ){
        let login_api = Login::new(self.auth_service.clone());
        self.login_api = Some(login_api);
        if self.login_api.is_none(){
            error!("Login API could not be initialised");
        }

        let workouts_api = Workouts::new(self.post_service.clone(), self.get_service.clone());
        self.workouts_api = Some(workouts_api);
    }

    pub fn configure(&self, cfg: &mut web::ServiceConfig){
        cfg.app_data(web::Data::from(self.auth_service.clone()))
           .app_data(web::Data::from(self.jwt_service.clone()))
           .app_data(web::Data::from(self.post_service.clone()))
           .app_data(web::Data::from(self.get_service.clone()));

        // configure routes
        cfg.service(
            web::scope("/api")
                .route("/login", web::post().to(crate::api::login::Login::login_handler))
                .route("/register", web::post().to(crate::api::login::Login::register_handler))
                .route("/forgot-password", web::post().to(crate::api::login::Login::forgot_password_handler))
                .route("/verify-token", web::get().to(crate::api::login::Login::verify_token_handler))
                .route("/workouts/addsession", web::post().to(crate::api::workouts::Workouts::workout_session_handler))
                .route("/workouts/session/{id}", web::put().to(crate::api::workouts::Workouts::update_session_handler))
                .route("/workouts/session/{id}", web::delete().to(crate::api::workouts::Workouts::delete_session_handler))
                .route("/workouts/addset", web::post().to(crate::api::workouts::Workouts::workout_set_handler))
                .route("/workouts/set/{id}", web::put().to(crate::api::workouts::Workouts::update_set_handler))
                .route("/workouts/set/{id}", web::delete().to(crate::api::workouts::Workouts::delete_set_handler))
                .route("/workouts/addcardio", web::post().to(crate::api::workouts::Workouts::cardio_log_handler))
                .route("/workouts/cardio/{id}", web::put().to(crate::api::workouts::Workouts::update_cardio_handler))
                .route("/workouts/cardio/{id}", web::delete().to(crate::api::workouts::Workouts::delete_cardio_handler))
                .route("/workouts/history", web::get().to(crate::api::workouts::Workouts::history_handler))
                .route("/workouts/session/{id}", web::get().to(crate::api::workouts::Workouts::session_details_handler))
                .route("/updateuser", web::put().to(crate::api::login::Login::update_user_handler))
                .route("/userinfo", web::get().to(crate::api::login::Login::user_info_handler))
                .route("/monthlylevels", web::post().to(crate::api::dashboard::Dashboard::monthly_workout_levels_handler))
                .route("/performancemetrics", web::post().to(crate::api::dashboard::Dashboard::performance_data_handler))
                .route("/mslegrpsumm", web::post().to(crate::api::dashboard::Dashboard::musclegrp_summary_handler))
                .route("/workouts/muscle_groups", web::get().to(crate::api::workouts::Workouts::get_muscle_groups_handler))
                .route("/workouts/variations", web::get().to(crate::api::workouts::Workouts::get_variations_handler))
                .route("/workouts/cardio_exercises", web::get().to(crate::api::workouts::Workouts::get_cardio_exercises_handler))
        );
    } 
}