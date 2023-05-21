//! file: simple_publisher.rs
//! author: Jacob Xie
//! date: 2023/05/20 23:29:47 Saturday
//! brief:

use amqprs::channel::BasicPublishArguments;
use amqprs::BasicProperties;
use rbmq_rs::*;
use tokio::time;

const HOST: &str = "localhost";
const PORT: u16 = 5672;
const USER: &str = "dev";
const PASS: &str = "devpass";
const VHOST: &str = "devhost";
const EXCHG: &str = "amq.direct";
const ROUT: &str = "rbmq-rs-rout";

#[tokio::main]
async fn main() {
    let conn = open_connection(HOST, PORT, USER, PASS, Some(VHOST))
        .await
        .unwrap();
    let chan = open_channel(&conn, None).await.unwrap();

    // publish msg
    let msg_bytes = String::from(
        r#"
            {
                "publisher": "example",
                "data": "Hello, amqprs!"
            }
        "#,
    )
    .into_bytes();

    // publish
    let args = BasicPublishArguments::new(EXCHG, ROUT);
    chan.basic_publish(BasicProperties::default(), msg_bytes, args)
        .await
        .unwrap();

    time::sleep(time::Duration::from_secs(1)).await;
    // explicitly close
    chan.close().await.unwrap();
    conn.close().await.unwrap();
}
