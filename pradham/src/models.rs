use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserPatchData {
    pub id: Option<u8>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub gender: Option<String>,
    pub email: Option<String>
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_connection: DatabaseConnection
}