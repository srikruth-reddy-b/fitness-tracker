use std::sync::Arc;
use log::{error, info};
use serde::Serialize;
use crate::db::{logger::LoggerDB, model::{UpdateCardioLog, UpdateWorkoutSession, UpdateWorkoutSet}};

#[derive(Debug, Serialize)]
pub struct PutResponse{
    pub user_id: i32,
    pub id: Option<i32>,
    pub success: bool,
    pub message: String,
}
pub struct PutService{
    logger: Arc<LoggerDB>
}

impl PutService{
    pub fn new(logger: Arc<LoggerDB>) -> Self{
        PutService{
            logger
        }
    }

    pub async fn update_workout_session(&self, user_id: i32, session_id: i32, title: Option<String>, date: Option<chrono::NaiveDate>, start_time: Option<chrono::NaiveDateTime>, end_time: Option<chrono::NaiveDateTime>, notes: Option<String>) -> PutResponse {
        let update_data = UpdateWorkoutSession { title, date, start_time, end_time, notes };
        match self.logger.update_workout_session(user_id, session_id, update_data).await {
            Ok(session) =>{ 
                info!("Workout session updated: {:?}", session);
                PutResponse {
                    user_id,
                    id: Some(session.id),
                    success: true,
                    message: "Session Updated".to_string()
                }
            },
            Err(err) => {
                error!("Error updating workout session for user_id {}: {}", user_id, err);
                PutResponse {
                    user_id,
                    id: None,
                    success: false,
                    message: format!("{}", err)
                }
            }
        }
    }

    pub async fn update_workout_set(&self, user_id: i32, set_id: i32, weight: Option<f64>, reps: Option<i32>) -> PutResponse {
        let update_data = UpdateWorkoutSet { weight, reps };
        match self.logger.update_workout_set(user_id, set_id, update_data).await {
            Ok(set) => {
                info!("Workout set updated: {:?}", set);
                PutResponse {
                    user_id,
                    id: Some(set.id),
                    success: true,
                    message: "Set Updated".to_string()
                }
            },
            Err(err) => {
                error!("Error updating workout set for user_id {}: {}", user_id, err);
                PutResponse {
                    user_id,
                    id: None,
                    success: false,
                    message: format!("{}", err)
                }
            }
        }
    }

    pub async fn update_cardio_log(&self, user_id: i32, log_id: i32, duration_minutes: Option<i32>) -> PutResponse {
        let update_data = UpdateCardioLog { duration_minutes };
        match self.logger.update_cardio_log(user_id, log_id, update_data).await {
            Ok(log) => {
                info!("Cardio log updated: {:?}", log);
                PutResponse {
                    user_id,
                    id: Some(log.id),
                    success: true,
                    message: "Cardio Log Updated".to_string()
                }
            },
            Err(err) => {
                error!("Error updating cardio log for user_id {}: {}", user_id, err);
                PutResponse {
                    user_id, id: None, success: false, message: format!("{}", err)
                }
            }
        }
    }

    pub async fn delete_workout_session(&self, user_id: i32, session_id: i32) -> PutResponse {
        match self.logger.delete_workout_session(user_id, session_id).await {
            Ok(_) => {
                info!("Workout session deleted: {}", session_id);
                PutResponse {
                    user_id, id: Some(session_id), success: true, message: "Session Deleted".to_string()
                }
            },
            Err(err) => {
                error!("Error deleting workout session for user_id {}: {}", user_id, err);
                PutResponse {
                    user_id, id: None, success: false, message: format!("{}", err)
                }
            }
        }
    }

    pub async fn delete_workout_set(&self, user_id: i32, set_id: i32) -> PutResponse {
        match self.logger.delete_workout_set(user_id, set_id).await {
            Ok(_) => {
                info!("Workout set deleted: {}", set_id);
                PutResponse {
                    user_id, id: Some(set_id), success: true, message: "Set Deleted".to_string()
                }
            },
            Err(err) => {
                error!("Error deleting workout set for user_id {}: {}", user_id, err);
                PutResponse {
                    user_id, id: None, success: false, message: format!("{}", err)
                }
            }
        }
    }

    pub async fn delete_cardio_log(&self, user_id: i32, log_id: i32) -> PutResponse {
        match self.logger.delete_cardio_log(user_id, log_id).await {
            Ok(_) => {
                info!("Cardio log deleted: {}", log_id);
                PutResponse {
                    user_id, id: Some(log_id), success: true, message: "Cardio Log Deleted".to_string()
                }
            },
            Err(err) => {
                error!("Error deleting cardio log for user_id {}: {}", user_id, err);
                PutResponse {
                    user_id, id: None, success: false, message: format!("{}", err)
                }
            }
        }
    }
}