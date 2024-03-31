use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files as fs;
use env_logger::Env;

mod route_handlers;
pub mod models;
pub mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    match dotenvy::dotenv() {
        Ok(_) => {
            println!("Environment variables loaded successfully...");
        },
        Err(env_err) => {
            println!("Environment loading error: {}", env_err.to_string())
        }
    }

    let db_connection = database::establish_connection().await;

    let app_state = models::AppState {
        db_connection
    };

    let app = move || {
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(app_state.clone()))
        .service(route_handlers::api_scope())
        .service(
            fs::Files::new("/guide", "./pradham/static").index_file("guide.html").prefer_utf8(true)
        )
        .service(
            fs::Files::new("/", "./pradham/static").index_file("index.html").prefer_utf8(true)
        )
    };

    let server = HttpServer::new(app);

    match server.bind(("127.0.0.1", 3000)) {
        Ok(server) => {
            println!("Server started listening on 127.0.0.1:3000 ...");
            server.run().await
        },
        Err(err) => Err(err)
    }
}