use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::mpsc;

pub struct ImageServer {
    incoming: mpsc::Receiver<()>,
}

impl ImageServer {
    pub const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
    pub const DEFAULT_PORT: u16 = 1804;

    pub async fn serve(&self) {
        todo!()
    }
}
