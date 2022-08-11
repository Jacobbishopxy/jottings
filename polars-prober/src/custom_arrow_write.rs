//! Arrow write
//!
//! A custom serializer
//!
//! According to arrow2 source code, writing operation's code follow along with:
//! 1. `arrow::io::json::Serializer`, `arrow::ndjson::write::Serializer`: create a serializer
//! 1. `arrow::io::json::write::write`: write action, calling `blocks.next()`
//! 1. `FallibleStreamingIterator`: implement for serializer, customized `serialize` method
//! 1. `serialize`: `new_serializer(&dyn arrow::array::Array)` matching different arrow's DataType
//! 1. `boolean_serializer`/`primitive_serializer`/`float_serializer`/`utf8_serializer` ...
//!
//! Goal: custom serializer + custom FallibleStreamingIterator

// TODO
