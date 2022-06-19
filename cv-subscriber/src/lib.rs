use crate::server::*;
use crate::visitor::ImageVisitor;
use std::net::SocketAddr;
use std::thread;
use tokio::runtime;
use tokio::sync::mpsc;
use tracing_core::{span, subscriber::Subscriber, Event};
use tracing_subscriber::{layer::Context, prelude::*, registry::LookupSpan, Layer};

pub mod server;
pub mod utils;
pub mod visitor;

pub struct Builder {
    address: SocketAddr,
}

pub struct ImageLayer {
    tx: mpsc::Sender<()>,
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
}

impl<S> Layer<S> for ImageLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let root = self
            .parent_context(attrs, &ctx)
            .unwrap_or_else(|| id.clone());
        let mut visitor = ImageVisitor::default();
        attrs.record(&mut visitor);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {}

    fn on_enter(&self, id: &span::Id, cx: Context<'_, S>) {}

    fn on_exit(&self, id: &span::Id, cx: Context<'_, S>) {}

    fn on_close(&self, id: span::Id, cx: Context<'_, S>) {}
}
