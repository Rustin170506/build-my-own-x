use super::{reader_parser::Parser, DataSource};
use crate::data_types::{
    arrow_field_array::ArrowFieldArray, column_array::ArrayRef, record_batch::RecordBatch,
    schema::Schema,
};
use anyhow::{Ok, Result};
use arrow::{
    array::{BooleanArray, PrimitiveArray, StringArray},
    datatypes::{
        ArrowPrimitiveType, DataType as ArrowDataType, Float32Type, Float64Type, Int16Type,
        Int32Type, Int64Type, Int8Type, Schema as ArrowSchema, UInt16Type, UInt32Type, UInt64Type,
        UInt8Type,
    },
};
use csv::{Reader, ReaderBuilder, StringRecord};
use std::{fs::File, rc::Rc};

// A data source that reads from a CSV file.
#[derive(Clone)]
pub(crate) struct CsvDataSource {
    file_name: String,
    schema: Schema,
    // The total number of rows in the CSV file.
    batch_size: usize,
}

impl DataSource for CsvDataSource {
    fn get_schema(&self) -> &Schema {
        &self.schema
    }

    fn scan(&self, projections: Vec<&str>) -> Result<Box<dyn Iterator<Item = RecordBatch>>> {
        let file = File::open(&self.file_name)?;
        let schema = if projections.is_empty() {
            self.schema.clone()
        } else {
            self.schema.select(projections)
        };
        let mut csv_reader_builder = ReaderBuilder::new();
        csv_reader_builder.has_headers(false);
        let mut csv_reader = csv_reader_builder.from_reader(file);
        // Set headers for the CSV reader.
        // This will append the name into the first record of reader.
        csv_reader.set_headers(schema.fields.iter().map(|f| f.name.clone()).collect());
        let csv_data_source_reader = CsvDataSourceReader::new(csv_reader, schema, self.batch_size);
        Ok(Box::new(csv_data_source_reader))
    }
}

impl CsvDataSource {
    pub(crate) fn new(file_name: String, schema: Schema, batch_size: usize) -> Self {
        Self {
            file_name,
            schema,
            batch_size,
        }
    }
}

// A reader for the CSV data source with the specified schema.
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

    // Build a record batch from the given records.
    // String -> ArrowFieldArray -> ArrayRef -> RecordBatch.
    fn create_batch(&mut self, rows: Vec<StringRecord>) -> RecordBatch {
        let schema: ArrowSchema = self.schema.clone().into();
        let arrays = schema
            .fields()
            .iter()
            .enumerate()
            .map(|(col_index, field)| match field.data_type() {
                ArrowDataType::Boolean => build_boolean_array(&rows, col_index),
                ArrowDataType::Int32 => build_primitive_array::<Int32Type>(&rows, col_index),
                ArrowDataType::Int64 => build_primitive_array::<Int64Type>(&rows, col_index),
                ArrowDataType::Float32 => build_primitive_array::<Float32Type>(&rows, col_index),
                ArrowDataType::Float64 => build_primitive_array::<Float64Type>(&rows, col_index),
                ArrowDataType::Utf8 => build_string_array(&rows, col_index),
                _ => unreachable!(),
            })
            .collect();

        RecordBatch {
            schema: self.schema.clone(),
            fields: arrays,
        }
    }
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

