//! A jotting lib used for testing polars crate and etc.

mod index;
mod unsafe_index;

use polars::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
enum MyValue {
    String(String),
    Bool(bool),
    Number(f64),
    Integer(i64),
    Null,
}

impl<'a> From<AnyValue<'a>> for MyValue {
    fn from(av: AnyValue<'a>) -> Self {
        match av {
            AnyValue::Boolean(v) => MyValue::Bool(v),
            AnyValue::Utf8(v) => MyValue::String(v.to_owned()),
            AnyValue::UInt8(v) => MyValue::Integer(v.into()),
            AnyValue::UInt16(_) => todo!(),
            AnyValue::UInt32(_) => todo!(),
            AnyValue::UInt64(_) => todo!(),
            AnyValue::Int8(_) => todo!(),
            AnyValue::Int16(_) => todo!(),
            AnyValue::Int32(_) => todo!(),
            AnyValue::Int64(_) => todo!(),
            AnyValue::Float32(_) => todo!(),
            AnyValue::Float64(_) => todo!(),
            _ => unimplemented!(),
        }
    }
}

struct MySeries {
    data: Series,
    dtype: DataType,
}

impl IntoIterator for MySeries {
    type Item = MyValue;
    type IntoIter = MySeriesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        match self.dtype {
            DataType::Boolean => {
                let arr = self.data.bool().unwrap();
                MySeriesIntoIterator::Bool(arr.clone(), arr.len(), 0)
            }
            DataType::UInt8 => todo!(),
            DataType::UInt16 => todo!(),
            DataType::UInt32 => todo!(),
            DataType::UInt64 => todo!(),
            DataType::Int8 => todo!(),
            DataType::Int16 => todo!(),
            DataType::Int32 => todo!(),
            DataType::Int64 => todo!(),
            DataType::Float32 => todo!(),
            DataType::Float64 => todo!(),
            DataType::Utf8 => todo!(),
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
enum MySeriesIntoIterator {
    Bool(BooleanChunked, usize, usize),
    I8(Int8Chunked, usize, usize),
    I16(Int16Chunked, usize, usize),
    I32(Int32Chunked, usize, usize),
    I64(Int64Chunked, usize, usize),
    U8(UInt8Chunked, usize, usize),
    U16(UInt16Chunked, usize, usize),
    U32(UInt32Chunked, usize, usize),
    U64(UInt64Chunked, usize, usize),
    F32(Float32Chunked, usize, usize),
    F64(Float64Chunked, usize, usize),
    Str(Utf8Chunked, usize, usize),
}

impl Iterator for MySeriesIntoIterator {
    type Item = MyValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MySeriesIntoIterator::Bool(arr, len, step) => {
                if len == step {
                    None
                } else {
                    let res = match arr.get(*step) {
                        Some(v) => MyValue::Bool(v),
                        None => MyValue::Null,
                    };
                    *step += 1;
                    Some(res)
                }
            }
            // MySeriesIntoIterator::I8(arr, len, step) => todo!(),
            // MySeriesIntoIterator::I16(arr, len, step) => todo!(),
            // MySeriesIntoIterator::I32(arr, len, step) => todo!(),
            // MySeriesIntoIterator::I64(arr, len, step) => todo!(),
            // MySeriesIntoIterator::U8(arr, len, step) => todo!(),
            // MySeriesIntoIterator::U16(arr, len, step) => todo!(),
            // MySeriesIntoIterator::U32(arr, len, step) => todo!(),
            // MySeriesIntoIterator::U64(arr, len, step) => todo!(),
            // MySeriesIntoIterator::F32(arr, len, step) => todo!(),
            // MySeriesIntoIterator::F64(arr, len, step) => todo!(),
            // MySeriesIntoIterator::Str(arr, len, step) => todo!(),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test_core {

    use polars::prelude::*;

    use super::*;

    #[test]
    fn test_into_iter() {
        let s = Series::new("dev", [true, false, false, true, false]);
        let s = MySeries {
            data: s,
            dtype: DataType::Boolean,
        };

        for i in s.into_iter() {
            println!("{:?}", i);
        }
    }

    #[test]
    fn test_lazy() {
        let df = df! {
            "column_a" => &[1, 2, 3, 4, 5],
            "column_b" => &["a", "b", "c", "d", "e"],
        }
        .unwrap();

        let new = df
            .lazy()
            .reverse()
            .with_column((col("column_a") * lit(10)).alias("new_column"))
            .collect()
            .unwrap();

        println!("{:?}", new);
    }
}
