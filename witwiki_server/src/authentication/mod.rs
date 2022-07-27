#[derive(Debug, Clone)]
pub enum Role {
    Admin,
    ReadOnly,
    UserDefined(String),
}

#[derive(Debug, Clone)]
pub struct User {
    roles: Option<Vec<Role>>,
    username: String,
}

#[derive(Debug, Clone)]
pub enum LoggedIn {
    In(User),
    Out,
}
