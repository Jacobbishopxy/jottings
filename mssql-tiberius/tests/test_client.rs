//! file: test_client.rs
//! author: Jacob Xie
//! date: 2023/09/15 14:43:30 Friday
//! brief:

use dotenv::dotenv;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::test]
async fn test_connection() -> anyhow::Result<()> {
    dotenv().ok();

    let conn = std::env::var("CONN")?;
    println!("CONN: {:?}", conn);

    let config = Config::from_jdbc_string(&conn)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stream = client.simple_query("select 1 as col").await?;
    let row = stream.into_row().await?;

    let res = row.unwrap().get::<i32, _>("col");
    println!("res: {:?}", res);

    Ok(())
}
