use actix_web::{delete, get, patch, post, put, web, Error, HttpResponse, Result};
use sea_orm::EntityTrait;
use serde_json::json;
use crate::models::{self, UserPatchData};
use entity::users::{Entity as UserEntity, Model as UserModel};

#[get("/")]
pub async fn get_users(data: web::Data<models::AppState>) -> Result<HttpResponse, Error> {
    
    let db_response = UserEntity::find().all(&data.db_connection).await;

    if let Some(users) = db_response.ok() {
        return Ok(
            HttpResponse::Ok().json(web::Json(users))
        );
    }

    Ok(
        HttpResponse::Ok().json(web::Json("[]"))
    )
}

#[get("/{user_id}")]
pub async fn get_user_by_id(data: web::Data<models::AppState>, path: web::Path<u8>) -> Result<HttpResponse, Error> {
    
    let user_id = path.into_inner();
    let db_response = UserEntity::find_by_id(user_id).one(&data.db_connection).await;

    if let Some(user) = db_response.ok().unwrap() {
        return Ok(
            HttpResponse::Ok().json(web::Json(user))
        );
    }

    Ok(
        HttpResponse::NotFound().body("User not found.")
    )
}

#[post("/")]
pub async fn post_user(new_user: web::Json<UserModel>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(web::Json(new_user)))
}

#[put("/{user_id}")]
pub async fn put_user(path: web::Path<u8>, new_user: web::Json<UserModel>) -> Result<HttpResponse, Error> {

    let user_id = path.into_inner();

    if user_id == new_user.id {
        return Ok(HttpResponse::Ok().json(web::Json(new_user)))
    }
    
    Ok(HttpResponse::BadRequest().body("id in payload json not match with path id"))
}

#[patch("/{user_id}")]
pub async fn patch_user(data: web::Data<models::AppState>, path: web::Path<u8>, patch_data: web::Json<UserPatchData>) -> Result<HttpResponse, Error> {

    let user_id = path.into_inner();
    let db_response = UserEntity::find_by_id(user_id).one(&data.db_connection).await;

    if let Some(mut user) = db_response.ok().unwrap() {

        if let Some(name) = &patch_data.name {
            user.name = name.to_string();
        }

        if let Some(username) = &patch_data.username {
            user.username = username.to_string();
        }

        if let Some(email) = &patch_data.email {
            user.email = email.to_string();
        }

        if let Some(gender) = &patch_data.gender {
            user.gender = gender.to_string();
        }

        return Ok(
            HttpResponse::Ok().json(web::Json(user))
        );
    }

    let mut patch_user = json!({});

    if let Some(name) = &patch_data.name {
        patch_user["name"] = serde_json::Value::String(name.to_string());
    }

    if let Some(username) = &patch_data.username {
        patch_user["username"] = serde_json::Value::String(username.to_string());
    }

    if let Some(email) = &patch_data.email {
        patch_user["email"] = serde_json::Value::String(email.to_string());
    }

    if let Some(gender) = &patch_data.gender {
        patch_user["gender"] = serde_json::Value::String(gender.to_string());
    }

    Ok(
        HttpResponse::Ok().json(web::Json(patch_user))
    )

}

#[delete("/{user_id}")]
pub async fn delete_user(_: web::Path<u8>) -> Result<HttpResponse, Error> {

    Ok(
        HttpResponse::Ok().body("User deleted successfully.")
    )
}