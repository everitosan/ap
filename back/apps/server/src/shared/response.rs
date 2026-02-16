use serde::Serialize;

/// Generic API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    /// Creates a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }

    /// Creates an error response with error message as data
    pub fn error(message: impl Into<String>) -> ApiResponse<String> {
        ApiResponse {
            success: false,
            data: message.into(),
        }
    }
}
