use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Subject {
    Guest(Uuid),
    Registered(String),
    Admin(String),
    Auth0,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth0User {
    auth0_id: String,
    given_name: Option<String>,
    family_name: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    phone: Option<String>,
    phone_verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    #[serde(rename(deserialize = "guest"))]
    Guest,
    #[serde(rename(deserialize = "admin"))]
    Admin,
    #[serde(rename(deserialize = "registered"))]
    Registered,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    pub guest_id: Uuid,
    auth0_id: Option<String>,
    pub user_type: UserType,
    pub last_active: DateTime<Utc>,
    name: Option<String>,
    email: Option<String>,
    birth_date: Option<NaiveDate>,
}

impl User {
    pub fn strip_sensisive_data(&mut self) {
        self.auth0_id = None;
        self.name = None;
        self.email = None;
        self.birth_date = None;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub birth_date: Option<NaiveDate>,
}
