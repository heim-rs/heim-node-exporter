use tokio::prelude::*;
use heim::disk;
use heim::disk::units::byte;

use crate::prelude::*;

pub fn spawn_disk(tx: Tx) {
    let partitions = disk::partitions()
        .and_then(|part| {
            disk::usage(part.mount_point().to_path_buf())
                .map(|usage| Ok((part, usage)))
        })
        .buffer_unordered(5)
        .map(|(part, usage)| {
            stream::iter_ok::<_, heim::Error>(vec![
                MetricBuilder::new().name("disk_total_bytes")
                    .label("device", part.device())
                    .label("mount_point", part.mount_point())
                    .label("file_system", part.file_system().as_str())
                    .value(usage.total().get::<byte>()),
                MetricBuilder::new().name("disk_used_bytes")
                    .label("device", part.device())
                    .label("mount_point", part.mount_point())
                    .label("file_system", part.file_system().as_str())
                    .value(usage.used().get::<byte>()),
                MetricBuilder::new().name("disk_free_bytes")
                    .label("device", part.device())
                    .label("mount_point", part.mount_point())
                    .label("file_system", part.file_system().as_str())
                    .value(usage.free().get::<byte>()),
            ])
        })
        .flatten();

    let io_counters = disk::io_counters()
        .map(|io| {
            stream::iter_ok::<_, heim::Error>(vec![
                MetricBuilder::new().name("disk_io_read_count")
                    .label("device", io.device_name())
                    .value(io.read_count()),
                MetricBuilder::new().name("disk_io_write_count")
                    .label("device", io.device_name())
                    .value(io.write_count()),
                MetricBuilder::new().name("disk_io_read_bytes")
                    .label("device", io.device_name())
                    .value(io.read_bytes().get::<byte>()),
                MetricBuilder::new().name("disk_io_write_bytes")
                    .label("device", io.device_name())
                    .value(io.write_bytes().get::<byte>()),
            ])
        })
        .flatten();

    spawn_and_forward(partitions, tx.clone());
    spawn_and_forward(io_counters, tx.clone());
}