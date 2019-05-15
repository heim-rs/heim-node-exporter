use std::io;
use std::ops;
use std::path;
use std::ffi;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use bytes::{BytesMut, BufMut};

//use crate::Tx;

#[derive(Debug)]
pub struct Buffer<'b>(&'b mut BytesMut);

// https://github.com/carllerche/bytes/issues/77
impl<'b> io::Write for Buffer<'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len();
        self.0.reserve(len);
        self.0.extend_from_slice(buf);

        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'b> ops::Deref for Buffer<'b> {
    type Target = BytesMut;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'b> ops::DerefMut for Buffer<'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MetricBuilder<'b> {
    bytes: Buffer<'b>,
}

impl<'b> MetricBuilder<'b> {
    pub fn new(bytes: &'b mut BytesMut) -> MetricBuilder<'b> {
        Self {
            bytes: Buffer(bytes),
        }
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.bytes.extend_from_slice(name.as_bytes());
        self.bytes.put(b'{');
        self
    }

    pub fn label<T: MetricValue>(mut self, name: &'static str, value: T) -> Self {
        self.bytes.extend_from_slice(name.as_bytes());
        self.bytes.put(&b"=\""[..]);
        value.put(&mut self.bytes);
        self.bytes.put(&b"\","[..]);
        self
    }

    pub fn value<T: MetricValue>(mut self, value: T) {
        self.bytes.put(&b"} "[..]);
        value.put(&mut self.bytes);
        self.bytes.put(b'\n');
    }
}


pub trait MetricValue {
    fn put(&self, bytes: &mut Buffer);
}

impl<'s> MetricValue for &'s str {
    fn put(&self, bytes: &mut Buffer) {
        bytes.extend_from_slice(self.as_bytes())
    }
}

impl MetricValue for String {
    fn put(&self, bytes: &mut Buffer) {
        bytes.extend_from_slice(self.as_bytes())
    }
}

impl<'s> MetricValue for &'s ffi::OsStr {
    fn put(&self, bytes: &mut Buffer) {
        bytes.extend_from_slice(self.as_bytes())
    }
}

impl MetricValue for f64 {
    fn put(&self, bytes: &mut Buffer) {
        dtoa::write(bytes, *self).unwrap();
    }
}

impl MetricValue for i32 {
    fn put(&self, bytes: &mut Buffer) {
        itoa::write(bytes, *self).unwrap();
    }
}

impl MetricValue for u64 {
    fn put(&self, bytes: &mut Buffer) {
        itoa::write(bytes, *self).unwrap();
    }
}

impl MetricValue for usize {
    fn put(&self, bytes: &mut Buffer) {
        itoa::write(bytes, *self).unwrap();
    }
}

impl MetricValue for path::Path {
    fn put(&self, bytes: &mut Buffer) {
        #[cfg(unix)]
        bytes.extend_from_slice(self.as_os_str().as_bytes());

        #[cfg(not(unix))]
        bytes.extend_from_slice(self.to_string_lossy().as_bytes());
    }
}

impl<'a> MetricValue for &'a path::Path {
    fn put(&self, bytes: &mut Buffer) {
        #[cfg(unix)]
        bytes.extend_from_slice(self.as_os_str().as_bytes());

        #[cfg(not(unix))]
        bytes.extend_from_slice(self.to_string_lossy().as_bytes());
    }
}

impl<T> MetricValue for Option<T> where T: MetricValue {
    fn put(&self, bytes: &mut Buffer) {
        if let Some(value) = self {
            value.put(bytes)
        }
    }
}
