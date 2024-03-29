//! file: test_client2.rs
//! author: Jacob Xie
//! date: 2023/09/16 15:54:50 Saturday
//! brief:

use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::test]
async fn test_connection() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("dev", "StrongPassword123"));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stream = client.simple_query("select 1 as col").await?;
    let row = stream.into_row().await?;

    let res = row.unwrap().get::<i32, _>("col");
    println!("res: {:?}", res);

    Ok(())
}
