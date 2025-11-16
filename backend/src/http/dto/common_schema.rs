use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success<S: Into<String>>(data: Option<T>, message: Option<S>) -> Self {
        ApiResponse {
            success: true,
            data,
            message: message.map(|s| s.into()),
            error: None }
    }
}

impl ApiResponse<()> {
    pub fn failed<S: Into<String>>(error: Option<S>) -> Self {
        ApiResponse {
            success: false,
            data: None,
            message: None,
            error: error.map(|s| s.into())
        }
    }
}
