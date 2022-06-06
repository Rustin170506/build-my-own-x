use super::data_source::DataSource;
use crate::datatypes::{record_batch::RecordBatch, schema::Schema};

struct MemeoryDataSource {
    schema: Schema,
    data: Vec<RecordBatch>,
}

impl DataSource for MemeoryDataSource {
    fn get_schema(&self) -> &Schema {
        &self.schema
    }

    fn scan(&self, projection: Vec<String>) -> Vec<RecordBatch> {
        let projection_indices = projection
            .iter()
            .filter_map(|name| self.schema.fields.iter().position(|f| f.name == *name))
            .collect::<Vec<_>>();
        self.data
            .iter()
            .map(|batch| RecordBatch {
                schema: self.schema.clone(),
                fields: projection_indices.iter().map(|i| batch.field(*i)).collect(),
            })
            .collect()
    }
}
