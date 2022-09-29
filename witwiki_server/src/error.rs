use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

enum WitErrorFatal {
    Db(sqlx::Error),
}
enum WitErrorHandled {
    E400(String, String),
    E409(String),
    E500(String, String),
}
pub enum WitError {
    Fatal(WitErrorFatal),
    Handled(WitErrorHandled),
}

pub fn e400(error_msg: &str, user_msg: &str) -> WitError {
    WitError::Handled(WitErrorHandled::E400(
        error_msg.to_owned(),
        user_msg.to_owned(),
    ))
}

pub fn e409(error_msg: &str) -> WitError {
    WitError::Handled(WitErrorHandled::E409(error_msg.to_owned()))
}

pub fn e500(error_msg: &str, user_msg: &str) -> WitError {
    WitError::Handled(WitErrorHandled::E500(
        error_msg.to_owned(),
        user_msg.to_owned(),
    ))
}

pub fn db_err(e: sqlx::Error) -> WitError {
    WitError::Fatal(WitErrorFatal::Db(e))
}
impl IntoResponse for WitError {
    fn into_response(self) -> Response {
        match self {
            WitError::Fatal(WitErrorFatal::Db(_)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong"),
            ),
            WitError::Handled(WitErrorHandled::E400(sysmsg, usrmsg)) => {
                (StatusCode::BAD_REQUEST, usrmsg)
            }
            WitError::Handled(WitErrorHandled::E409(_)) => {
                (StatusCode::UNAUTHORIZED, "unauthorized".to_owned())
            }
            WitError::Handled(WitErrorHandled::E500(_, usrmsg)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, usrmsg)
            }
        }
        .into_response()
    }
}
