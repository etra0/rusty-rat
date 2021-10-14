
use crate::parsers::Page;

#[derive(Debug)]
pub enum NetworkError {
    StatusError(usize),
    Timeout,
    Unknown
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StatusError(v) => write!(f, "Status error: {}", v),
            Self::Timeout => write!(f, "Timeout"),
            Self::Unknown => write!(f, "Unknown Network Error")
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum InternalError {
    ParsingError(Page)
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingError(p) => write!(f, "Error parsing a link of the page {:?}", p),
        };
        Ok(())
    }
}

pub trait Error {}

/*
#[derive(Debug)]
pub enum Error {
    Internal(InternalError),
    Network(NetworkError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self);
        Ok(())
    }
}
*/


impl Error for NetworkError {}
impl Error for InternalError {}
