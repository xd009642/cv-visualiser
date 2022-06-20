use crate::*;
use common::*;
use service::trace_service_client::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::mpsc;

mod common {
    tonic::include_proto!("cv.visualiser.common");
}
mod service {
    tonic::include_proto!("cv.visualiser.protocol");
}

// Thoughts we want some sort of image interner so we can make it simple to register a single image
// and show where that image is used in operations. Otherwise for every op if we have to provide
// the inputs and outputs and transmit them again that is _pain_. But also, can we do this in a way
// that's actually nice for the user to use?
pub struct ImageServer {
    incoming: mpsc::Receiver<SubscriberEvent>,
}

impl ImageServer {
    pub fn new(incoming: mpsc::Receiver<SubscriberEvent>) -> Self {
        Self { incoming }
    }

    pub async fn serve(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = TraceServiceClient::connect("127.0.0.1:1804").await?;
        while let Some(event) = self.incoming.recv().await {}
        Ok(())
    }
}
