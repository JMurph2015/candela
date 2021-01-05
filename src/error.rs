use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CandelaError {}

impl fmt::Display for CandelaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl Error for CandelaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _other => {
                return None;
            }
        }
    }
}
