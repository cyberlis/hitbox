use actix::dev::{MessageResponse, ToEnvelope};
use actix::{Actor, Addr, Handler, Message};
use log::warn;
use serde::de::DeserializeOwned;
use serde::Serialize;

use hitbox::response::CacheableResponse;
use hitbox::runtime::{AdapterResult, RuntimeAdapter, EvictionPolicy, TtlSettings};
use hitbox::{CacheState, Cacheable, CachedValue};
use hitbox_backend::{Backend, Get, Set};

use crate::QueryCache;

pub struct ActixAdapter<A, M, B>
where
    A: Actor + Handler<M>,
    M: Message + Cacheable + Send,
    M::Result: MessageResponse<A, M> + Send,
    B: Backend,
{
    message: QueryCache<A, M>,
    backend: Addr<B>,
}

impl<A, M, B> ActixAdapter<A, M, B>
where
    A: Actor + Handler<M>,
    M: Message + Cacheable + Send,
    M::Result: MessageResponse<A, M> + Send,
    B: Backend,
{
    pub fn new(message: QueryCache<A, M>, backend: Addr<B>) -> Self {
        Self { message, backend }
    }
}

impl<A, M, T, B, U> RuntimeAdapter for ActixAdapter<A, M, B>
where
    A: Actor + Handler<M>,
    A::Context: ToEnvelope<A, M>,
    M: Message<Result = T> + Cacheable + Send + Clone + 'static,
    M::Result: MessageResponse<A, M> + Send,
    B: Backend,
    <B as Actor>::Context: ToEnvelope<B, Get> + ToEnvelope<B, Set>,
    T: CacheableResponse<Cached = U>,
    U: DeserializeOwned + Serialize,
{
    type UpstreamResult = T;

    fn poll_upstream(&self) -> AdapterResult<Self::UpstreamResult> {
        let message = self.message.message.clone();
        let upstream = self.message.upstream.clone();
        Box::pin(async move { Ok(upstream.send(message).await?) })
    }

    fn poll_cache(&self) -> AdapterResult<CacheState<Self::UpstreamResult>> {
        let backend = self.backend.clone();
        let cache_key = self.message.cache_key(); // @TODO: Please, don't recalculate cache key multiple times.
        Box::pin(async move {
            let key = cache_key?;
            let cached_value = backend.send(Get { key }).await??;
            CacheState::from_bytes(cached_value.as_ref())
        })
    }

    fn update_cache(&self, cached_value: &CachedValue<Self::UpstreamResult>) -> AdapterResult<()> {
        let serialized = cached_value.serialize();
        let ttl = self.message.message.cache_ttl();
        let backend = self.backend.clone();
        let cache_key = self.message.cache_key(); // @TODO: Please, don't recalculate cache key multiple times.
        Box::pin(async move {
            let serialized = serialized?;
            let key = cache_key?;
            backend
                .send(Set {
                    key,
                    value: serialized,
                    ttl: Some(ttl),
                })
                .await
                .map_err(|_| warn!("Updating Cache Error. Actix Mailbox Error."))
                .and_then(|value| {
                    value.map_err(|error| warn!("Updating Cache Error. {}", error.to_string()))
                });
            Ok(())
        })
    }
    fn eviction_settings(&self) -> EvictionPolicy {
        let ttl_settings = TtlSettings {
            ttl: self.message.message.cache_ttl(),
            stale_ttl: self.message.message.cache_stale_ttl(),
        };
        EvictionPolicy::Ttl(ttl_settings)
    }
}