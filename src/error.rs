use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum PacketError {
    OutOfBounds,
    InvalidPacketType,
    Utf8Error(FromUtf8Error),
    IoError(String),
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketError::OutOfBounds => write!(f, "Read out of bounds"),
            PacketError::InvalidPacketType => write!(f, "Invalid packet type"),
            PacketError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            PacketError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<FromUtf8Error> for PacketError {
    fn from(err: FromUtf8Error) -> Self {
        PacketError::Utf8Error(err)
    }
}