use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct User{
    pub id: i32,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
