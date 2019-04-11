use tokio::prelude::*;
use heim::host;
use heim::host::units::{Time};
use heim::cpu::units::{second};

use crate::prelude::*;

impl IntoMetric for host::Platform {
    fn into_metric(self) -> hyper::Chunk {
        MetricBuilder::new()
            .name("host_platform")
            .label("system", self.system())
            .label("release", self.release())
            .label("version", self.version())
            .label("architecture", self.architecture().as_str())
            .value(1)
    }
}

pub struct Uptime(Time);

impl IntoMetric for Uptime {
    fn into_metric(self) -> hyper::Chunk {
        MetricBuilder::new()
            .name("host_uptime_seconds")
            .value(self.0.get::<second>())
    }
}

impl IntoMetric for host::User {
    fn into_metric(self) -> hyper::Chunk {
        MetricBuilder::new()
            .name("host_user")
            .label("username", self.username())
            .label("terminal", self.terminal().unwrap_or(""))
            .value(1)
    }
}

pub fn spawn_host(tx: Tx) {
    spawn_and_send(host::platform().map(|platform| platform.into_metric()), tx.clone());
    spawn_and_send(host::uptime().map(Uptime).map(|uptime| uptime.into_metric()), tx.clone());
    spawn_and_forward(host::users().map(|user| user.into_metric()), tx);
}