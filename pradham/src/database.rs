use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

pub async fn establish_connection() -> DatabaseConnection {

   match Database::connect("sqlite:fake_data.sqlite?mode=rwc").await {
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