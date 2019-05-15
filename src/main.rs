#![feature(async_await, futures_api, await_macro)]

use tide::{App, Context, EndpointResult};
use bytes::BytesMut;
use http_service::Body;

mod metrics;
mod data;

async fn collect(_cx: Context<()>) -> EndpointResult<http::Response<Body>> {
    let mut buffer = BytesMut::with_capacity(16_384);

    self::data::cpu::cpu(&mut buffer).await;
    self::data::host::host(&mut buffer).await;
    self::data::disk::disk(&mut buffer).await;
    self::data::memory::memory(&mut buffer).await;

    let resp = http::Response::builder()
        .status(http::status::StatusCode::OK)
        .body(Body::from(buffer))
        .unwrap();
    Ok(resp)
}

fn main() {
    let mut app = App::new(());
    app.at("/metrics").get(collect);
    app.serve("0.0.0.0:9101").unwrap();
}
