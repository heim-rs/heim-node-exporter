use futures::StreamExt;
use heim::Result;

async fn frequencies() -> Result<()> {
    let freq = heim::cpu::frequency().await?;

    metrics::gauge!(
        "node_cpu_scaling_frequency_hertz",
        freq.current().get() as i64
    );

    if let Some(value) = freq.min() {
        metrics::gauge!("node_cpu_scaling_frequency_min_hrts", value.get() as i64);
    }

    if let Some(value) = freq.max() {
        metrics::gauge!("node_cpu_scaling_frequency_max_hrts", value.get() as i64);
    }

    Ok(())
}

async fn stats() -> Result<()> {
    let stats = heim::cpu::stats().await?;

    metrics::counter!("node_cpu_statistics", stats.ctx_switches(),
        "key" => "ctx_switches");
    metrics::counter!("node_cpu_statistics", stats.interrupts(),
        "key" => "interrupts");

    Ok(())
}

async fn times() -> Result<()> {
    let mut times = heim::cpu::times();
    let mut idx = 0;

    while let Some(time) = times.next().await {
        let time = time?;
        let idx_repr = format!("{}", idx);

        metrics::counter!("node_cpu_seconds_total", time.user().get() as u64,
            "cpu" => idx_repr.clone(),
            "mode" => "user");
        metrics::counter!("node_cpu_seconds_total", time.system().get() as u64,
            "cpu" => idx_repr.clone(),
            "mode" => "system");
        metrics::counter!("node_cpu_seconds_total", time.idle().get() as u64,
            "cpu" => idx_repr,
            "mode" => "idle");

        idx += 1;
    }

    Ok(())
}

pub async fn collect() -> Result<()> {
    let _ = futures::try_join!(frequencies(), stats(), times(),)?;

    Ok(())
}
