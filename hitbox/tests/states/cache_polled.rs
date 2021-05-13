use hitbox::states::cache_polled::CachePolledActual;
use hitbox::dev::MockAdapter;
use hitbox::{CachedValue, CacheError};
use hitbox::states::upstream_polled::{UpstreamPolled, UpstreamPolledSuccessful, UpstreamPolledError};

#[test]
fn test_cache_actual_debug() {
    let adapter = MockAdapter::build().with_upstream_value(42).finish();
    let actual = CachePolledActual { adapter, result: CachedValue::new(41, chrono::Utc::now()) };
    assert_eq!(format!("{:?}", actual), "CachePolledActual");
}

#[actix_rt::test]
async fn test_actual_poll_upstream_successful() {
    let adapter = MockAdapter::build().with_upstream_value(42).finish();
    let actual = CachePolledActual { adapter, result: CachedValue::new(0, chrono::Utc::now()) };
    let upstream_polled = actual.poll_upstream().await;
    let adapter = MockAdapter::build().with_upstream_value(42).finish();
    let expected = UpstreamPolled::Successful(UpstreamPolledSuccessful { adapter, result: 42 });
    assert!(matches!(upstream_polled, expected));
}

#[actix_rt::test]
async fn test_actual_poll_upstream_error() {
    let adapter: MockAdapter<i32> = MockAdapter::build().with_upstream_error().finish();
    let actual = CachePolledActual { adapter, result: CachedValue::new(0, chrono::Utc::now()) };
    let upstream_polled = actual.poll_upstream().await;
    let adapter: MockAdapter<i32> = MockAdapter::build().with_upstream_error().finish();
    let expected: UpstreamPolled<MockAdapter<i32>, i32> = UpstreamPolled::Error(
        UpstreamPolledError { error: CacheError::DeserializeError }
    );
    assert!(matches!(upstream_polled, expected));
}
