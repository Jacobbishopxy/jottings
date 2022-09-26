//! Datagrid

use anyhow::{Error, Result};
use arrow2::array::*;
use arrow2::chunk::Chunk;
use arrow2::datatypes::*;
use arrow2::io::avro::avro_schema;
use arrow2::io::avro::read as avro_read;
use arrow2::io::avro::write as avro_write;
use arrow2::io::parquet::read as parquet_read;
use arrow2::io::parquet::write as parquet_write;

pub struct Datagrid(Chunk<Box<dyn Array>>);

impl Datagrid {
    pub fn empty() -> Self {
        Datagrid(Chunk::new(vec![]))
    }

    pub fn new(arrays: Vec<Box<dyn Array>>) -> Self {
        Datagrid(Chunk::new(arrays))
    }

    pub fn try_new(arrays: Vec<Box<dyn Array>>) -> Result<Self> {
        let chunk = Chunk::try_new(arrays).map_err(Error::msg)?;
        Ok(Datagrid(chunk))
    }

    pub fn write_avro<W: std::io::Write>(
        &self,
        file: &mut W,
        schema: &Schema,
        compression: Option<avro_schema::file::Compression>,
    ) -> Result<()> {
        let record = avro_write::to_record(schema)?;
        let arrays = self.0.arrays();

        let mut serializers = arrays
            .iter()
            .zip(record.fields.iter())
            .map(|(array, field)| avro_write::new_serializer(array.as_ref(), &field.schema))
            .collect::<Vec<_>>();
        let mut block = avro_schema::file::Block::new(arrays[0].as_ref().len(), vec![]);

        avro_write::serialize(&mut serializers, &mut block);

        let mut compressed_block = avro_schema::file::CompressedBlock::default();

        let _was_compressed =
            avro_schema::write::compress(&mut block, &mut compressed_block, compression)
                .map_err(Error::msg)?;

        avro_schema::write::write_metadata(file, record, compression).map_err(Error::msg)?;

        avro_schema::write::write_block(file, &compressed_block).map_err(Error::msg)?;

        Ok(())
    }

    pub fn read_avro<R: std::io::Read>(&mut self, reader: &mut R) -> Result<()> {
        let metadata = avro_schema::read::read_metadata(reader).map_err(Error::msg)?;

        let schema = avro_read::infer_schema(&metadata.record).map_err(Error::msg)?;

        let mut blocks = avro_read::Reader::new(reader, metadata, schema.fields, None);

        if let Some(Ok(c)) = blocks.next() {
            self.0 = c;
        }

        Ok(())
    }

    pub fn write_parquet<W: std::io::Write>() -> Result<()> {
        todo!()
    }

    pub fn read_parquet<R: std::io::Read>() -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test_datagrid {

    use super::*;

    const FILE_PATH: &str = "./cache/test.avro";

    #[test]
    fn avro_write_success() {
        let a = Int32Array::from([Some(1), None, Some(3)]).boxed();
        let b = Float32Array::from([Some(2.1), None, Some(6.2)]).boxed();
        let c = Utf8Array::<i32>::from([Some("a"), Some("b"), Some("c")]).boxed();

        let schema = vec![
            Field::new("c1", a.data_type().clone(), true),
            Field::new("c2", b.data_type().clone(), true),
            Field::new("c3", c.data_type().clone(), true),
        ]
        .into();

        let datagrid = Datagrid::new(vec![a, b, c]);

        let mut file = std::fs::File::create(FILE_PATH).unwrap();

        datagrid
            .write_avro(&mut file, &schema, None)
            .expect("write success")
    }

    #[test]
    fn avro_read_success() {
        let mut datagrid = Datagrid::empty();

        let mut file = std::fs::File::open(FILE_PATH).unwrap();

        datagrid.read_avro(&mut file).unwrap();

        let data_types = datagrid
            .0
            .arrays()
            .iter()
            .map(|a| a.data_type())
            .collect::<Vec<_>>();
        println!("{:?}", data_types);
    }
}
