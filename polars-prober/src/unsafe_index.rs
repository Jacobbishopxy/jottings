//! Test

use std::{fmt::Debug, ops::Index};

use polars::prelude::*;

trait MyValueTrait: Debug {
    fn dtype(&self) -> &'static str;
}

impl MyValueTrait for bool {
    fn dtype(&self) -> &'static str {
        "bool"
    }
}

#[derive(Debug)]
struct Null;

impl MyValueTrait for Null {
    fn dtype(&self) -> &'static str {
        "null"
    }
}

#[derive(Debug)]
struct MyValue(Box<dyn MyValueTrait>);

impl AsRef<MyValue> for Box<dyn MyValueTrait> {
    fn as_ref(&self) -> &MyValue {
        unsafe { std::mem::transmute(self) }
    }
}

#[test]
fn my_value_as_ref() {
    let dv = Box::new(false) as Box<dyn MyValueTrait>;

    let dvr: &MyValue = dv.as_ref();

    println!("{:?}", dvr);
}

struct MySeriesIndexing<'a> {
    data: &'a Series,
    cache: Box<dyn MyValueTrait>,
}

#[allow(dead_code)]
impl<'a> MySeriesIndexing<'a> {
    fn new(series: &'a Series) -> Self {
        Self {
            data: series,
            cache: Box::new(Null),
        }
    }
}

impl<'a> Index<usize> for MySeriesIndexing<'a> {
    type Output = MyValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self.data.dtype() {
            DataType::Boolean => {
                let res: Box<dyn MyValueTrait> = match self.data.bool().unwrap().get(index) {
                    Some(v) => Box::new(v),
                    None => Box::new(Null),
                };

                let r = &self.cache as *const Box<dyn MyValueTrait>;
                let m = r as *mut Box<dyn MyValueTrait>;
                unsafe {
                    // release `r`
                    std::ptr::read(r);
                    std::ptr::write(m, res);
                };

                self.cache.as_ref()
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

    let s = MySeriesIndexing::new(&s);

    println!("{:?}", &s[1]);
    println!("{:?}", &s[3]);
}
