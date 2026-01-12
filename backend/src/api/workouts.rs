use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use std::sync::Arc;
use crate::{db::user, services::{get_service::GetService, post_service::PostService}};
use actix_web::{web, HttpResponse, Responder};
use crate::api::middleware::AuthenticatedUser;

#[derive(Debug,Deserialize)]
pub struct StrengthSet{
    pub user_id: i32,
    pub workout_session_id: Option<i32>,
    pub muscle_group_id: i32,
    pub variation_id: i32,
    pub weight: f64,
    pub reps: i32,
    pub performed_on: NaiveDate
}

#[derive(Debug,Deserialize)]
pub struct CardioSet{
    pub user_id: i32,
    pub workout_session_id: Option<i32>,
    pub cardio_exercise_id: i32,
    pub duration: i32
}

#[derive(Debug,Deserialize)]
pub struct WorkoutSession{
    pub user_id: i32,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub date: NaiveDate,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSessionRequest {
    pub title: Option<String>,
    pub end_time: Option<NaiveDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSetRequest {
    pub weight: Option<f64>,
    pub reps: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardioRequest {
    pub duration: Option<i32>,
}

#[derive(Debug, serde::Serialize)]
pub struct SessionDetailsResponse {
    pub session: crate::db::model::WorkoutSession,
    pub sets: Vec<crate::db::model::WorkoutSet>,
    pub cardio_logs: Vec<crate::db::model::CardioLog>,
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub limit: Option<i64>,
}

#[derive(Clone)]
pub struct Workouts{
    post_service: Arc<PostService>,
    get_service: Arc<GetService>
}

impl Workouts{
    pub fn new(post_service: Arc<PostService>, get_service: Arc<GetService>) -> Self{
        Workouts { 
            post_service,
            get_service
        }
    }

    pub async fn workout_session_handler(post_service: web::Data<PostService>,
        payload: web::Json<WorkoutSession>,
        user: AuthenticatedUser,
    ) -> impl Responder {
        let mut session = payload.into_inner();
        session.user_id = user.id;
        let resp = post_service.add_workout_session(session).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn workout_set_handler(post_service: web::Data<PostService>,
        payload: web::Json<StrengthSet>,
        user: AuthenticatedUser,
    ) -> impl Responder {
        let mut set = payload.into_inner();
        set.user_id = user.id;
        let resp = post_service.add_workout_set(set).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn cardio_log_handler(post_service: web::Data<PostService>,
        payload: web::Json<CardioSet>,
        user: AuthenticatedUser,
    ) -> impl Responder {
        let mut log = payload.into_inner();
        log.user_id = user.id;
        let resp = post_service.add_cardio_set(log).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn update_session_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
        payload: web::Json<UpdateSessionRequest>,
    ) -> impl Responder {
        let session_id = path.into_inner();
        let req = payload.into_inner();
        let resp = post_service.update_workout_session(user.id, session_id, req.title, req.end_time, req.notes).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn delete_session_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
    ) -> impl Responder {
        let session_id = path.into_inner();
        let resp = post_service.delete_workout_session(user.id, session_id).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn update_set_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
        payload: web::Json<UpdateSetRequest>,
    ) -> impl Responder {
        let set_id = path.into_inner();
        let req = payload.into_inner();
        let resp = post_service.update_workout_set(user.id, set_id, req.weight, req.reps).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn delete_set_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
    ) -> impl Responder {
        let set_id = path.into_inner();
        let resp = post_service.delete_workout_set(user.id, set_id).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn update_cardio_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
        payload: web::Json<UpdateCardioRequest>,
    ) -> impl Responder {
        let log_id = path.into_inner();
        let req = payload.into_inner();
        let resp = post_service.update_cardio_log(user.id, log_id, req.duration).await;
        HttpResponse::Ok().json(resp)
    }

    pub async fn delete_cardio_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
    ) -> impl Responder {
        let log_id = path.into_inner();
        let resp = post_service.delete_cardio_log(user.id, log_id).await;
        HttpResponse::Ok().json(resp)
    }
    pub async fn history_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        query: web::Query<HistoryQuery>,
    ) -> impl Responder {
        let limit = query.limit.unwrap_or(20);
        match post_service.get_history(user.id, limit).await {
            Ok(sessions) => HttpResponse::Ok().json(sessions),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    pub async fn session_details_handler(
        post_service: web::Data<PostService>,
        user: AuthenticatedUser,
        path: web::Path<i32>,
    ) -> impl Responder {
        let session_id = path.into_inner();
        match post_service.get_session_details(user.id, session_id).await {
            Ok((session, sets, cardio)) => {
                let resp = SessionDetailsResponse {
                    session,
                    sets,
                    cardio_logs: cardio,
                };
                HttpResponse::Ok().json(resp)
            },
            Err(e) => HttpResponse::NotFound().body(format!("Error: {}", e)),
        }
    }

    pub async fn get_muscle_groups_handler(
        get_service: web::Data<GetService>,
        user: AuthenticatedUser,
    ) -> impl Responder {
        match get_service.get_muscle_groups(user.id).await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    pub async fn get_variations_handler(
        get_service: web::Data<GetService>,
        user: AuthenticatedUser,
    ) -> impl Responder {
        match get_service.get_variations(user.id).await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    pub async fn get_cardio_exercises_handler(
        get_service: web::Data<GetService>,
        user: AuthenticatedUser,
    ) -> impl Responder {
        match get_service.get_cardio_exercises(user.id).await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }
}