#![feature(async_await)]

use std::io;

use bytes::BytesMut;
use http_service::Body;
use tide::{App, Context, EndpointResult};

mod data;
mod metrics;

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

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.at("/metrics").get(collect);
    app.run("0.0.0.0:9101")?;

    Ok(())
}
