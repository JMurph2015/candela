use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CandelaError {
    ProtoDecodeError(::prost::DecodeError),
    ZmqError(::zmq::Error),
    IoError(std::io::Error),
}

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

impl From<::prost::DecodeError> for CandelaError {
    fn from(e: ::prost::DecodeError) -> CandelaError {
        return CandelaError::ProtoDecodeError(e);
    }
}

impl From<zmq::Error> for CandelaError {
    fn from(e: zmq::Error) -> CandelaError {
        return CandelaError::ZmqError(e);
    }
}

impl From<std::io::Error> for CandelaError {
    fn from(e: std::io::Error) -> CandelaError {
        return CandelaError::IoError(e);
    }
}
