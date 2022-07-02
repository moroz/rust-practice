use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum APIErrorType {
    Unauthorized,
    RequestError,
    Other,
}

#[derive(Debug, Clone)]
pub struct APIError {
    pub kind: APIErrorType,
    pub message: Option<String>,
}

impl From<APIError> for std::io::Error {
    fn from(error: APIError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
}

impl Error for APIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "APIError: {:?}, reason: {}",
            &self.kind,
            self.message
                .as_ref()
                .unwrap_or(&"Unknown error".to_string())
        )
    }
}
