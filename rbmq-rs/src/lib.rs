//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/05/18 22:53:10 Thursday
//! brief: RabbitMQ rs

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{Channel, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use anyhow::Result;

macro_rules! aem {
    () => {
        ::anyhow::Error::msg
    };
}

/// Open a connection
pub async fn open_connection(host: &str, port: u16, user: &str, pass: &str) -> Result<Connection> {
    let arg = OpenConnectionArguments::new(host, port, user, pass);

    let conn = Connection::open(&arg).await.map_err(aem!())?;

    conn.register_callback(DefaultConnectionCallback)
        .await
        .map_err(aem!())?;

    Ok(conn)
}

/// Open a channel
pub async fn open_channel(conn: &Connection, channel_id: Option<u16>) -> Result<Channel> {
    let channel = conn.open_channel(channel_id).await.map_err(aem!())?;

    channel
        .register_callback(DefaultChannelCallback)
        .await
        .map_err(aem!())?;

    Ok(channel)
}

/// Declare a durable queue
/// Return: (queue_name, message_count, consumer_count)
pub async fn declare_queue(
    chan: &Channel,
    que: &str,
    rout: &str,
    exchg: &str,
) -> Result<(String, u32, u32)> {
    let res = chan
        .queue_declare(QueueDeclareArguments::durable_client_named(que))
        .await
        .map_err(aem!())?
        .unwrap();

    // bind que to an exchange
    // https://www.cloudamqp.com/blog/part4-rabbitmq-for-beginners-exchanges-routing-keys-bindings.html?gclid=Cj0KCQjwmZejBhC_ARIsAGhCqncGezxd7a25kEtBlBDr2r61_LkJODfMgykT2mP35TQ8eF0XrJ-NEKwaAvm1EALw_wcB
    chan.queue_bind(QueueBindArguments::new(que, exchg, rout))
        .await
        .map_err(aem!())?;

    Ok(res)
}
