//! Sidecar for [Kubernetes] probing.
//!
//! [Kubernetes]: https://kubernetes.io

#![deny(
    intra_doc_link_resolution_failure,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]
#![warn(
    deprecated_in_future,
    missing_copy_implementations,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results
)]

use std::convert::Infallible;

use warp::Filter as _;

#[tokio::main]
async fn main() {
    let routes = warp::any().and_then(check);

    warp::serve(routes).run(([0, 0, 0, 0], 10027)).await;
}

async fn check() -> Result<impl warp::Reply, Infallible> {
    Ok(format!(
        "Hello, Mr. Prober!\n\n{:?}",
        probe_rust_lang().await,
    ))
}

async fn probe_rust_lang() -> HttpProbeResult {
    reqwest::get("https://www.rust-lang.org")
        .await
        .map(|r| HttpProbeResult::Finished(r.status()))
        .unwrap_or_else(HttpProbeResult::Incomplete)
}

#[derive(Debug)]
enum HttpProbeResult {
    Incomplete(reqwest::Error),
    Finished(reqwest::StatusCode),
}
