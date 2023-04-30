//! file: main.rs
//! author: Jacob Xie
//! date: 2023/04/28 09:44:15 Friday
//! brief:

use arrow2::error::Result;

use ipc_rs::read_ipc::*;
// use ipc_rs::write_ipc::*;

// cd project root
// ./arrow-ipc/ipc-rs/target/debug/ipc-rs
fn main() -> Result<()> {
    const FILENAME: &str = "dev.ipc";

    let (schema, chunks) = read_chunks(FILENAME)?;

    println!("{:?}", schema);
    println!("{:?}", chunks);

    Ok(())
}
