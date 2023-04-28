//! file: main.rs
//! author: Jacob Xie
//! date: 2023/04/28 09:44:15 Friday
//! brief:

use std::fs::File;

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::error::Result;
use arrow2::io::ipc::read;

#[allow(clippy::type_complexity)]
fn read_chunks(path: &str) -> Result<(Schema, Vec<Chunk<Box<dyn Array>>>)> {
    let mut file = File::open(path)?;

    // 读取文件 metadata。
    let metadata = read::read_file_metadata(&mut file)?;

    let schema = metadata.schema.clone();

    // 最简单的方法：使用 reader 并遍历 batches
    let reader = read::FileReader::new(file, metadata, None, None);

    let chunks = reader.collect::<Result<Vec<_>>>()?;

    Ok((schema, chunks))
}

// 随机访问的方法：从文件中读取单个 record batch。
#[allow(dead_code)]
fn read_batch(path: &str, chunk_index: usize) -> Result<(Schema, Chunk<Box<dyn Array>>)> {
    let mut file = File::open(path)?;

    let metadata = read::read_file_metadata(&mut file)?;

    let schema = metadata.schema.clone();

    let dictionaries = read::read_file_dictionaries(&mut file, &metadata, &mut Default::default())?;

    let chunk = read::read_batch(
        &mut file,
        &dictionaries,
        &metadata,
        None,
        None,
        chunk_index,
        &mut Default::default(),
        &mut Default::default(),
    )?;

    Ok((schema, chunk))
}

// cd project root
// ./arrow-ipc/ipc-rs/target/debug/ipc-rs
fn main() -> Result<()> {
    const FILENAME: &str = "dev.ipc";

    let (schema, chunks) = read_chunks(FILENAME)?;

    println!("{:?}", schema);
    println!("{:?}", chunks);

    Ok(())
}
