use crate::datatypes::{record_batch::RecordBatch, schema::Schema};

pub(crate) trait DataSource {
    fn get_schema(&self) -> &Schema;
    fn scan(&self, prokection: Vec<String>) -> Vec<RecordBatch>;
}
