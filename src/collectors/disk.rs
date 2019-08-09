use std::borrow::Cow;

use futures::StreamExt;
use heim::Result;

async fn partitions() -> Result<()> {
    let mut partitions = heim::disk::partitions();

    while let Some(part) = partitions.next().await {
        let part = part?;
        let usage = match heim::disk::usage(part.mount_point()).await {
            Ok(usage) => usage,
            Err(..) => continue,
        };
        let device: Cow<'static, str> = match part.device() {
            Some(device) => device.to_string_lossy().to_string().into(),
            None => "".into(),
        };
        let mount = format!("{}", part.mount_point().display());
        let fs_type = part.file_system().as_str().to_string();

        metrics::gauge!("node_filesystem_total_bytes", usage.total().get() as i64,
            "device" => device.clone(),
            "fstype" => fs_type.clone(),
            "mountpoint" => mount.clone(),
        );
        metrics::gauge!("node_filesystem_free_bytes", usage.free().get() as i64,
            "device" => device,
            "fstype" => fs_type,
            "mountpoint" => mount,
        );
    }

    Ok(())
}

async fn io_counters() -> Result<()> {
    let mut counters = heim::disk::io_counters_physical();

    while let Some(counter) = counters.next().await {
        let counter = counter?;
        let device = counter.device_name().to_string_lossy().to_string();

        metrics::counter!("node_disk_reads_completed_total", counter.read_count(),
            "device" => device.clone());
        metrics::counter!("node_disk_read_bytes_total", counter.read_bytes().get(),
            "device" => device.clone());
        metrics::counter!("node_disk_writes_completed_total", counter.write_count(),
            "device" => device.clone());
        metrics::counter!("node_disk_writes_bytes_total", counter.write_bytes().get(),
            "device" => device);
    }

    Ok(())
}

pub async fn collect() -> Result<()> {
    let _ = futures::try_join!(partitions(), io_counters(),)?;

    Ok(())
}
