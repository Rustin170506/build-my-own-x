use std::path::PathBuf;

use crate::{
    data_source::{csv_data_source::CsvDataSource, Source},
    data_types::{
        column_array::DataType,
        schema::{Field, Schema},
    },
};

pub(crate) fn get_primitive_field_data_source() -> (String, Source) {
    let schema = Schema::new(vec![
        Field::new("c1".to_string(), DataType::Int32),
        Field::new("c2".to_string(), DataType::Int32),
        Field::new("c3".to_string(), DataType::Int64),
        Field::new("c4".to_string(), DataType::Int64),
        Field::new("c5".to_string(), DataType::Float32),
        Field::new("c6".to_string(), DataType::Float64),
    ]);
    let path = rq_test_data("primitive_field.csv");
    let csv_data_source = CsvDataSource::new(path.clone(), schema, 3);
    (path, Source::Csv(csv_data_source))
}

/// Returns the rq test data directory.
pub(crate) fn rq_test_data(file_name: &str) -> String {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push(format!("tests/data/{}", file_name));
    data_path
        .into_os_string()
        .into_string()
        .expect("failed to get arrow data dir")
}
