use self::{csv_data_source::CsvDataSource, memory_data_source::MemoryDataSource};
use crate::data_types::{record_batch::RecordBatch, schema::Schema};
use anyhow::Result;

pub(crate) mod csv_data_source;
pub(crate) mod memory_data_source;
pub(crate) mod reader_parser;

pub(crate) trait DataSource {
    /// Return the schema for the underlying data source.
    fn get_schema(&self) -> &Schema;
    /// Scan the data source, selecting the specified columns.
    fn scan(&self, projection: Vec<&str>) -> Result<Vec<RecordBatch>>;
}

#[derive(Clone)]
pub(crate) enum Source {
    Csv(CsvDataSource),
    Mem(MemoryDataSource),
}

impl DataSource for Source {
    fn get_schema(&self) -> &Schema {
        match self {
            Source::Csv(csv_data_source) => csv_data_source.get_schema(),
            Source::Mem(memory_data_source) => memory_data_source.get_schema(),
        }
    }

    fn scan(&self, projection: Vec<&str>) -> Result<Vec<RecordBatch>> {
        match self {
            Source::Csv(csv_data_source) => csv_data_source.scan(projection),
            Source::Mem(memory_data_source) => memory_data_source.scan(projection),
        }
    }
}
