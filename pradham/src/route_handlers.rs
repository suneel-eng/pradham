use actix_web::web;

mod users_handlers;

fn users_scope() -> actix_web::Scope {
    web::scope("/users")
    .service(users_handlers::get_users)
    .service(users_handlers::get_user_by_id)
    .service(users_handlers::post_user)
    .service(users_handlers::put_user)
    .service(users_handlers::patch_user)
    .service(users_handlers::delete_user)
}

pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
    .service(users_scope())
}