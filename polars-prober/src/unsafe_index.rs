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

impl MyValueTrait for i64 {
    fn dtype(&self) -> &'static str {
        "i64"
    }
}

#[derive(Debug)]
struct Null;

impl MyValueTrait for Null {
    fn dtype(&self) -> &'static str {
        "null"
    }
}

// This struct won't work, because for example `MyGenericValue<bool>` and `MyGenericValue<i64>` do not have the same size.
#[allow(dead_code)]
struct MyGenericValue<T: MyValueTrait>(T);

#[test]
fn my_generic_value_not_same_size() {
    let v1 = MyGenericValue(true);
    println!("{:?}", std::mem::size_of_val(&v1));

    let v2 = MyGenericValue(1i64);
    println!("{:?}", std::mem::size_of_val(&v2));

    assert_ne!(std::mem::size_of_val(&v1), std::mem::size_of_val(&v2));
}

// This struct won't work, because when impl `Index`, there is no way to hold the original variable which is also UnSized.
#[derive(Debug)]
struct MyRefValue<'a>(&'a dyn MyValueTrait);

impl<'a> AsRef<MyRefValue<'a>> for &'a dyn MyValueTrait {
    fn as_ref(&self) -> &MyRefValue<'a> {
        unsafe { std::mem::transmute(self) }
    }
}

#[test]
fn my_ref_value_as_ref() {
    let v = true;

    let dv = &v as &dyn MyValueTrait;

    let dvr: &MyRefValue = dv.as_ref();

    println!("{:?}", dvr);
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

impl From<bool> for MyValue {
    fn from(v: bool) -> Self {
        Self(Box::new(v))
    }
}

impl From<i64> for MyValue {
    fn from(v: i64) -> Self {
        Self(Box::new(v))
    }
}

impl From<Null> for MyValue {
    fn from(v: Null) -> Self {
        Self(Box::new(v))
    }
}

#[test]
fn my_value_from_x() {
    let v1 = MyValue::from(false);
    let v2 = MyValue::from(1i64);
    let v3 = MyValue::from(Null);

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
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
                // unpack series to `ChunkedArray`
                let res: Box<dyn MyValueTrait> = match self.data.bool().unwrap().get(index) {
                    Some(v) => Box::new(v),
                    None => Box::new(Null),
                };

                let r = &self.cache as *const Box<dyn MyValueTrait>;
                let m = r as *mut Box<dyn MyValueTrait>;
                unsafe {
                    *m = res;
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
            DataType::Int64 => {
                // directly call `.get` method, which has a runtime casting (less efficiency)
                let res: Box<dyn MyValueTrait> = match self.data.get(index) {
                    AnyValue::Int64(v) => Box::new(v),
                    _ => Box::new(Null),
                };

                let r = &self.cache as *const Box<dyn MyValueTrait>;
                let m = r as *mut Box<dyn MyValueTrait>;
                unsafe {
                    *m = res;
                };

                self.cache.as_ref()
            }
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