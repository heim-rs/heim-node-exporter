use heim::cpu;
use futures::prelude::*;

use crate::metrics::MetricBuilder;

pub async fn cpu(buffer: &mut bytes::BytesMut) {
    let frequencies = cpu::frequency()
        .map_ok(|freq| {
            let current = freq.current().get();
            let min = freq.min().map(|v| v.get());
            let max = freq.max().map(|v| v.get());

            MetricBuilder::new(buffer)
                .name("cpu_frequency_hertz")
                .value(current);

            MetricBuilder::new(buffer)
                .name("cpu_frequency_max_hertz")
                .value(min);

            MetricBuilder::new(buffer)
                .name("cpu_frequency_max_hertz")
                .value(max);
        });

    await!(frequencies).unwrap();

    let stats = cpu::stats()
        .map_ok(|stat| {
            MetricBuilder::new(buffer)
                .name("cpu_stats_ctx_switches")
                .value(stat.ctx_switches());

            MetricBuilder::new(buffer)
                .name("cpu_stats_interrupts")
                .value(stat.interrupts());

            MetricBuilder::new(buffer)
                .name("cpu_stats_soft_interrupts")
                .value(stat.soft_interrupts());
        });

    await!(stats).unwrap();

    let time = cpu::time()
        .map_ok(|time| {
            let user = time.user().get();
            let system = time.system().get();
            let idle = time.idle().get();

            MetricBuilder::new(buffer).name("cpu_time_user").value(user);
            MetricBuilder::new(buffer).name("cpu_time_system").value(system);
            MetricBuilder::new(buffer).name("cpu_time_idle").value(idle);
        });

    await!(time).unwrap();

    let times = cpu::times()
        .enumerate()
        .fold(buffer, |buf, (idx, try_time)| {
            let buf = match try_time {
                Ok(time) => {
                    let user = time.user().get();
                    let system = time.system().get();
                    let idle = time.idle().get();

                    MetricBuilder::new(buf)
                        .name("cpu_time_user")
                        .label("cpu", idx)
                        .value(user);

                    MetricBuilder::new(buf)
                        .name("cpu_time_system")
                        .label("cpu", idx)
                        .value(system);

                    MetricBuilder::new(buf)
                        .name("cpu_time_idle")
                        .label("cpu", idx)
                        .value(idle);

                    buf
                },
                Err(_) => buf,
            };

            future::ready(buf)
        });

    await!(times);
}