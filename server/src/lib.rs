use thiserror::Error;
use tokio::net::{TcpListener, ToSocketAddrs};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("internal error: {0}")]
    InternalError(tokio::io::Error),

}

async fn start<A: ToSocketAddrs>(addr: A) -> Result<(), ServerError> {
    let listener = TcpListener::bind(addr).await.map_err(|e| ServerError::InternalError(e))?;

    Ok(())
}