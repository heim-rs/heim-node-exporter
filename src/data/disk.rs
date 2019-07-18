use bytes::BytesMut;
use futures::prelude::*;
use heim::disk;

use crate::metrics::MetricBuilder;

pub async fn disk(buffer: &mut BytesMut) {
    let partitions = disk::partitions()
        .and_then(|part| {
            // TODO: Get rid of the `to_path_buf` if it is possible
            disk::usage(part.mount_point().to_path_buf()).map_ok(|usage| (part, usage))
        })
        // Some filesystems might return an error for `disk::usage` (ex. Linux' debugfs),
        // skipping them silently
        .filter(|res| future::ready(res.is_ok()))
        .try_fold(buffer, |buf, (part, usage)| {
            MetricBuilder::new(buf)
                .name("disk_total_bytes")
                .label("device", part.device())
                .label("mount_point", part.mount_point())
                .label("file_system", part.file_system().as_str())
                .value(usage.total().get());

            MetricBuilder::new(buf)
                .name("disk_used_bytes")
                .label("device", part.device())
                .label("mount_point", part.mount_point())
                .label("file_system", part.file_system().as_str())
                .value(usage.used().get());

            MetricBuilder::new(buf)
                .name("disk_free_bytes")
                .label("device", part.device())
                .label("mount_point", part.mount_point())
                .label("file_system", part.file_system().as_str())
                .value(usage.free().get());

            future::ok(buf)
        });

    let buffer = partitions.await.unwrap();

    let io_counters = disk::io_counters().try_fold(buffer, |buf, io| {
        MetricBuilder::new(buf)
            .name("disk_io_read_count")
            .label("device", io.device_name())
            .value(io.read_count());

        MetricBuilder::new(buf)
            .name("disk_io_write_count")
            .label("device", io.device_name())
            .value(io.write_count());

        MetricBuilder::new(buf)
            .name("disk_io_read_bytes")
            .label("device", io.device_name())
            .value(io.read_bytes().get());

        MetricBuilder::new(buf)
            .name("disk_io_write_bytes")
            .label("device", io.device_name())
            .value(io.write_bytes().get());

        future::ok(buf)
    });

    io_counters.await.unwrap();
}
