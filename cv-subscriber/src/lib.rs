use crate::server::*;
use crate::visitor::ImageVisitor;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::thread;
use tokio::runtime;
use tokio::sync::mpsc;
use tracing_core::{span, subscriber::Subscriber, Event};
use tracing_subscriber::{layer::Context, prelude::*, registry::LookupSpan, Layer};

pub mod server;
pub mod utils;
pub mod visitor;

#[derive(Hash, Clone)]
pub struct Image;

/// So thoughts on interning. I could make an interner here or just make it on the server side and
/// send out images as soon as I get them _out of laziness_. I should do something though
pub type ImageInterner = HashSet<Image>;

pub struct Builder {
    address: SocketAddr,
}

pub struct ImageLayer {
    tx: mpsc::Sender<ImageVisitor>,
    images: HashMap<Option<span::Id>, ImageInterner>,
}

impl Builder {
    #[must_use = "a `Layer` must be added to a `tracing::Subscriber` in order to be used"]
    pub fn spawn<S>(self) -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let (layer, server) = ImageLayer::new();

        thread::Builder::new()
            .name("cv-visualiser".into())
            .spawn(move || {
                let runtime = runtime::Builder::new_current_thread()
                    .enable_io()
                    .enable_time()
                    .build()
                    .expect("failed to start cv-visualiser tokio runtime");

                runtime.block_on(async move {
                    server.serve().await;
                });
            })
            .expect("unable to spawn cv-visualiser server thread");

        layer
    }
}

impl ImageLayer {
    pub fn new() -> (Self, ImageServer) {
        todo!();
    }

    fn parent_context<S>(
        &self,
        attrs: &span::Attributes<'_>,
        ctx: &Context<'_, S>,
    ) -> Option<span::Id>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        attrs.parent().cloned().or_else(|| {
            if attrs.is_contextual() {
                ctx.lookup_current().map(|span| span.id())
            } else {
                None
            }
        })
    }

    fn publish_images(&self, visitor: ImageVisitor) {
        use mpsc::error::TrySendError;
        match self.tx.try_reserve() {
            Ok(permit) => {
                // Send images to server using our permit
                permit.send(visitor);
            }
            Err(TrySendError::Closed(_)) => {}
            Err(TrySendError::Full(_)) => {}
        }
        // Force a flush if we're filling up
        let _capacity = self.tx.capacity();
    }
}

impl<S> Layer<S> for ImageLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let root = self
            .parent_context(attrs, &ctx)
            .unwrap_or_else(|| id.clone());
        let mut visitor = ImageVisitor::new(Some(root));
        attrs.record(&mut visitor);
        self.publish_images(visitor);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let id = ctx.lookup_current().map(|x| x.id());
        let mut visitor = ImageVisitor::new(id);
        event.record(&mut visitor);
        self.publish_images(visitor);
    }

    fn on_enter(&self, id: &span::Id, cx: Context<'_, S>) {}

    fn on_exit(&self, id: &span::Id, cx: Context<'_, S>) {}

    fn on_close(&self, id: span::Id, cx: Context<'_, S>) {}
}
