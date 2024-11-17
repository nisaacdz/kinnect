use axum::{response::Response, http::StatusCode, response::IntoResponse};

pub struct ClientError {
    pub message: String,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.message.clone()).into_response()
    }
}