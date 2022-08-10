use crate::{
    data_source::{csv_data_source::CsvDataSource, Source},
    data_types::{
        column_array::DataType,
        schema::{Field, Schema},
    },
};
use std::path::PathBuf;

pub(crate) fn get_data_source() -> (String, Box<Source>) {
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
    let path = data_path.into_os_string().into_string().unwrap();
    let csv_data_source = CsvDataSource::new(path.clone(), schema, 3);
    (path, Box::new(Source::Csv(csv_data_source)))
}
