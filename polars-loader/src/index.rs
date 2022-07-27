//! Indexing

use std::{any::Any, cell::RefCell, fmt::Debug, ops::Index};

use polars::prelude::*;
use ref_cast::RefCast;

trait MyValueTrait {
    fn dtype(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}

struct Null;

impl MyValueTrait for bool {
    fn dtype(&self) -> &'static str {
        "bool"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyValueTrait for u8 {
    fn dtype(&self) -> &'static str {
        "u8"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyValueTrait for i32 {
    fn dtype(&self) -> &'static str {
        "i32"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyValueTrait for Null {
    fn dtype(&self) -> &'static str {
        "null"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(RefCast)]
#[repr(transparent)]
struct MyValue(RefCell<Box<dyn MyValueTrait>>);

impl MyValue {
    fn dtype(&self) -> &'static str {
        self.0.borrow().dtype()
    }
}

impl AsRef<MyValue> for RefCell<Box<dyn MyValueTrait>> {
    fn as_ref(&self) -> &MyValue {
        MyValue::ref_cast(self)
    }
}

impl Debug for MyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_tuple("MyValue");

        match self.dtype() {
            "bool" => {
                let v = self.0.borrow();
                let v = v.as_any().downcast_ref::<bool>().unwrap();
                d.field(v);
            }
            _ => unimplemented!(),
        }

        d.finish()
    }
}

#[test]
fn my_value_ref_cast() {
    let v = RefCell::new(Box::new(true) as Box<dyn MyValueTrait>);

    let res = MyValue::ref_cast(&v);

    println!("{:?}", res.dtype());
}

struct MySeries(Series, RefCell<Box<dyn MyValueTrait>>);

#[allow(dead_code)]
impl MySeries {
    fn new(series: Series) -> Self {
        Self(series, RefCell::new(Box::new(Null)))
    }
}

impl Index<usize> for MySeries {
    type Output = MyValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self.0.dtype() {
            DataType::Boolean => {
                let res: Box<dyn MyValueTrait> = match self.0.bool().unwrap().get(index) {
                    Some(v) => Box::new(v),
                    None => Box::new(Null),
                };

                self.1.replace(res);

                MyValue::ref_cast(&self.1)
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

#[test]
fn my_series_index_success() {
    let s = Series::new("funk", [true, false, true, true]);

    let s = MySeries::new(s);

    let v = &s[1];

    println!("{:?}", v);
}
