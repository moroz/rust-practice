use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CustomError(pub String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<CustomError> for std::io::Error {
    fn from(error: CustomError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotFound, error)
    }
}
