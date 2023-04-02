//! file: gat.rs
//! author: Jacob Xie
//! date: 2023/04/02 00:44:54 Sunday
//! brief:

use anyhow::Result;
use std::future::Future;

pub trait FutMeta: Sized {
    // async function's return for self constructor
    type FutSelf<'a>: Future<Output = Result<Self>>
    where
        Self: 'a;

    fn new(path: &str) -> Self::FutSelf<'_>;

    // fn new_empty<'e>() -> Self::FutSelf<'e>;

    // async fn another_new(path: &str) -> Result<Self>;
}

struct FutExec(String);

impl FutMeta for FutExec {
    type FutSelf<'a> = impl Future<Output = Result<Self>> + 'a;

    fn new(path: &str) -> Self::FutSelf<'_> {
        async { Ok(Self(path.to_string())) }
    }

    // error: concrete type differs from previous defining opaque type use
    // fn new_empty<'e>() -> Self::FutSelf<'e> {
    // async { Ok(Self("".to_string())) }
    // }

    // async fn another_new(path: &str) -> Result<Self> {
    //     Ok(Self(path.to_string()))
    // }
}

// #[tokio::test]
// async fn fut_exec_test() {
//     let fe = FutExec::another_new("path").await;

//     assert!(fe.is_ok());
// }
