use actix_web::{web, get, Responder, Result};

use crate::models;

#[get("")]
pub async fn get_users() -> Result<impl Responder> {
    
    let person_1 = models::User {
        id: 1,
        name: String::from("Qwerty"),
        username: String::from("Qwerty211"),
        gender: models::Gender::Male,
        email: String::from("qwerty@example.com")
    };

    let users: Vec<models::User> = vec![ person_1];

    Ok(web::Json(users))
}