
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
            Self::StatusError(v) => write!(f, "Status error: {}", v)?,
            Self::Timeout => write!(f, "Timeout")?,
            Self::Unknown => write!(f, "Unknown Network Error")?
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum InternalError {
    ParsingError(Page),
    TimeoutLimit
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingError(p) => write!(f, "Error parsing a link of the page {:?}", p)?,
            Self::TimeoutLimit => write!(f, "Timeout limit reached")?,
        };
        Ok(())
    }
}

pub trait Error : std::error::Error {}

impl std::error::Error for NetworkError {}
impl std::error::Error for InternalError {}
impl Error for NetworkError {}
impl Error for InternalError {}

impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
    fn from(e: E) -> Self {
        Box::new(e)
    }
}
