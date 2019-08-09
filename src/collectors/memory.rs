use heim::Result;

async fn memory() -> Result<()> {
    let mem = heim::memory::memory().await?;

    metrics::gauge!("node_memory", mem.total().get() as i64, "key" => "total");
    metrics::gauge!("node_memory", mem.available().get() as i64, "key" => "available");
    metrics::gauge!("node_memory", mem.free().get() as i64, "key" => "free");

    Ok(())
}

async fn swap() -> Result<()> {
    let swap = heim::memory::swap().await?;

    metrics::gauge!("node_swap", swap.total().get() as i64, "key" => "total");
    metrics::gauge!("node_swap", swap.used().get() as i64, "key" => "used");
    metrics::gauge!("node_swap", swap.free().get() as i64, "key" => "free");

    Ok(())
}

pub async fn collect() -> Result<()> {
    let _ = futures::try_join!(memory(), swap(),)?;

    Ok(())
}
