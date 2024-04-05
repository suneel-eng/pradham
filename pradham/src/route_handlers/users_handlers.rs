use crate::models::{self, AppState, UserPatchData};
use actix_web::{delete, get, patch, post, put, web, Error, HttpResponse, Result};
use entity::{served_requests_count, users};
use sea_orm::{EntityTrait, QueryFilter, Set};
use serde_json::json;
use std::env;

async fn update_users_api_requests_count(data: web::Data<models::AppState>) {
    let shuttle = env::var("SHUTTLE").ok();

    if let None = shuttle {
        return ();
    }

    if shuttle.unwrap() == "true" {
        use sea_orm::ColumnTrait;

        let db = &data.db_connection;

        let db_response = served_requests_count::Entity::find()
            .filter(served_requests_count::Column::ApiName.eq("users"))
            .one(db)
            .await;

        let users_api_read = db_response.ok().unwrap().unwrap();

        use sea_orm::ActiveModelTrait;

        let db_response = served_requests_count::Entity::find()
            .filter(served_requests_count::Column::ApiName.eq("users"))
            .one(db)
            .await;

        let mut users_api_write: served_requests_count::ActiveModel =
            db_response.ok().unwrap().unwrap().into();

        users_api_write.requests_count = Set(users_api_read.requests_count + 1);

        let _ = users_api_write.update(db).await;
    }
}

#[get("/")]
pub async fn get_users(data: web::Data<models::AppState>) -> Result<HttpResponse, Error> {
    let db_response = users::Entity::find().all(&data.db_connection).await;

    let response = if let Some(users) = db_response.ok() {
        Ok(HttpResponse::Ok().json(web::Json(users)))
    } else {
        Ok(HttpResponse::Ok().json(web::Json("[]")))
    };

    update_users_api_requests_count(data).await;

    response
}

#[get("/{user_id}")]
pub async fn get_user_by_id(
    data: web::Data<models::AppState>,
    path: web::Path<u8>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let db_response = users::Entity::find_by_id(user_id)
        .one(&data.db_connection)
        .await;

    let response = if let Some(user) = db_response.ok().unwrap() {
        Ok(HttpResponse::Ok().json(web::Json(user)))
    } else {
        Ok(HttpResponse::NotFound().body("User not found."))
    };

    update_users_api_requests_count(data).await;

    response
}

#[post("/")]
pub async fn post_user(
    data: web::Data<AppState>,
    new_user: web::Json<users::Model>,
) -> Result<HttpResponse, Error> {
    update_users_api_requests_count(data).await;
    Ok(HttpResponse::Ok().json(web::Json(new_user)))
}

#[put("/{user_id}")]
pub async fn put_user(
    data: web::Data<AppState>,
    path: web::Path<u8>,
    new_user: web::Json<users::Model>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let response = if user_id == new_user.id {
        Ok(HttpResponse::Ok().json(web::Json(new_user)))
    } else {
        Ok(HttpResponse::BadRequest().body("id in payload json not match with path id"))
    };

    update_users_api_requests_count(data).await;

    response
}

#[patch("/{user_id}")]
pub async fn patch_user(
    data: web::Data<models::AppState>,
    path: web::Path<u8>,
    patch_data: web::Json<UserPatchData>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let db_response = users::Entity::find_by_id(user_id)
        .one(&data.db_connection)
        .await;

    let response = if let Some(mut user) = db_response.ok().unwrap() {
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

        Ok(HttpResponse::Ok().json(web::Json(user)))
    } else {
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

        Ok(HttpResponse::Ok().json(web::Json(patch_user)))
    };

    update_users_api_requests_count(data).await;

    response
}

#[delete("/{user_id}")]
pub async fn delete_user(
    data: web::Data<AppState>,
    _: web::Path<u8>,
) -> Result<HttpResponse, Error> {
    update_users_api_requests_count(data).await;
    Ok(HttpResponse::Ok().body("User deleted successfully."))
}
