use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("packet was not fully read, remaining bytes: {0}")]
    RemainingBytes(usize),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("unsupported type: {0}")]
    UnsupportedType(&'static str),
}