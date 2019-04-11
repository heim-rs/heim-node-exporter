use std::io;
use std::ops;
use std::path;

use tokio::prelude::*;
use hyper::Chunk;
use bytes::{BytesMut, BufMut};

use crate::Tx;

#[derive(Debug, Default)]
pub struct Buffer(BytesMut);

impl Buffer {
    fn into_inner(self) -> BytesMut {
        self.0
    }
}

// https://github.com/carllerche/bytes/issues/77
impl io::Write for Buffer {
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

impl ops::Deref for Buffer {
    type Target = BytesMut;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MetricBuilder {
    bytes: Buffer,
}

impl MetricBuilder {
    pub fn new() -> MetricBuilder {
        Self {
            bytes: Buffer(BytesMut::with_capacity(256)),
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

    pub fn value<T: MetricValue>(mut self, value: T) -> Chunk {
        self.bytes.put(&b"} "[..]);
        value.put(&mut self.bytes);
        self.bytes.put(b'\n');

        Chunk::from(self.bytes.into_inner().freeze())
    }
}


pub trait IntoMetric {
    fn into_metric(self) -> Chunk;
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
        bytes.extend_from_slice(self.to_string_lossy().as_bytes());
    }
}

impl<'a> MetricValue for &'a path::Path {
    fn put(&self, bytes: &mut Buffer) {
        bytes.extend_from_slice(self.to_string_lossy().as_bytes())
    }
}

impl<T> MetricValue for Option<T> where T: MetricValue {
    fn put(&self, bytes: &mut Buffer) {
        if let Some(value) = self {
            value.put(bytes)
        }
    }
}

pub fn spawn_and_send<F, I>(f: F, tx: Tx)
    where
        F: Future<Item=I> + Send + 'static,
        I: Into<Chunk> + 'static {
    let f = f
        .map_err(|_| ())
        .map(Into::into)
        .and_then(|chunk| {
            tx.send(chunk).map_err(|_| ())
        })
        .map(|_| ())
        .map_err(|_| ());
    warp::spawn(f);
}

pub fn spawn_and_forward<S, I>(s: S, tx: Tx)
    where
        S: Stream<Item=I> + Send + 'static,
        I: Into<Chunk> + 'static {
    let f = s
        .map_err(|_| ())
        .map(Into::into)
        .and_then(move |chunk| {
            tx.clone().send(chunk).map_err(|_| ())
        })
        .for_each(|_| Ok(()));

    warp::spawn(f);
}