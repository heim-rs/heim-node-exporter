use heim::host;
use futures::prelude::*;

use crate::metrics::MetricBuilder;

pub async fn host(buffer: &mut bytes::BytesMut) {
    let platform = host::platform()
        .map_ok(|platform| {
            MetricBuilder::new(buffer)
                .name("host_platform")
                .label("system", platform.system())
                .label("release", platform.release())
                .label("version", platform.version())
                .label("architecture", platform.architecture().as_str())
                .value(1);
        });

    await!(platform).unwrap();

    let uptime = host::uptime()
        .map_ok(|uptime| {
            MetricBuilder::new(buffer)
                .name("host_uptime_seconds")
                .value(uptime.get());
        });

    await!(uptime).unwrap();

    let users = host::users()
        .try_fold(buffer, |buf, user| {
            MetricBuilder::new(buf)
                .name("host_user")
                .label("username", user.username())
                .value(1);

            future::ok(buf)
        });

    await!(users).unwrap();
}
