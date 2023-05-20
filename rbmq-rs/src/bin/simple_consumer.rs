//! file: simple_consumer.rs
//! author: Jacob Xie
//! date: 2023/05/20 21:01:38 Saturday
//! brief:

use amqprs::channel::BasicConsumeArguments;
use amqprs::consumer::DefaultConsumer;
use rbmq_rs::*;
use tokio::sync::Notify;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

const HOST: &str = "localhost";
const PORT: u16 = 5672;
const USER: &str = "dev";
const PASS: &str = "devpass";
const VHOST: &str = "devhost";
// amq.direct/amq.fanout/amq.headers/amq.match/amq.rabbitmq.trace/amq.topic
const EXCHG: &str = "amq.direct";
const QUE: &str = "rbmq-rs-test";
const ROUT: &str = "rbmq-rs-rout";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        // .with(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .unwrap();

    let conn = open_connection(HOST, PORT, USER, PASS, Some(VHOST))
        .await
        .unwrap();
    let chan = open_channel(&conn, None).await.unwrap();

    let (que, _, _) = declare_queue(&chan, QUE, ROUT, EXCHG).await.unwrap();

    let consumer_tag = "rbmq-rs";

    // start consumer
    let args = BasicConsumeArguments::new(&que, consumer_tag)
        .manual_ack(false)
        .finish();

    // impl amqprs::consumer::AsyncConsumer for our CustomConsumer
    let ct = chan
        .basic_consume(DefaultConsumer::new(args.no_ack), args)
        .await
        .unwrap();
    println!("consumer tag: {:?}", ct);

    // consume forever
    println!("consume forever..., ctrl+c to exit");
    let guard = Notify::new();
    guard.notified().await;
}
