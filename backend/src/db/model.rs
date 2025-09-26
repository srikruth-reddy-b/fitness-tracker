use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::fittrack::users;
// #[derive(Debug,Serialize,Deserialize)]
// pub struct User{
//     // pub id: i32,
//     pub fullname: String,
//     pub username: String,
//     pub email: String,
//     pub password: String,
// }

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub fullname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MuscleGroup{
    pub id: i32,
    pub name: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Set{
    pub id: i32,
    pub muscle_group_id: i32,
    pub variation_id: i32,
    pub name: String,
    pub repetitions: i32,
    pub date: String, //REVIEW,
    pub weight: f32,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Variation{
    pub id: i32,
    pub muscle_group_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Cardio{
    id: i32,
    pub name: String,
    pub duration: i32, // REVIEW,
    pub type_: String,
    pub date: String, // REVIEW,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Schedule{
    pub id: i32,
    pub muscle_group_id: i32,
    pub variation_id: i32,
    pub date: String, //REVIEW
}