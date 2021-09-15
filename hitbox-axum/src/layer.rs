use tower_service::Service;
use std::task::{Poll, Context};
use tower_layer::Layer;
use std::fmt;

/// Applies a timeout to requests via the supplied inner service.
#[derive(Debug, Clone)]
pub struct CacheLayer {}

impl CacheLayer {
    /// Create a timeout from a duration
    pub fn new() -> Self {
        CacheLayer {}
    }
}

impl<S> Layer<S> for CacheLayer {
    type Service = CacheService<S>;

    fn layer(&self, service: S) -> Self::Service {
        CacheService { service }
    }
}

// This service implements the Log behavior
pub struct CacheService<S> {
    service: S,
}

impl<S, Request> Service<Request> for CacheService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}", request);
        self.service.call(request)
    }
}
