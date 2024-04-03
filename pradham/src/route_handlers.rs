use actix_files as fs;
use actix_web::web;

mod pages_handlers;
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
    web::scope("/api").service(users_scope())
}

pub fn pages_scope() -> actix_web::Scope {
    web::scope("").service(pages_handlers::home_page).service(
        fs::Files::new("/guide", "./pradham/templates")
            .index_file("guide.html")
            .prefer_utf8(true),
    )
    .service(
        fs::Files::new("/assets", "./pradham/assets")
            .show_files_listing()
            .prefer_utf8(true)
    )
}
