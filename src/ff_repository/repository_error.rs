use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum RepositoryError {
    FileNotFound,

    // wtf is trait object
    JSONDecoding,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl Error for RepositoryError {}
