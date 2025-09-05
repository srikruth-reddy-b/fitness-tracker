use serde::Deserialize;
use serde::Serialize;

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub id: i32,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MuscleGroup{

}