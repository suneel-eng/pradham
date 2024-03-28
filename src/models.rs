use serde::Serialize;

#[derive(Serialize)]
pub enum Gender {
    Male,
    Female
}

#[derive(Serialize)]
pub struct User {
    pub id: u8,
    pub name: String,
    pub username: String,
    pub gender: Gender,
    pub email: String
}