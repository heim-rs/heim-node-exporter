use heim::Result;

mod cpu;
mod disk;
mod host;
mod memory;

pub async fn collect() -> Result<()> {
    futures::try_join!(
        cpu::collect(),
        disk::collect(),
        host::collect(),
        memory::collect(),
    )
    .map(|_| ())
}
