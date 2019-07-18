use bytes::BytesMut;
use futures::prelude::*;
use heim::memory;

use crate::metrics::MetricBuilder;

pub async fn memory(buffer: &mut BytesMut) {
    let memory = memory::memory().map_ok(|mem| {
        MetricBuilder::new(buffer)
            .name("memory_total_bytes")
            .value(mem.total().get());

        MetricBuilder::new(buffer)
            .name("memory_available_bytes")
            .value(mem.available().get());

        MetricBuilder::new(buffer)
            .name("memory_free_bytes")
            .value(mem.free().get());
    });

    memory.await.unwrap();

    let swap = memory::swap().map_ok(|swap| {
        MetricBuilder::new(buffer)
            .name("swap_total_bytes")
            .value(swap.total().get());
        MetricBuilder::new(buffer)
            .name("swap_used_bytes")
            .value(swap.used().get());
        MetricBuilder::new(buffer)
            .name("swap_free_bytes")
            .value(swap.free().get());
    });

    swap.await.unwrap();
}
