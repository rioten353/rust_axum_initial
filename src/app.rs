use crate::routes;
use axum;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;

pub struct BasicServer;

impl BasicServer {
    pub async fn run() -> Result<(), std::io::Error> {
        let port: u16 = 3000;
        let app: axum::Router = routes::routes();
        let addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
        let listener: TcpListener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await
    }
}
