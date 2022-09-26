//! Datagrid

use std::sync::Arc;

use futures::pin_mut;
use futures::StreamExt;
use tokio::fs::File;
use tokio_util::compat::*;

use anyhow::{anyhow, Error, Result};
use arrow2::array::*;
use arrow2::chunk::Chunk;
use arrow2::datatypes::{Field, Schema};
use arrow2::io::avro::avro_schema;
use arrow2::io::avro::read;
use arrow2::io::avro::write;

pub struct Datagrid(Chunk<Box<dyn Array>>);

impl Datagrid {
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
        let record = write::to_record(schema)?;
        let arrays = self.0.arrays();

        let mut serializers = arrays
            .iter()
            .zip(record.fields.iter())
            .map(|(array, field)| write::new_serializer(array.as_ref(), &field.schema))
            .collect::<Vec<_>>();
        let mut block = avro_schema::file::Block::new(arrays[0].as_ref().len(), vec![]);

        write::serialize(&mut serializers, &mut block);

        let mut compressed_block = avro_schema::file::CompressedBlock::default();

        let _was_compressed =
            avro_schema::write::compress(&mut block, &mut compressed_block, compression)
                .map_err(Error::msg)?;

        avro_schema::write::write_metadata(file, record, compression).map_err(Error::msg)?;

        avro_schema::write::write_block(file, &compressed_block).map_err(Error::msg)?;

        Ok(())
    }

    pub async fn read_avro<R: std::io::Read>(
        &mut self,
        reader: &mut R,
        compression: Option<avro_schema::file::Compression>,
    ) -> Result<()> {
        let metadata = avro_schema::read::read_metadata(reader).map_err(Error::msg)?;

        let schema = read::infer_schema(&metadata.record).map_err(Error::msg)?;

        let mut blocks = read::Reader::new(reader, metadata, schema.fields, None);

        while let Some(Ok(c)) = blocks.next() {
            self.0 = c;
        }

        Ok(())
    }
}

#[test]
fn name() {
    let a = Int32Vec::from([Some(1), None, Some(3)]).as_box();
    let b = Float32Vec::from([Some(2.1), None, Some(6.2)]).as_box();
    let c = Utf8Array::<i32>::from([Some("a"), Some("b"), Some("c")]).boxed();

    let foo = Chunk::new(vec![a, b, c]);
}
