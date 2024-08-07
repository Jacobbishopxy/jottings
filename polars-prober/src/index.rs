//! Indexing

use std::{any::Any, cell::RefCell, fmt::Debug, ops::Index};

use polars::prelude::*;
use ref_cast::RefCast;

// ================================================================================================
// MyValueTrait
//
// impl for rust primitive values, String and Null
// ================================================================================================

trait MyValueTrait {
    fn dtype(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}

// null value
#[derive(Debug, PartialEq, Eq)]
struct Null;

macro_rules! impl_my_value_trait {
    ($t:ident, $s:expr) => {
        impl $crate::index::MyValueTrait for $t {
            fn dtype(&self) -> &'static str {
                $s
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

impl_my_value_trait!(bool, "bool");
impl_my_value_trait!(u8, "u8");
impl_my_value_trait!(u16, "u16");
impl_my_value_trait!(u32, "u32");
impl_my_value_trait!(u64, "u64");
impl_my_value_trait!(i8, "i8");
impl_my_value_trait!(i16, "i16");
impl_my_value_trait!(i32, "i32");
impl_my_value_trait!(i64, "i64");
impl_my_value_trait!(f32, "f32");
impl_my_value_trait!(f64, "f64");
impl_my_value_trait!(String, "String");
impl_my_value_trait!(Null, "null");

// ================================================================================================
// MyValue
//
// Wrapped by RefCell in order to mutate value without using mutate reference
// ================================================================================================

#[derive(RefCast)]
#[repr(transparent)]
struct MyValue(RefCell<Box<dyn MyValueTrait>>);

#[allow(dead_code)]
impl MyValue {
    fn dtype(&self) -> &'static str {
        self.0.borrow().dtype()
    }

    fn new<T>(v: T) -> Self
    where
        Self: From<T>,
    {
        Self::from(v)
    }
}

macro_rules! compare_my_values {
    ($l:expr, $r:expr, $t:ident) => {{
        let lv = $l.0.borrow();
        let lv = lv.as_any().downcast_ref::<$t>().unwrap();
        let rv = $r.0.borrow();
        let rv = rv.as_any().downcast_ref::<$t>().unwrap();

        lv == rv
    }};
}

impl PartialEq for MyValue {
    fn eq(&self, other: &Self) -> bool {
        let tl = self.0.borrow().dtype();
        let tr = other.0.borrow().dtype();

        match (tl, tr) {
            ("bool", "bool") => compare_my_values!(self, other, bool),
            ("u8", "u8") => compare_my_values!(self, other, u8),
            ("u16", "u16") => compare_my_values!(self, other, u16),
            ("u32", "u32") => compare_my_values!(self, other, u32),
            ("u64", "u64") => compare_my_values!(self, other, u64),
            ("i8", "i8") => compare_my_values!(self, other, i8),
            ("i16", "i16") => compare_my_values!(self, other, i16),
            ("i32", "i32") => compare_my_values!(self, other, i32),
            ("i64", "i64") => compare_my_values!(self, other, i64),
            ("f32", "f32") => compare_my_values!(self, other, f32),
            ("f64", "f64") => compare_my_values!(self, other, f64),
            ("String", "String") => compare_my_values!(self, other, String),
            ("null", "null") => compare_my_values!(self, other, Null),
            _ => false,
        }
    }
}

macro_rules! impl_from_x_for_my_value {
    ($t:ident) => {
        impl From<$t> for $crate::index::MyValue {
            fn from(v: $t) -> Self {
                MyValue(RefCell::new(Box::new(v)))
            }
        }
    };
}

impl_from_x_for_my_value!(bool);
impl_from_x_for_my_value!(u8);
impl_from_x_for_my_value!(u16);
impl_from_x_for_my_value!(u32);
impl_from_x_for_my_value!(u64);
impl_from_x_for_my_value!(i8);
impl_from_x_for_my_value!(i16);
impl_from_x_for_my_value!(i32);
impl_from_x_for_my_value!(i64);
impl_from_x_for_my_value!(f32);
impl_from_x_for_my_value!(f64);
impl_from_x_for_my_value!(String);
impl_from_x_for_my_value!(Null);

impl AsRef<MyValue> for RefCell<Box<dyn MyValueTrait>> {
    fn as_ref(&self) -> &MyValue {
        MyValue::ref_cast(self)
    }
}

macro_rules! set_debug {
    ($s:expr, $d:expr, $t:ident) => {{
        let v = $s.0.borrow();
        let v = v.as_any().downcast_ref::<$t>().unwrap();
        $d.field(v);
    }};
}

impl Debug for MyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_tuple("MyValue");

        match self.dtype() {
            "bool" => set_debug!(self, d, bool),
            "u8" => set_debug!(self, d, u8),
            "u16" => set_debug!(self, d, u16),
            "u32" => set_debug!(self, d, u32),
            "u64" => set_debug!(self, d, u64),
            "i8" => set_debug!(self, d, i8),
            "i16" => set_debug!(self, d, i16),
            "i32" => set_debug!(self, d, i32),
            "i64" => set_debug!(self, d, i64),
            "f32" => set_debug!(self, d, f32),
            "f64" => set_debug!(self, d, f64),
            "String" => set_debug!(self, d, String),
            "null" => set_debug!(self, d, Null),
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
    assert_eq!(res.dtype(), "bool");
}

// ================================================================================================
// MySeriesIndexing
//
// cache the temporary value created in `index` function, so that to return
// the indexing value by a reference.
// ================================================================================================

struct MySeriesIndexing {
    data: Series,
    cache: RefCell<Box<dyn MyValueTrait>>,
}

#[allow(dead_code)]
impl MySeriesIndexing {
    fn new(series: Series) -> Self {
        Self {
            data: series,
            cache: RefCell::new(Box::new(Null)),
        }
    }
}

macro_rules! get_index_value {
    ($s:expr, $idx:expr, $f:ident) => {{
        let res: Box<dyn $crate::index::MyValueTrait> = match $s.data.$f().unwrap().get($idx) {
            Some(v) => Box::new(v),
            None => Box::new(Null),
        };

        $s.cache.replace(res);

        MyValue::ref_cast(&$s.cache)
    }};
}

impl Index<usize> for MySeriesIndexing {
    type Output = MyValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self.data.dtype() {
            DataType::Boolean => get_index_value!(self, index, bool),
            DataType::UInt8 => get_index_value!(self, index, u8),
            DataType::UInt16 => get_index_value!(self, index, u16),
            DataType::UInt32 => get_index_value!(self, index, u32),
            DataType::UInt64 => get_index_value!(self, index, u64),
            DataType::Int8 => get_index_value!(self, index, i8),
            DataType::Int16 => get_index_value!(self, index, i16),
            DataType::Int32 => get_index_value!(self, index, i32),
            DataType::Int64 => get_index_value!(self, index, i64),
            DataType::Float32 => get_index_value!(self, index, f32),
            DataType::Float64 => get_index_value!(self, index, f64),
            DataType::String => {
                let res: Box<dyn MyValueTrait> = match self.data.str().unwrap().get(index) {
                    Some(v) => Box::new(v.to_string()),
                    None => Box::new(Null),
                };

                self.cache.replace(res);

                MyValue::ref_cast(&self.cache)
            }
            _ => {
                self.cache.replace(Box::new(Null));
                MyValue::ref_cast(&self.cache)
            }
        }
    }
}

#[test]
fn my_series_index_success() {
    let s = Series::new("funk", [true, false, true, true]);

    let s = MySeriesIndexing::new(s);

    assert_eq!(&s[1], &MyValue::new(false));
    assert_eq!(&s[3], &MyValue::new(true));
    println!("{:?}", &s[1]);
    println!("{:?}", &s[3]);
}
