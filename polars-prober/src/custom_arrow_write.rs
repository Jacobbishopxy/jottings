//! Arrow write
//!
//! A custom serializer
//!
//! According to arrow2 source code, writing operation's code follow along with:
//! 1. `arrow::io::json::write::Serializer`, `arrow::ndjson::write::Serializer`: create a serializer
//! 1. `arrow::io::json::write::write`: write action, calling `blocks.next()`
//! 1. `FallibleStreamingIterator`: implement for serializer, customized `serialize` method
//! 1. `serialize`: `new_serializer(&dyn arrow::array::Array)` matching different arrow's DataType
//! 1. `boolean_serializer`/`primitive_serializer`/`float_serializer`/`utf8_serializer` ...
//!
//! Goal: custom serializer + custom FallibleStreamingIterator

// use arrow2::io::json::write::Serializer as jsonSerializer;
use fallible_streaming_iterator::FallibleStreamingIterator;
// use arrow2::io::ndjson::write::{Serializer as ndjsonSerializer};
// use arrow2::io::json::write::write;
use arrow2::error::Error;

#[allow(dead_code)]
pub fn write<W, I>(writer: &mut W, mut blocks: I) -> Result<(), Error>
where
    W: std::io::Write,
    I: FallibleStreamingIterator<Item = [u8], Error = Error>,
{
    writer.write_all(&[b'['])?;
    let mut is_first_row = true;
    while let Some(block) = blocks.next()? {
        if !is_first_row {
            writer.write_all(&[b','])?;
        }
        is_first_row = false;
        writer.write_all(block)?;
    }
    writer.write_all(&[b']'])?;
    Ok(())
}
