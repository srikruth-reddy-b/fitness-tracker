use actix_web::{HttpResponse, Responder, web};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::{api::middleware::AuthenticatedUser, services::get_service::{GetService}};

#[derive(Debug,Deserialize,Serialize)]
pub struct MonthlyWorkoutRequest{
    pub year: i32,
    pub month: i32
}
#[derive(Debug, Deserialize,Serialize)]
pub struct PerformanceRequest{
    pub variation_id: i32,
    pub start_date: String,
    pub end_date: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuscleGroupSummaryRequest{
    pub muscle_group_ids: Vec<i32>,
    pub start_date: String,
    pub end_date: String
}

pub struct Dashboard;
impl Dashboard{
    pub fn new() -> Self{
        Dashboard
    }

    pub async fn monthly_workout_levels_handler(
        get_service:web::Data<GetService>,
        user: AuthenticatedUser,
        payload: web::Json<MonthlyWorkoutRequest>,
    ) -> impl Responder{
        let user_id = user.id;
        let payload_inner = payload.into_inner();
        let year = payload_inner.year;
        let month = payload_inner.month;

        match get_service.get_workout_levels(user_id, year, month).await{
            Ok(sessions) => return HttpResponse::Ok().json(sessions),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
        };
        // HttpResponse::Ok().json({})
    }

    pub async fn performance_data_handler(
        get_service: web::Data<GetService>,
        user: AuthenticatedUser,
        payload: web::Json<PerformanceRequest>
    ) -> impl Responder{

        let user_id = user.id;
        let payload_inner = payload.into_inner();
        let variation_id = payload_inner.variation_id;
        let start_date = match NaiveDate::parse_from_str(&payload_inner.start_date, "%Y-%m-%d"){
            Ok(date) => date,
            Err(err) => return HttpResponse::BadRequest().body(format!("Invalid start_date format: {}", err))
        };
        let end_date = match NaiveDate::parse_from_str(&payload_inner.end_date, "%Y-%m-%d"){
            Ok(date) => date,
            Err(err) => return HttpResponse::BadRequest().body(format!("Invalid end_date format: {}", err))
        };
        match get_service.get_performance_details(user_id, variation_id, start_date, end_date).await{
            Ok(data) => return HttpResponse::Ok().json(data),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
        }
    }

    pub async fn musclegrp_summary_handler(
        get_service: web::Data<GetService>,
        user: AuthenticatedUser,
        payload: web::Json<MuscleGroupSummaryRequest> 
    ) -> impl Responder{
        let user_id = user.id;
        let payload_inner = payload.into_inner();
        let muscle_group_ids = payload_inner.muscle_group_ids;
        let start_date = match NaiveDate::parse_from_str(&payload_inner.start_date, "%Y-%m-%d"){
            Ok(date) => date,
            Err(err) => return HttpResponse::BadRequest().body(format!("Invalid start_date format: {}", err))
        };
        let end_date = match NaiveDate::parse_from_str(&payload_inner.end_date, "%Y-%m-%d"){
            Ok(date) => date,
            Err(err) => return HttpResponse::BadRequest().body(format!("Invalid end_date format: {}", err))
        };

        match get_service.get_numberof_sets_per_musclegroup(user_id, start_date, end_date, muscle_group_ids).await{
            Ok(summary) => return HttpResponse::Ok().json(summary),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
        }
    }


}

