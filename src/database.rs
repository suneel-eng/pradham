use rusqlite::Connection;

pub fn establish_connection() -> Connection {

    let database_url = String::from("./fake_data.db");

    match Connection::open(database_url) {
        Ok(connection) => {
            println!("Database connection successful...");
            connection
        },
        Err(connection_err) => {
            panic!("Error occured: {}", connection_err);
        }
    }
}