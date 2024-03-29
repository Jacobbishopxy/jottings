//! file: write_ipc.rs
//! author: Jacob Xie
//! date: 2023/04/28 16:51:19 Friday
//! brief:

use std::fs::File;
use std::net::TcpStream;

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::error::Result;
use arrow2::io::ipc::write;

pub fn write_batches(path: &str, schema: Schema, chunks: &[Chunk<Box<dyn Array>>]) -> Result<()> {
    let file = File::create(path)?;

    let options = write::WriteOptions { compression: None };
    let mut writer = write::FileWriter::new(file, schema, None, options);

    writer.start()?;
    for chunk in chunks {
        writer.write(chunk, None)?;
    }
    writer.finish()
}

// Streaming write
pub fn write_stream(addr: &str, schema: Schema, chunks: &[Chunk<Box<dyn Array>>]) -> Result<()> {
    let mut writer = TcpStream::connect(addr)?;
    let mut stream =
        write::StreamWriter::new(&mut writer, write::WriteOptions { compression: None });

    stream.start(&schema, None)?;
    for chk in chunks {
        stream.write(chk, None)?;
    }
    stream.finish()?;

    Ok(())
}
