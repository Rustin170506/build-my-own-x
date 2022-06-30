use super::plan::LogicalPlan;
use crate::{data_source::DataSource, data_types::schema::Schema};

pub(crate) struct Scan {
    path: String,
    data_source: Box<dyn DataSource>,
    projection: Vec<String>,
}

impl LogicalPlan for Scan {
    fn schema(&self) -> Schema {
        self.data_source.get_schema().clone()
    }

    fn children(&self) -> Vec<Box<dyn LogicalPlan>> {
        vec![]
    }
}

impl ToString for Scan {
    fn to_string(&self) -> String {
        if self.projection.is_empty() {
            format!("Scan: {}; projection=None", self.path)
        } else {
            format!(
                "Scan: {}; projection=[{}]",
                self.path,
                self.projection
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
    }
}

impl Scan {
    fn new(path: String, data_source: Box<dyn DataSource>, projection: Vec<String>) -> Self {
        Scan {
            path,
            data_source,
            projection,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scan;
    use crate::{
        data_source::{csv_data_source::CsvDataSource, DataSource},
        data_types::schema::{Field, Schema},
        logical_plan::plan::LogicalPlan,
    };
    use arrow::datatypes::DataType;
    use std::path::PathBuf;

    fn get_data_source() -> (String, Box<dyn DataSource>) {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/primitive_field.csv");
        let schema = Schema::new(vec![
            Field::new("c1".to_string(), DataType::Int8),
            Field::new("c2".to_string(), DataType::Int16),
            Field::new("c3".to_string(), DataType::UInt32),
            Field::new("c4".to_string(), DataType::UInt64),
            Field::new("c5".to_string(), DataType::Float32),
            Field::new("c6".to_string(), DataType::Float64),
        ]);
        let path = data_path.into_os_string().into_string().unwrap();
        let csv_data_source = CsvDataSource::new(path.clone(), schema, 3);
        (path, Box::new(csv_data_source))
    }

    #[test]
    fn test_schema() {
        let (path, csv_data_source) = get_data_source();
        let schema = csv_data_source.get_schema().clone();
        let plan = Scan::new(path, csv_data_source, vec![]);
        assert_eq!(plan.schema(), schema);
    }

    #[test]
    fn test_children() {
        let (path, csv_data_source) = get_data_source();
        let plan = Scan::new(path, csv_data_source, vec![]);
        assert_eq!(plan.children().len(), 0);
    }

    #[test]
    fn test_to_string_without_projection() {
        let (path, csv_data_source) = get_data_source();
        let plan = Scan::new(path.clone(), csv_data_source, vec![]);
        assert_eq!(plan.to_string(), format!("Scan: {}; projection=None", path));
    }

    #[test]
    fn test_to_string_with_projection() {
        let (path, csv_data_source) = get_data_source();
        let plan = Scan::new(
            path.clone(),
            csv_data_source,
            vec!["c1".to_string(), "c2".to_string()],
        );
        assert_eq!(
            plan.to_string(),
            format!("Scan: {}; projection=[c1,c2]", path)
        );
    }
}
