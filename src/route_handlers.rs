use actix_web::web;

mod users_handlers;

fn users_scope() -> actix_web::Scope {
    web::scope("/users")
    .service(users_handlers::get_users)
}

pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
    .service(users_scope())
}