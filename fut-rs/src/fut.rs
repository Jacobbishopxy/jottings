//! file: fut.rs
//! author: Jacob Xie
//! date: 2023/03/29 19:04:17 Wednesday
//! brief:

use futures::stream::{Stream, StreamExt};
use rand::prelude::*;

// ================================================================================================
// Simple: stream unfold
// ================================================================================================

#[allow(dead_code)]
async fn get_events_from_page(_page_nb: usize) -> String {
    "done".to_string()
}

#[allow(dead_code)]
async fn stream_unfold() {
    let stream = futures::stream::unfold(0, |page_nb| async move {
        if page_nb > 50 {
            return None;
        }

        let events = get_events_from_page(page_nb).await;

        Some((events, page_nb + 1))
    });

    let n = Box::pin(stream).next().await;

    println!("{:?}", n);
}

// ================================================================================================
// Simple: producer & consumer
// ================================================================================================

#[allow(dead_code)]
async fn channel_producer_consumer() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        loop {
            let num = rand::thread_rng().gen_range(0..=101);

            tx.send(num).await.expect("yes");

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let num = rand::thread_rng().gen_range(0..=101);

            tx2.send(num).await.expect("yes");

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("GOT = {:?}", msg);
    }
}

// ================================================================================================
// Async State Machine
// ================================================================================================

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum State {
    Idle,
    Starting,
    Running,
    Failed,
    Stopped,
    Restarting,
}

#[allow(dead_code)]
async fn gen_state(event: i32) -> State {
    match event {
        (0..=10) => State::Idle,
        (11..=20) => State::Starting,
        (21..=70) => State::Running,
        (71..=99) => State::Failed,
        100 => State::Restarting,
        _ => State::Stopped,
    }
}

// tokio mpsc receiver
#[allow(dead_code)]
struct Consumer {
    rx: tokio::sync::mpsc::Receiver<i32>,
}

impl Consumer {
    #[allow(dead_code)]
    fn new(rx: tokio::sync::mpsc::Receiver<i32>) -> Self {
        Self { rx }
    }

    #[allow(dead_code)]
    fn as_stream(&mut self) -> impl Stream<Item = State> + '_ {
        futures::stream::unfold(self, |s| async {
            if let Some(e) = s.rx.recv().await {
                Some((gen_state(e).await, s))
            } else {
                None
            }
        })
    }
}

#[tokio::test]
async fn test_consumer() {
    let (tx, rx) = tokio::sync::mpsc::channel::<i32>(10);
    let tx2 = tx.clone();

    let mut consumer = Consumer::new(rx);
    let mut stream = consumer.as_stream().boxed();

    tokio::spawn(async move {
        loop {
            let num = rand::thread_rng().gen_range(0..=101);
            tx.send(num).await.expect("yes");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let num = rand::thread_rng().gen_range(0..=101);
            tx2.send(num).await.expect("yes");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });

    while let Some(s) = stream.next().await {
        println!("State: {:?}", s);
    }
}

// futures mpsc receiver
#[allow(dead_code)]
struct Consumer2 {
    rx: futures::channel::mpsc::Receiver<i32>,
}

impl Consumer2 {
    #[allow(dead_code)]
    fn new(rx: futures::channel::mpsc::Receiver<i32>) -> Self {
        Self { rx }
    }

    #[allow(dead_code)]
    fn as_stream(&mut self) -> impl Stream<Item = State> + '_ {
        futures::stream::unfold(self, |s| async {
            if let Some(e) = s.rx.next().await {
                dbg!(e);
                Some((gen_state(e).await, s))
            } else {
                dbg!("None");
                None
            }
        })
    }
}

#[tokio::test]
async fn test_consumer2() {
    let (mut tx, rx) = futures::channel::mpsc::channel::<i32>(10);
    let mut tx2 = tx.clone();

    let mut consumer = Consumer2::new(rx);
    let mut stream = consumer.as_stream().boxed();

    tokio::spawn(async move {
        loop {
            let num = rand::thread_rng().gen_range(0..=101);
            tx.try_send(num).expect("yes");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            let num = rand::thread_rng().gen_range(0..=101);
            tx2.try_send(num).expect("yes");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    while let Some(s) = stream.next().await {
        println!("State: {:?}", s);
    }
}
