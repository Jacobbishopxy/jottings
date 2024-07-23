//! Impl arrow Array

use std::{any::Any, fmt::Debug, fmt::Display, fs::File, hash::Hash};

use arrow2::{
    array::Array,
    bitmap::{Bitmap, MutableBitmap},
    datatypes::DataType,
    error::Error,
    io::json,
};
use uuid::Uuid;

// ================================================================================================
// MyArrowObject
// Same as `PolarsObject`
// ================================================================================================

pub trait MyArrowObject:
    Any + Debug + Clone + Send + Sync + Default + Display + Hash + PartialEq + Eq
{
    fn type_name() -> &'static str;
}

// ================================================================================================
// MyUuid
// NewType as custom type
// ================================================================================================

#[derive(Clone, PartialEq, Eq, Hash, Default)]
struct MyUuid(Uuid);

#[allow(dead_code)]
impl MyUuid {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Debug for MyUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for MyUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MyArrowObject for MyUuid {
    fn type_name() -> &'static str {
        "uuid"
    }
}

// ================================================================================================
// MyUuid
// Same as polars `ObjectArray`, which implements arrow's `Array`
// ================================================================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MyObjectArray<T>
where
    T: MyArrowObject,
{
    values: Vec<T>,
    null_bitmap: Option<Bitmap>,
    offset: usize,
    len: usize,
}

#[allow(dead_code)]
impl<T> MyObjectArray<T>
where
    T: MyArrowObject,
{
    fn values(&self) -> &[T] {
        &self.values
    }
}

impl<T> Array for MyObjectArray<T>
where
    T: MyArrowObject,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn len(&self) -> usize {
        self.len
    }

    // There is no DataType for us to choose.
    // This indicates which serializer to use while writing `MyObjectArray<T>` to Json.
    // See `arrow2::io::json::write::serialize::new_serializer` for more detail.
    // Consequently, even though we implemented `Array` for our `MyObjectArray<T>`,
    // we still cannot use arrow's `io::json::write::write` method.
    fn data_type(&self) -> &DataType {
        unimplemented!()
    }

    fn validity(&self) -> Option<&Bitmap> {
        self.null_bitmap.as_ref()
    }

    fn slice(&mut self, offset: usize, length: usize) {
        assert!(
            offset + length <= self.len(),
            "the offset of the new Buffer cannot exceed the existing length"
        );
        unsafe { self.slice_unchecked(offset, length) };
    }

    unsafe fn slice_unchecked(&mut self, offset: usize, length: usize) {
        let len = std::cmp::min(self.len() - offset, length);

        self.len = len;
        self.offset = offset;
        self.null_bitmap
            .as_mut()
            .map(|x| x.slice_unchecked(offset, len));
    }

    fn with_validity(&self, validity: Option<Bitmap>) -> Box<dyn Array> {
        let mut arr = self.clone();
        arr.null_bitmap = validity;
        Box::new(arr)
    }

    fn to_boxed(&self) -> Box<dyn Array> {
        Box::new(self.clone())
    }
}

// ================================================================================================
// MyObjectArrayBuilder<T>
// ObjectArray builder
// ================================================================================================

#[allow(dead_code)]
struct MyObjectArrayBuilder<T> {
    bitmask_builder: MutableBitmap,
    values: Vec<T>,
}

#[allow(dead_code)]
impl<T> MyObjectArrayBuilder<T>
where
    T: MyArrowObject,
{
    fn new(capacity: usize) -> Self {
        Self {
            bitmask_builder: MutableBitmap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    fn append_value(&mut self, v: T) {
        self.values.push(v);
        self.bitmask_builder.push(true);
    }

    fn finish(self) -> MyObjectArray<T> {
        let null_bitmap: Option<Bitmap> = self.bitmask_builder.into();
        let len = self.values.len();

        MyObjectArray {
            values: self.values,
            null_bitmap,
            offset: 0,
            len,
        }
    }

    fn type_name() -> &'static str {
        T::type_name()
    }
}

#[allow(dead_code)]
fn write_array(path: &str, array: &dyn Array) -> Result<(), Error> {
    let mut writer = File::create(path)?;

    let arrays = vec![Ok(array)].into_iter();

    let blocks = json::write::Serializer::new(arrays, vec![]);

    json::write::write(&mut writer, blocks)?;

    Ok(())
}

#[test]
fn write_uuid_success() -> Result<(), Error> {
    let _file_path = "./cache/uuid.json";

    let cap = 5;

    let mut array_builder = MyObjectArrayBuilder::new(cap);
    print!("{:?}", array_builder.values.first());

    for _ in 0..cap {
        array_builder.append_value(MyUuid::new());
    }

    let _array = array_builder.finish();

    // this will cause panic
    // write_array(_file_path, &_array)

    Ok(())
}
