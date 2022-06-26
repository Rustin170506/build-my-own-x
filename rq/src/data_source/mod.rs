use crate::data_types::{record_batch::RecordBatch, schema::Schema};
use anyhow::Result;

pub(crate) mod csv_data_source;
pub(crate) mod memory_data_source;
pub(crate) mod reader_parser;


pub(crate) trait DataSource {
    /// Return the schema for the underlying data source.
    fn get_schema(&self) -> &Schema;
    /// Scan the data source, selecting the specified columns.
    fn scan(&self, projection: Vec<String>) -> Result<Box<dyn Iterator<Item = RecordBatch> + '_>>;
}
