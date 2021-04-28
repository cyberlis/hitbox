use hitbox::settings::{CacheSettings, Status};
use hitbox::dev::MockAdapter;
use hitbox::states::initial::InitialState;
use hitbox::states::upstream_polled::UpstreamPolled;
use hitbox::settings::InitialCacheSettings;
use hitbox::CacheError;

#[actix_rt::test]
async fn test_cache_disabled_upstream_polled() {
    let settings = CacheSettings {
        cache: Status::Disabled,
        stale: Status::Disabled,
        lock: Status::Disabled,
    };
    let adapter = MockAdapter::build()
        .with_upstream_value(42)
        .finish();
    let initial_state = InitialCacheSettings::from(settings);
    let initial_state = InitialState { adapter, settings: initial_state };
    let result = match initial_state.poll_upstream().await {
        UpstreamPolled::Successful(state) => Ok(state.finish().result()),
        UpstreamPolled::Error(error) => Err(error.finish().result()),
    };
    assert_eq!(result.unwrap(), 42);
}

#[actix_rt::test]
async fn test_cache_disabled_upstream_error() {
    let settings = CacheSettings {
        cache: Status::Disabled,
        stale: Status::Disabled,
        lock: Status::Disabled,
    };
    let adapter = MockAdapter::build()
        .with_upstream_value(42)
        .with_upstream_error()
        .finish();
    let initial_state = InitialCacheSettings::from(settings);
    let initial_state = InitialState { adapter, settings: initial_state };
    let result = match initial_state.poll_upstream().await {
        UpstreamPolled::Successful(state) => Ok(state.finish().result()),
        UpstreamPolled::Error(error) => Err(error.finish().result()),
    };
    assert!(result.is_err());
}
