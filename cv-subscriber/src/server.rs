use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::mpsc;

// Thoughts we want some sort of image interner so we can make it simple to register a single image
// and show where that image is used in operations. Otherwise for every op if we have to provide
// the inputs and outputs and transmit them again that is _pain_. But also, can we do this in a way
// that's actually nice for the user to use?
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
