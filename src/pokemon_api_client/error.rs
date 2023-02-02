use std::{fmt, error::Error};

/// Error for the uniting the different errors.
#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
    Deserialize(serde_json::Error),
    Io(std::io::Error),
    General(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            ApiError::Deserialize(err) => write!(f, "serde_json error: {}", err),
            ApiError::Io(err) => write!(f, "IO error: {}", err),
            ApiError::General(s) => write!(f, "General error: {}", s),
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiError::Reqwest(err) => Some(err),
            ApiError::Deserialize(err) => Some(err),
            ApiError::Io(err) => Some(err),
            ApiError::General(s) => None,
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
impl From<serde_json::error::Error> for ApiError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::Deserialize(e)
    }
}