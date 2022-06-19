use super::data_source::DataSource;
use crate::datatypes::{
    arrow_field_array::ArrowFieldArray, column_array::ArrayRef, record_batch::RecordBatch,
    schema::Schema,
};
use anyhow::{Ok, Result};
use arrow::{
    array::BooleanArray,
    datatypes::{DataType, Schema as ArrowSchema},
};
use csv::{Reader, ReaderBuilder, StringRecord};
use std::{fs::File, rc::Rc};

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
        let mut csv_reader_builder = ReaderBuilder::new();
        csv_reader_builder.has_headers(self.has_headers);
        let mut csv_reader = csv_reader_builder.from_reader(file);
        csv_reader.set_headers(schema.fields.iter().map(|f| f.name.clone()).collect());
        let csv_data_source_reader = CsvDataSourceReader::new(csv_reader, schema, self.batch_size);
        Ok(Box::new(csv_data_source_reader))
    }
}

impl CsvDataSource {
    fn new(file_name: String, schema: Schema, has_headers: bool, batch_size: usize) -> Self {
        Self {
            file_name,
            schema,
            has_headers,
            batch_size,
        }
    }
}

struct CsvDataSourceReader {
    schema: Schema,
    parser: Reader<File>,
    batch_size: usize,
}

impl Iterator for CsvDataSourceReader {
    type Item = RecordBatch;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_batch()
    }
}

impl CsvDataSourceReader {
    fn new(parser: Reader<File>, schema: Schema, batch_size: usize) -> CsvDataSourceReader {
        CsvDataSourceReader {
            schema,
            parser,
            batch_size,
        }
    }

    fn next_batch(&mut self) -> Option<RecordBatch> {
        let mut records = Vec::with_capacity(self.batch_size);
        // Skip headers.
        let _ = self.parser.records().next();
        loop {
            let line = self.parser.records().next();
            if let Some(line) = line {
                records.push(line.unwrap());
            } else {
                break;
            }

            if records.len() >= self.batch_size {
                break;
            }
        }

        if records.is_empty() {
            return None;
        }

        Some(self.create_batch(records))
    }

    fn create_batch(&mut self, rows: Vec<StringRecord>) -> RecordBatch {
        let schema: ArrowSchema = self.schema.clone().into();
        let arrays = schema
            .fields()
            .iter()
            .enumerate()
            .map(|(col_index, field)| match field.data_type() {
                DataType::Boolean => build_boolean_array(rows.clone(), col_index),
                _ => unreachable!(),
            })
            .collect();

        RecordBatch {
            schema: self.schema.clone(),
            fields: arrays,
        }
    }
}

fn build_boolean_array(rows: Vec<StringRecord>, col_index: usize) -> ArrayRef {
    let array = Box::new(
        rows.iter()
            .map(|row| match row.get(col_index) {
                Some(s) => {
                    if s.is_empty() {
                        return None;
                    }

                    let parsed = parse_bool(s);
                    match parsed {
                        Some(e) => Some(e),
                        None => panic!("Failed to parse bool: {}", s),
                    }
                }
                None => None,
            })
            .collect::<BooleanArray>(),
    );

    Rc::new(ArrowFieldArray::new(array)) as ArrayRef
}

fn parse_bool(string: &str) -> Option<bool> {
    if string.eq_ignore_ascii_case("false") {
        Some(false)
    } else if string.eq_ignore_ascii_case("true") {
        Some(true)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatypes::schema::Field;
    use std::path::PathBuf;

    #[test]
    fn test_boolean_field_csv_data_source() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/boolean_field.csv");
        let schema = Schema::new(vec![Field::new("c1".to_string(), DataType::Boolean)]);
        let csv_data_source = CsvDataSource::new(
            data_path.into_os_string().into_string().unwrap(),
            schema,
            false,
            3,
        );
        let mut reader = csv_data_source.scan(vec!["c1".to_string()]).unwrap();
        let batch = reader.next().unwrap();

        assert_eq!(batch.row_count(), 3);
        assert_eq!(batch.column_count(), 1);
        assert_eq!(batch.field(0).get_type(), DataType::Boolean);
        assert!(batch
            .field(0)
            .get_value(0)
            .unwrap()
            .downcast_ref::<bool>()
            .unwrap());
        assert!(!batch
            .field(0)
            .get_value(1)
            .unwrap()
            .downcast_ref::<bool>()
            .unwrap());
        assert!(batch
            .field(0)
            .get_value(2)
            .unwrap()
            .downcast_ref::<bool>()
            .unwrap());
    }
}
