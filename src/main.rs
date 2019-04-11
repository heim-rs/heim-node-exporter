use warp::Filter;
use tokio::sync::mpsc;
use hyper::{Body, Chunk};

mod metrics;
mod data;

mod prelude {
    pub use super::metrics::{MetricBuilder, IntoMetric, spawn_and_send, spawn_and_forward};
    pub use super::Tx;
}

pub type Tx = mpsc::UnboundedSender<Chunk>;

fn collect() -> hyper::Response<Body> {
    let (tx, rx) = mpsc::unbounded_channel::<Chunk>();
    let body = Body::wrap_stream(rx);

    data::cpu::spawn_cpu(tx.clone());
    data::host::spawn_host(tx.clone());
    data::disk::spawn_disk(tx.clone());
    data::memory::spawn_memory(tx.clone());
    data::net::spawn_net(tx.clone());

    hyper::Response::new(body)
}

fn main() {
    let routes = warp::any().map(collect);

    warp::serve(routes).run(([0, 0, 0, 0], 9101));
}