fn build_boolean_array(rows: &[StringRecord], col_index: usize) -> ArrayRef {
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

fn build_primitive_array<T: ArrowPrimitiveType + Parser>(
    rows: &[StringRecord],
    col_index: usize,
) -> ArrayRef {
    let array = Box::new(
        rows.iter()
            .map(|row| match row.get(col_index) {
                Some(s) => {
                    if s.is_empty() {
                        return None;
                    }

                    let parsed = T::parse(s);
                    match parsed {
                        Some(e) => Some(e),
                        None => panic!("Failed to parse {}", s),
                    }
                }
                None => None,
            })
            .collect::<PrimitiveArray<T>>(),
    );

    Rc::new(ArrowFieldArray::new(array)) as ArrayRef
}

fn build_string_array(rows: &[StringRecord], col_index: usize) -> ArrayRef {
    let array = Box::new(
        rows.iter()
            .map(|row| row.get(col_index))
            .collect::<StringArray>(),
    );

    Rc::new(ArrowFieldArray::new(array)) as ArrayRef
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_types::{column_array::DataType, schema::Field};
    use std::{any::Any, fmt::Debug, path::PathBuf};

    #[test]
    fn test_boolean_field_csv_data_source() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/boolean_field.csv");
        let schema = Schema::new(vec![Field::new("c1".to_string(), DataType::Boolean)]);
        let csv_data_source =
            CsvDataSource::new(data_path.into_os_string().into_string().unwrap(), schema, 3);
        let mut reader = csv_data_source.scan(vec!["c1"]).unwrap();
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

    #[test]
    fn test_primitive_field_csv_data_source() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/primitive_field.csv");
        let schema = Schema::new(vec![
            Field::new("c1".to_string(), DataType::Int32),
            Field::new("c2".to_string(), DataType::Int32),
            Field::new("c3".to_string(), DataType::Int64),
            Field::new("c4".to_string(), DataType::Int64),
            Field::new("c5".to_string(), DataType::Float32),
            Field::new("c6".to_string(), DataType::Float64),
        ]);
        let csv_data_source =
            CsvDataSource::new(data_path.into_os_string().into_string().unwrap(), schema, 3);
        let mut reader = csv_data_source
            .scan(vec!["c1", "c2", "c3", "c4", "c5", "c6"])
            .unwrap();
        let batch = reader.next().unwrap();

        assert_eq!(batch.row_count(), 3);
        assert_eq!(batch.column_count(), 6);

        fn assert_type_and_values<T: Any + PartialEq + Debug>(
            batch: &RecordBatch,
            index: usize,
            data_type: DataType,
            values: Vec<T>,
        ) {
            assert_eq!(batch.field(index).get_type(), data_type);

            for (idx, val) in values.iter().enumerate() {
                assert_eq!(
                    batch
                        .field(index)
                        .get_value(idx)
                        .unwrap()
                        .downcast_ref::<T>()
                        .unwrap(),
                    val
                );
            }
        }

        assert_type_and_values::<i32>(&batch, 0, DataType::Int32, vec![1, 2, 3]);
        assert_type_and_values::<i32>(&batch, 1, DataType::Int32, vec![9, 10, 11]);
        assert_type_and_values::<i64>(&batch, 2, DataType::Int64, vec![20, 21, 22]);
        assert_type_and_values::<i64>(&batch, 3, DataType::Int64, vec![30, 31, 32]);
        assert_type_and_values::<f32>(&batch, 4, DataType::Float32, vec![1.0, 1.1, 1.2]);
        assert_type_and_values::<f64>(
            &batch,
            5,
            DataType::Float64,
            vec![
                0.0000000000000000000001,
                0.0000000000000000000002,
                0.0000000000000000000003,
            ],
        );
    }

    #[test]
    fn test_string_field_csv_data_source() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/string_field.csv");
        let schema = Schema::new(vec![Field::new("c1".to_string(), DataType::Utf8)]);
        let csv_data_source =
            CsvDataSource::new(data_path.into_os_string().into_string().unwrap(), schema, 3);
        let mut reader = csv_data_source.scan(vec!["c1"]).unwrap();
        let batch = reader.next().unwrap();

        assert_eq!(batch.row_count(), 3);
        assert_eq!(batch.column_count(), 1);

        assert_eq!(
            batch
                .field(0)
                .get_value(0)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
            "a"
        );
        assert_eq!(
            batch
                .field(0)
                .get_value(1)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
            "b"
        );
        assert_eq!(
            batch
                .field(0)
                .get_value(2)
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
            "c"
        );
    }
}
