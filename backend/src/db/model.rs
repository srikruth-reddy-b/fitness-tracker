use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::fittrack::{users, muscle_groups, variations, sets, cardio_exercises, cardio_logs, workout_sessions};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub dob: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub fullname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub dob: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a>{
    pub fullname: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub dob: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = muscle_groups)]
pub struct MuscleGroup {
    pub id: i32,
    pub name: String,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(MuscleGroup))]
#[diesel(table_name = variations)]
pub struct Variation {
    pub id: i32,
    pub muscle_group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations,Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = workout_sessions)]
pub struct WorkoutSession {
    pub id: i32,
    pub user_id: i32,
    pub title: Option<String>,
    pub date: chrono::NaiveDate,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = workout_sessions)]
pub struct NewWorkoutSession {
    pub user_id: i32,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub date: chrono::NaiveDate,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = workout_sessions)]
pub struct UpdateWorkoutSession {
    pub title: Option<String>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Variation))]
#[diesel(belongs_to(WorkoutSession))]
#[diesel(table_name = sets)]
pub struct WorkoutSet {
    pub id: i32,
    pub workout_session_id: Option<i32>,
    pub user_id: i32,
    pub variation_id: i32,
    pub weight: f64,
    pub reps: i32,
    pub performed_on: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = sets)]
pub struct NewWorkoutSet {
    pub workout_session_id: Option<i32>,
    pub user_id: i32,
    pub variation_id: i32,
    pub weight: f64,
    pub reps: i32,
    pub performed_on: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = sets)]
pub struct UpdateWorkoutSet {
    pub weight: Option<f64>,
    pub reps: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = cardio_exercises)]
pub struct CardioExercise {
    pub id: i32,
    pub name: String,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CardioExercise))]
#[diesel(belongs_to(WorkoutSession))]
#[diesel(table_name = cardio_logs)]
pub struct CardioLog {
    pub id: i32,
    pub workout_session_id: Option<i32>,
    pub user_id: i32,
    pub cardio_exercise_id: i32,
    pub duration_minutes: i32,
    pub performed_on: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = cardio_logs)]
pub struct NewCardioLog {
    pub workout_session_id: Option<i32>,
    pub user_id: i32,
    pub cardio_exercise_id: i32,
    pub duration_minutes: i32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = cardio_logs)]
pub struct UpdateCardioLog {
    pub duration_minutes: Option<i32>,
}