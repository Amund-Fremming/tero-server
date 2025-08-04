#[derive(Debug)]
pub enum Subject {
    GuestUser(String),
    PersistentUser(String),
    Admin(String),
}
