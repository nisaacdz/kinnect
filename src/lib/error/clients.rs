pub struct KinnectError {
    pub message: String,
    pub status: u16,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<axum::http::Body> {
        Json(self.message).into_response()
    }
}