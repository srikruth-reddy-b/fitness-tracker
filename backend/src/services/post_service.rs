use std::sync::Arc;
use log::{error, info};
use serde::Serialize;
use crate::{api::workouts::{ CardioSet, CreateCardioExerciseRequest, CreateMuscleGroupRequest, CreateVariationRequest, StrengthSet, WorkoutSession}, 
            db::{logger::LoggerDB, 
                model::{NewCardioExercise, NewCardioLog, NewMuscleGroup, NewVariation, NewWorkoutSession, NewWorkoutSet}}};

#[derive(Debug, Serialize)]
pub struct PostResponse{
    pub user_id: i32,
    pub id: Option<i32>,
    pub success: bool,
    pub message: String,
}

pub struct PostService{
    logger: Arc<LoggerDB>
}

impl PostService{
    pub fn new(logger: Arc<LoggerDB>) -> Self{
        PostService{
            logger
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
        match self.logger.add_workout_session(workout_session).await{
            Ok(session) => {
                info!("Workout session added with ID: {}", session.id);
                PostResponse { 
                    user_id: session.user_id,
                    id: Some(session.id), 
                    success: true, 
                    message: "Session Added".to_string()
                }
            },
            Err(err) => {
                error!("Error adding workout session: {}", err);
                PostResponse{
                    user_id: 1, 
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
        if let Err(err) = self.logger.add_workout_set(workout_session).await{
            info!("Error adding workout set: {}", err);
            return PostResponse{
                user_id: session_request.user_id,
                id: None,
                success: false,
                message: format!("{}",err)
            }
        };

        info!("Workout set added for user ID: {}", session_request.user_id);
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
        if let Err(err) = self.logger.add_workout_cardio(workout_session).await{
            error!("Error adding cardio log: {}", err);
            return PostResponse{
                user_id: session_request.user_id,
                id: None,
                success: false,
                message: format!("{}",err)
            };
        }

        info!("Cardio log added for user ID: {}", session_request.user_id);
        PostResponse { 
            user_id: session_request.user_id, 
            id: None,
            success: true, 
            message: "Cardio Log Added".to_string()
        }
    }

    pub async fn add_muscle_group(&self, user_id: i32, request: CreateMuscleGroupRequest) -> PostResponse {
        let new_mg = NewMuscleGroup {
            name: &request.name,
            user_id,
        };
        match self.logger.add_muscle_group(new_mg).await {
            Ok(mg) => {
                info!("Adding muscle group for user_id: {}", user_id);
                PostResponse {
                    user_id,
                    id: Some(mg.id),
                    success: true,
                    message: "Muscle Group Added".to_string()
                }
            },
            Err(err) => {
                error!("Error adding muscle group for user_id {}: {}", user_id, err);
                PostResponse {
                    user_id,
                    id: None,
                    success: false,
                    message: format!("{}", err)
                }
            }
        }
    }

    pub async fn add_variation(&self, user_id: i32, request: CreateVariationRequest) -> PostResponse {
        let new_var = NewVariation {
            muscle_group_id: request.muscle_group_id,
            name: &request.name,
            user_id,
            description: None, // Or add to request if needed
        };
        match self.logger.add_variation(new_var).await {
            Ok(var) => {
                info!("Variation added for user_id: {}", user_id);
                PostResponse {
                    user_id,
                    id: Some(var.id),
                    success: true,
                    message: "Variation Added".to_string()
                }
            },
            Err(err) => {
                error!("Error adding variation for user_id {}: {}", user_id, err);
                PostResponse {
                    user_id,
                    id: None,
                    success: false,
                    message: format!("{}", err)
                }
            }
        }
    }

    pub async fn add_cardio_exercise(&self, user_id: i32, request: CreateCardioExerciseRequest) -> PostResponse {
        let new_ex = NewCardioExercise {
            name: &request.name,
            user_id,
        };
        match self.logger.add_cardio_exercise(new_ex).await {
            Ok(ex) => {
                info!("Cardio exercise added for user_id: {}", user_id);
                PostResponse {
                    user_id,
                    id: Some(ex.id),
                    success: true,
                    message: "Cardio Exercise Added".to_string()
                }
            },
            Err(err) => {
                error!("Error adding cardio exercise for user_id {}: {}", user_id, err);
                PostResponse {
                    user_id,
                    id: None,
                    success: false,
                    message: format!("{}", err)
                }
            }
        }
    }
}