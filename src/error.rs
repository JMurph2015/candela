use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CandelaError {
    ProtoDecodeError(::prost::DecodeError),
    IoError(std::io::Error),
    #[cfg(any(feature = "zmq-client", feature = "zmq-server"))]
    ZmqError(::zmq::Error),
}

impl fmt::Display for CandelaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl Error for CandelaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<::prost::DecodeError> for CandelaError {
    fn from(e: ::prost::DecodeError) -> CandelaError {
        CandelaError::ProtoDecodeError(e)
    }
}

impl From<std::io::Error> for CandelaError {
    fn from(e: std::io::Error) -> CandelaError {
        CandelaError::IoError(e)
    }
}

#[cfg(any(feature = "zmq-client", feature = "zmq-server"))]
impl From<zmq::Error> for CandelaError {
    fn from(e: zmq::Error) -> CandelaError {
        CandelaError::ZmqError(e)
    }
}
