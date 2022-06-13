use super::data_source::DataSource;
use crate::datatypes::{record_batch::RecordBatch, schema::Schema};
use anyhow::{Ok, Result};
use std::{fs::File, slice::Iter};

struct CsvDataSource {
    file_name: String,
    schema: Schema,
    has_headers: bool,
    batch_size: usize,
}

impl DataSource for CsvDataSource {
    fn get_schema(&self) -> &Schema {
        &self.schema
    }

    fn scan(&self, projections: Vec<String>) -> Result<Box<dyn Iterator<Item = RecordBatch>>> {
        let file = File::open(self.file_name.clone())?;
        let schema = if projections.is_empty() {
            self.schema.clone()
        } else {
            self.schema.select(projections)
        };
        let mut csv_reader_builder = csv::ReaderBuilder::new();
        csv_reader_builder.has_headers(self.has_headers);
        let mut csv_reader = csv_reader_builder.from_reader(file);
        csv_reader.set_headers(schema.fields.iter().map(|f| f.name.clone()).collect());
        todo!()
    }
}
