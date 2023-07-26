use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::spawn;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("internal error: {0}")]
    InternalError(#[from] tokio::io::Error),
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("internal error: {0}")]
    InternalError(#[from] tokio::io::Error),
}

async fn handle_client(mut socket: TcpStream, _addr: SocketAddr) -> Result<(), ClientError> {
    socket.flush().await?;
    socket.shutdown().await?;
    Ok(())
}

pub async fn start<A: ToSocketAddrs>(addr: A) -> Result<(), ServerError> {
    let listener = TcpListener::bind(addr).await?;
    log::info!("Listening on {}...", listener.local_addr()?);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                spawn(async move {
                    log::info!("Client {} connected", addr);
                    if let Err(e) = handle_client(socket, addr).await {
                        log::error!("Client {} errored: {}", addr, e);
                    } else {
                        log::info!("Client {} disconnected", addr);
                    }
                });
            }
            Err(e) => {
                log::error!("Client connection errored: {}", e);
            }
        }
    }
}