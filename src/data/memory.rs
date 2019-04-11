use tokio::prelude::*;
use heim::memory;
use heim::memory::units::byte;

use crate::prelude::*;

pub fn spawn_memory(tx: Tx) {
    let memory = memory::memory()
        .map(|mem| {
            let total = mem.total().get::<byte>();
            let available = mem.available().get::<byte>();
            let free = mem.free().get::<byte>();
            stream::iter_ok(vec![
                MetricBuilder::new().name("memory_total_bytes").value(total),
                MetricBuilder::new().name("memory_available_bytes").value(available),
                MetricBuilder::new().name("memory_free_bytes").value(free),
            ])
        })
        .flatten_stream();
    let swap = memory::swap()
        .map(|swap| {
            let total = swap.total().get::<byte>();
            let used = swap.used().get::<byte>();
            let free = swap.free().get::<byte>();
            stream::iter_ok(vec![
                MetricBuilder::new().name("swap_total_bytes").value(total),
                MetricBuilder::new().name("swap_used_bytes").value(used),
                MetricBuilder::new().name("swap_free_bytes").value(free),
            ])
        })
        .flatten_stream();

    spawn_and_forward(memory, tx.clone());
    spawn_and_forward(swap, tx.clone());
}