//! IO-JSON
//!
//! https://jorgecarleitao.github.io/arrow2/io/json_write.html

use std::fs::File;

use arrow2::{array::Array, error::Error, io::json};

#[allow(dead_code)]
fn write_array(path: &str, array: Box<dyn Array>) -> Result<(), Error> {
    let mut writer = File::create(path)?;

    let arrays = vec![Ok(array)].into_iter();

    // Advancing this iterator serializes the next array to its internal buffer (i.e. CPU-bounded)
    let blocks = json::write::Serializer::new(arrays, vec![]);

    // the operation of writing is IO-bounded
    json::write::write(&mut writer, blocks)?;

    Ok(())
}

#[test]
fn write_success() -> Result<(), Error> {
    use arrow2::array::Int32Array;

    let file_path = "./cache/tmp.json";

    let array = Int32Array::from(&[Some(0), None, Some(2), Some(3), Some(4), Some(5), Some(6)]);

    write_array(file_path, Box::new(array))
}
