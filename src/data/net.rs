use tokio::prelude::*;
use heim::net;
use heim::disk::units::byte;

use crate::prelude::*;

pub fn spawn_net(tx: Tx) {
    let counters = net::io_counters()
        .map(|io| {
            stream::iter_ok::<_, heim::Error>(vec![
                MetricBuilder::new().name("net_io_bytes_sent")
                    .label("device", io.interface())
                    .value(io.bytes_sent().get::<byte>()),
                MetricBuilder::new().name("net_io_bytes_recv")
                    .label("device", io.interface())
                    .value(io.bytes_recv().get::<byte>()),
                MetricBuilder::new().name("net_io_packets_sent")
                    .label("device", io.interface())
                    .value(io.packets_sent()),
                MetricBuilder::new().name("net_io_packets_recv")
                    .label("device", io.interface())
                    .value(io.packets_recv()),
                MetricBuilder::new().name("net_io_errors_sent")
                    .label("device", io.interface())
                    .value(io.errors_sent()),
                MetricBuilder::new().name("net_io_errors_recv")
                    .label("device", io.interface())
                    .value(io.errors_recv()),
                MetricBuilder::new().name("net_io_drop_sent")
                    .label("device", io.interface())
                    .value(io.drop_sent()),
                MetricBuilder::new().name("net_io_drop_recv")
                    .label("device", io.interface())
                    .value(io.drop_recv()),
            ])
        })
        .flatten();

    let nic = net::nic()
        .map(|nic| {
            stream::iter_ok::<_, heim::Error>(vec![
                MetricBuilder::new()
                    .name("net_nic_up")
                    .label("device", nic.name())
                    .value({
                        if nic.is_up() { 1 } else { 0 }
                    }),
            ])
        })
        .flatten();

    spawn_and_forward(counters, tx.clone());
    spawn_and_forward(nic, tx);
}