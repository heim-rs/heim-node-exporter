#![feature(async_await)]

use metrics_core::{Builder, Drain, Observe};
use metrics_runtime::{observers::PrometheusBuilder, Controller, Receiver};

mod collectors;

pub struct State {
    controller: Controller,
}

async fn metrics(ctx: tide::Context<State>) -> tide::EndpointResult {
    match collectors::collect().await {
        Ok(..) => {
            let mut observer = PrometheusBuilder::new().build();
            ctx.state().controller.observe(&mut observer);

            Ok(tide::Response::new(tide::Body::from(observer.drain())))
        }
        Err(e) => {
            let body = tide::Body::from(format!("{}", e));
            let mut resp = tide::Response::new(body);
            *resp.status_mut() = tide::http::StatusCode::INTERNAL_SERVER_ERROR;

            Ok(resp)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let receiver = Receiver::builder().build()?;

    let mut app = tide::App::with_state(State {
        controller: receiver.get_controller(),
    });
    app.at("/metrics").get(metrics);
    receiver.install();

    app.run(("0.0.0.0", 9101))?;

    Ok(())
}
