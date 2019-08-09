use heim::Result;

async fn uptime() -> Result<()> {
    let uptime = heim::host::uptime().await?;

    metrics::counter!("node_uptime_seconds", uptime.get() as u64);

    Ok(())
}

async fn platform() -> Result<()> {
    let platform = heim::host::platform().await?;

    let system = platform.system().to_string();
    let release = platform.release().to_string();
    let version = platform.version().to_string();
    let hostname = platform.hostname().to_string();
    let arch = platform.architecture().as_str();

    metrics::counter!("node_platform_info", 0,
        "system" => system,
        "release" => release,
        "version" => version,
        "hostname" => hostname,
        "arch" => arch,
    );

    Ok(())
}

pub async fn collect() -> Result<()> {
    let _ = futures::try_join!(uptime(), platform(),)?;

    Ok(())
}
