use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Subject {
    GuestUser(Uuid),
    RegisteredUser(String),
    Admin(String),
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
    age: Option<DateTime<Utc>>,
}

impl User {
    pub fn new_guest_user() -> Self {
        Self {
            id: 0,
            guest_id: Uuid::new_v4(),
            auth0_id: None,
            user_type: UserType::Guest,
            last_active: Utc::now(),
            name: None,
            email: None,
            age: None,
        }
    }

    pub fn new_registered_user(name: &str, email: Option<&str>) -> Self {
        todo!();
    }

    pub fn new_admin_user() -> Self {
        todo!();
    }

    pub fn strip_sensisive_data(&mut self) {
        self.auth0_id = None;
        self.name = None;
        self.email = None;
        self.age = None;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutUserRequest {
    name: Option<String>,
    email: Option<String>,
    age: Option<DateTime<Utc>>,
}
