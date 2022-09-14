//! GAT Async Trait
//!
//! https://zhuanlan.zhihu.com/p/463367405

use std::future::Future;
use std::io::Write;

pub trait KvIterator {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;

    fn next(&mut self) -> Self::NextFuture<'_>;
}

pub struct TestIterator {
    idx: usize,
    to_idx: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

#[allow(dead_code)]
impl TestIterator {
    pub fn new(from_idx: usize, to_idx: usize) -> Self {
        Self {
            idx: from_idx,
            to_idx,
            key: Vec::new(),
            value: Vec::new(),
        }
    }
}

impl KvIterator for TestIterator {
    type NextFuture<'a> = impl Future<Output = Option<(&'a [u8], &'a [u8])>>;

    fn next(&mut self) -> Self::NextFuture<'_> {
        async move {
            if self.idx >= self.to_idx {
                return None;
            }

            self.key.clear();
            write!(&mut self.key, "key_{:05}", self.idx).unwrap();

            self.value.clear();
            write!(&mut self.value, "value_{:05}", self.idx).unwrap();

            self.idx += 1;
            Some((&self.key[..], &self.value[..]))
        }
    }
}

#[tokio::test]
async fn test_kv_iterator() {
    use bytes::Bytes;

    let mut tt = TestIterator::new(5, 10);

    while let Some((k, v)) = tt.next().await {
        println!(
            "{:?} {:?}",
            Bytes::copy_from_slice(k),
            Bytes::copy_from_slice(v)
        );
    }
}
