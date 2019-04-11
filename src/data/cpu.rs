use tokio::prelude::*;
use heim::cpu;
use heim::cpu::units::{second, hertz};

use crate::prelude::*;

pub fn spawn_cpu(tx: Tx) {
    let frequencies = cpu::frequency()
        .map(|freq| {
            let current = freq.current().get::<hertz>();
            let min = freq.min().map(|v| v.get::<hertz>());
            let max = freq.max().map(|v| v.get::<hertz>());
            stream::iter_ok(vec![
                MetricBuilder::new().name("cpu_frequency_hertz").value(current),
                MetricBuilder::new().name("cpu_frequency_max_hertz").value(min),
                MetricBuilder::new().name("cpu_frequency_max_hertz").value(max),
            ])
        })
        .flatten_stream();

    let stats = cpu::stats()
        .map(|stat| {
            stream::iter_ok(vec![
                MetricBuilder::new().name("cpu_stats_ctx_switches").value(stat.ctx_switches()),
                MetricBuilder::new().name("cpu_stats_interrupts").value(stat.interrupts()),
                MetricBuilder::new().name("cpu_stats_soft_interrupts").value(stat.soft_interrupts()),
            ])
        })
        .flatten_stream();

    let time = cpu::time()
        .map(|time| {
            let user = time.user().get::<second>();
            let system = time.system().get::<second>();
            let idle = time.idle().get::<second>();
            stream::iter_ok(vec![
                MetricBuilder::new().name("cpu_time_user").value(user),
                MetricBuilder::new().name("cpu_time_system").value(system),
                MetricBuilder::new().name("cpu_time_idle").value(idle),
            ])
        })
        .flatten_stream();

    let times = cpu::times()
        .enumerate()
        .map(|(idx, time)| {
            let user = time.user().get::<second>();
            let system = time.system().get::<second>();
            let idle = time.idle().get::<second>();
            stream::iter_ok::<_, heim::Error>(vec![
                MetricBuilder::new().name("cpu_time_user").label("cpu", idx).value(user),
                MetricBuilder::new().name("cpu_time_system").label("cpu", idx).value(system),
                MetricBuilder::new().name("cpu_time_idle").label("cpu", idx).value(idle),
            ])
        })
        .flatten();

    spawn_and_forward(frequencies, tx.clone());
    spawn_and_forward(stats, tx.clone());
    spawn_and_forward(time, tx.clone());
    spawn_and_forward(times, tx.clone());
}