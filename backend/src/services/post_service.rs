use std::sync::Arc;
use anyhow::bail;
use serde::Serialize;
use crate::{api::workouts::{ CardioSet, StrengthSet, WorkoutSession}, db::{model::{NewCardioLog, NewWorkoutSession, NewWorkoutSet, UpdateCardioLog, UpdateWorkoutSession, UpdateWorkoutSet}, workouts::WorkoutDB}};

#[derive(Debug, Serialize)]
pub struct PostResponse{
    pub user_id: i32,
    pub id: Option<i32>,
    pub success: bool,
    pub message: String,
}

pub struct PostService{
    workout: Arc<WorkoutDB>
}

impl PostService{
    pub fn new(workout: Arc<WorkoutDB>) -> Self{
        PostService{
            workout
        }
    }

    pub async fn add_workout_session(&self, session_request: WorkoutSession) -> PostResponse{
        let workout_session = NewWorkoutSession{
            user_id: session_request.user_id,
            date: session_request.date,
            title: session_request.title,
            notes: session_request.notes,
            start_time: session_request.start_time,
            end_time: session_request.end_time,
        };
        match self.workout.add_workout_session(workout_session).await{
            Ok(session) => {
                 PostResponse { 
                    user_id: session.user_id,
                    id: Some(session.id), 
                    success: true, 
                    message: "Session Added".to_string()
                }
            },
            Err(err) => {
                 PostResponse{
                    user_id: 1, // Default or check how to get user_id if failure early? session_request.user_id is available
                    id: None,
                    success: false,
                    message: format!("{}",err)
                }
            }
        }
    }

    pub async fn add_workout_set(&self, session_request: StrengthSet) -> PostResponse{
        let workout_session = NewWorkoutSet{
            user_id: session_request.user_id,
            workout_session_id: session_request.workout_session_id,
            variation_id: session_request.variation_id,
            weight: session_request.weight,
            reps: session_request.reps,
            performed_on: session_request.performed_on
        };
        if let Err(err) = self.workout.add_workout_set(workout_session).await{
            return PostResponse{
                user_id: session_request.user_id,
                id: None,
                success: false,
                message: format!("{}",err)
            }
        };

        PostResponse { 
            user_id: session_request.user_id, 
            id: None,
            success: true, 
            message: "Set Added".to_string()
        }
    }

    pub async fn add_cardio_set(&self, session_request: CardioSet) -> PostResponse{
        let workout_session = NewCardioLog{
            user_id: session_request.user_id,
            workout_session_id: session_request.workout_session_id,
            cardio_exercise_id: session_request.cardio_exercise_id,
            duration_minutes: session_request.duration
            
        };
        if let Err(err) = self.workout.add_workout_cardio(workout_session).await{
            return PostResponse{
                user_id: session_request.user_id,
                id: None,
                success: false,
                message: format!("{}",err)
            };
        }

        PostResponse { 
            user_id: session_request.user_id, 
            id: None,
            success: true, 
            message: "Cardio Log Added".to_string()
        }
    }

    pub async fn update_workout_session(&self, user_id: i32, session_id: i32, title: Option<String>, end_time: Option<chrono::NaiveDateTime>, notes: Option<String>) -> PostResponse {
        let update_data = UpdateWorkoutSession { title, end_time, notes };
        match self.workout.update_workout_session(user_id, session_id, update_data).await {
            Ok(session) => PostResponse {
                user_id,
                id: Some(session.id),
                success: true,
                message: "Session Updated".to_string()
            },
            Err(err) => PostResponse {
                user_id, id: None, success: false, message: format!("{}", err)
            }
        }
    }

    pub async fn delete_workout_session(&self, user_id: i32, session_id: i32) -> PostResponse {
        match self.workout.delete_workout_session(user_id, session_id).await {
            Ok(_) => PostResponse {
                user_id, id: Some(session_id), success: true, message: "Session Deleted".to_string()
            },
            Err(err) => PostResponse {
                user_id, id: None, success: false, message: format!("{}", err)
            }
        }
    }

    pub async fn update_workout_set(&self, user_id: i32, set_id: i32, weight: Option<f64>, reps: Option<i32>) -> PostResponse {
        let update_data = UpdateWorkoutSet { weight, reps };
        match self.workout.update_workout_set(user_id, set_id, update_data).await {
            Ok(set) => PostResponse {
                user_id, id: Some(set.id), success: true, message: "Set Updated".to_string()
            },
            Err(err) => PostResponse {
                user_id, id: None, success: false, message: format!("{}", err)
            }
        }
    }

    pub async fn delete_workout_set(&self, user_id: i32, set_id: i32) -> PostResponse {
        match self.workout.delete_workout_set(user_id, set_id).await {
            Ok(_) => PostResponse {
                user_id, id: Some(set_id), success: true, message: "Set Deleted".to_string()
            },
            Err(err) => PostResponse {
                user_id, id: None, success: false, message: format!("{}", err)
            }
        }
    }

    pub async fn update_cardio_log(&self, user_id: i32, log_id: i32, duration_minutes: Option<i32>) -> PostResponse {
        let update_data = UpdateCardioLog { duration_minutes };
        match self.workout.update_cardio_log(user_id, log_id, update_data).await {
            Ok(log) => PostResponse {
                user_id, id: Some(log.id), success: true, message: "Cardio Log Updated".to_string()
            },
            Err(err) => PostResponse {
                user_id, id: None, success: false, message: format!("{}", err)
            }
        }
    }

    pub async fn delete_cardio_log(&self, user_id: i32, log_id: i32) -> PostResponse {
        match self.workout.delete_cardio_log(user_id, log_id).await {
            Ok(_) => PostResponse {
                user_id, id: Some(log_id), success: true, message: "Cardio Log Deleted".to_string()
            },
            Err(err) => PostResponse {
                user_id, id: None, success: false, message: format!("{}", err)
            }
        }
    }

    pub async fn get_history(&self, user_id: i32, limit: i64) -> anyhow::Result<Vec<crate::db::model::WorkoutSession>> {
        self.workout.get_history(user_id, limit).await
    }

    pub async fn get_session_details(&self, user_id: i32, session_id: i32) -> anyhow::Result<(crate::db::model::WorkoutSession, Vec<crate::db::model::WorkoutSet>, Vec<crate::db::model::CardioLog>)> {
        self.workout.get_session_details(user_id, session_id).await
    }
}