use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use std::env;

pub async fn establish_connection() -> DatabaseConnection {

    let db_connection_string = env::var("DATABASE_URL").ok().unwrap();

   match Database::connect(db_connection_string).await {
    Ok(connection) => {
        println!("Database connection successful...");

        match Migrator::up(&connection, None).await {
            Ok(_) => {
                println!("Migration successful...");
                connection
            },
            Err(error) => {
                panic!("Migration failed {}", error.to_string())
            }
        }
    },
    Err(error) => panic!("{}", error)
   }

}