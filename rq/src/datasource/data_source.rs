use crate::datatypes::{record_batch::RecordBatch, schema::Schema};

pub(crate) trait DataSource {
    /// Return the schema for the underlying data source.
    fn get_schema(&self) -> &Schema;
    /// Scan the data source, selecting the specified columns.
    fn scan(&self, projection: Vec<String>) -> Vec<RecordBatch>;
}
