#![cfg(test)]

use super::Proxy;

const MEGABYTE: usize = 1024 * 1024;
const MIN_SIZE: usize = 100 * MEGABYTE;

#[tokio::test]
#[allow(clippy::unwrap_used)]
#[allow(clippy::cast_precision_loss)]
async fn proxy() {
    // Create the proxy.
    let proxy = Proxy::new(
        "npmccallum",
        "test",
        1,
        vec!["githubusercontent.com".into()],
    )
    .unwrap();

    // Test that we get success.
    let response = proxy.get("latest", "test.efi").await.unwrap();
    assert!(!response.status().is_redirection());
    assert!(response.status().is_success());

    // Confirm that the file is large.
    let content_length = response.headers().get("content-length").unwrap();
    let length_str = content_length.to_str().unwrap();
    let length: usize = length_str.parse().unwrap();
    assert!(
        length > MIN_SIZE,
        "Expected body >100MB, got {} bytes ({:.2} MB)",
        length,
        length as f64 / (1024.0 * 1024.0)
    );
}
