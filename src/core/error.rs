use std::fmt;

#[derive(Debug)]
pub enum NexaroError {
    Io(std::io::Error),
}

impl fmt::Display for NexaroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NexaroError::Io(error) => {
                write!(f, "Filesystem error: {}", error)
            }
        }
    }
}

impl std::error::Error for NexaroError {}

impl From<std::io::Error> for NexaroError {
    fn from(error: std::io::Error) -> Self {
        NexaroError::Io(error)
    }
}