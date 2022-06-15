use super::data_source::DataSource;
use crate::datatypes::{
    arrow_field_array::ArrowFieldArray, column_array::ArrayRef, record_batch::RecordBatch,
    schema::Schema,
};
use anyhow::{Ok, Result};
use arrow::{
    array::{Array, BooleanArray},
    datatypes::{DataType, Field as ArrowField, Schema as ArrowSchema},
};
use csv::StringRecord;
use std::{fs::File, rc::Rc, slice::Iter};

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
        Ok(Box::new(
            ReaderIterator::new(csv_reader, schema, self.batch_size).into_iter(),
        ))
    }
}

struct ReaderIterator {
    schema: Schema,
    parser: csv::Reader<File>,
    batch_size: usize,
    next_batch: Option<RecordBatch>,
    started: bool,
}

impl Iterator for ReaderIterator {
    type Item = RecordBatch;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.has_next();
        }

        self.next_batch.clone()
    }
}

impl ReaderIterator {
    fn new(parser: csv::Reader<File>, schema: Schema, batch_size: usize) -> ReaderIterator {
        ReaderIterator {
            schema,
            parser,
            batch_size,
            next_batch: None,
            started: false,
        }
    }

    fn has_next(&mut self) -> bool {
        if !self.started {
            self.started = true;
            self.next_batch = self.next_batch();
        }

        self.next_batch.is_some()
    }

    fn next_batch(&mut self) -> Option<RecordBatch> {
        let mut records = Vec::with_capacity(self.batch_size);
        loop {
            let line = self.parser.records().next();
            if line.is_none() {
                break;
            }

            if line.is_some() {
                let line = line.unwrap().unwrap();
                records.push(line);
            }

            if records.len() >= self.batch_size {
                break;
            }

            if records.is_empty() {
                return None;
            }
        }
        Some(self.create_batch(records))
    }

    fn create_batch(&mut self, rows: Vec<StringRecord>) -> RecordBatch {
        let mut schema: ArrowSchema = self.schema.clone().into();
        let arrays: Vec<Box<dyn Array>> = schema
            .fields()
            .iter()
            .enumerate()
            .map(|(col_index, field)| match field.data_type() {
                DataType::Boolean => Box::new(
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
                ) as Box<dyn Array>,
                _ => panic!("Unsupported type"),
            })
            .collect();

        let fields = arrays
            .into_iter()
            .map(|a| Rc::new(ArrowFieldArray::new(a)) as ArrayRef)
            .collect();
        RecordBatch {
            schema: self.schema.clone(),
            fields: fields,
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
