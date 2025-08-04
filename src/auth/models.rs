use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Subject {
    GuestUser(i32),
    PersistentUser(i32),
    Admin(i32),
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    Guest,
    Admin,
    Registered,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    pub guest_id: Uuid,
    auth0_id: Option<String>,
    pub user_type: UserType,
    pub last_active: DateTime<Utc>,
    name: Option<String>,
    email: Option<String>,
    age: Option<u8>,
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
}
