//! Series custom iter

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
            AnyValue::String(v) => MyValue::String(v.to_owned()),
            AnyValue::UInt8(v) => MyValue::Integer(v.into()),
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
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
    Str(StringChunked, usize, usize),
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
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test_iter {

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
