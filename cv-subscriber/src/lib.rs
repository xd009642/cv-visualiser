use tracing_core::subscriber::Subscriber;
use tracing_subscriber::layer::Layer;

pub struct Server;

pub struct ImageLayer;

impl ImageLayer {
    pub fn new() -> (Self, Server) {
        todo!();
    }
}

impl<S> Layer<S> for ImageLayer
where
    S: Subscriber,
{
    // Implement the handlers we want
}
