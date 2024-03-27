use actix_web::{App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app = || {
        App::new()
        .service(
            fs::Files::new("/guide", "./static").index_file("guide.html").prefer_utf8(true)
        )
        .service(
            fs::Files::new("/", "./static").index_file("index.html").prefer_utf8(true)
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