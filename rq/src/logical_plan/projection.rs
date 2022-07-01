use super::{logical_expr::LogicalExpr, plan::LogicalPlan};
use crate::data_types::schema::Schema;

pub(crate) struct Projection<'p> {
    pub(crate) input: &'p dyn LogicalPlan,
    pub(crate) expr: Vec<&'p dyn LogicalExpr>,
}

impl LogicalPlan for Projection<'_> {
    fn schema(&self) -> Schema {
        let fields = self
            .expr
            .iter()
            .map(|&e| e.to_field(self.input).unwrap())
            .collect();
        Schema::new(fields)
    }

    fn children(&self) -> Vec<&dyn LogicalPlan> {
        return vec![self.input];
    }
}

impl ToString for Projection<'_> {
    fn to_string(&self) -> String {
        format!(
            "Projection: {}",
            self.expr
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl<'p> Projection<'p> {
    pub(crate) fn new(input: &'p dyn LogicalPlan, expr: Vec<&'p dyn LogicalExpr>) -> Self {
        Projection { input, expr }
    }
}

#[cfg(test)]
mod tests {
    use super::Projection;
    use crate::{
        data_source::{csv_data_source::CsvDataSource, DataSource},
        data_types::schema::{Field, Schema},
        logical_plan::{expr_fn::col, plan::LogicalPlan, scan::Scan},
    };
    use arrow::datatypes::DataType;
    use std::path::PathBuf;

    pub(crate) fn get_data_source() -> (String, Box<dyn DataSource>) {
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
    fn test_test_schema() {
        let (path, csv_data_source) = get_data_source();
        let schema = csv_data_source.get_schema().clone();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = &col("c1");
        let c2 = &col("c2");
        let c3 = &col("c3");
        let c4 = &col("c4");
        let c5 = &col("c5");
        let c6 = &col("c6");
        let plan = Projection::new(&scan_plan, vec![c1, c2, c3, c4, c5, c6]);
        assert_eq!(plan.schema(), schema);
    }

    #[test]
    fn test_children() {
        let (path, csv_data_source) = get_data_source();
        let schema = csv_data_source.get_schema().clone();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = &col("c1");
        let c2 = &col("c2");
        let c3 = &col("c3");
        let c4 = &col("c4");
        let c5 = &col("c5");
        let c6 = &col("c6");
        let plan = Projection::new(&scan_plan, vec![c1, c2, c3, c4, c5, c6]);
        assert_eq!(plan.children().len(), 1);
        assert_eq!(plan.children()[0].schema(), schema);
    }

    #[test]
    fn test_to_string() {
        let (path, csv_data_source) = get_data_source();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = &col("c1");
        let c2 = &col("c2");
        let c3 = &col("c3");
        let c4 = &col("c4");
        let c5 = &col("c5");
        let c6 = &col("c6");
        let plan = Projection::new(&scan_plan, vec![c1, c2, c3, c4, c5, c6]);
        assert_eq!(plan.to_string(), "Projection: #c1,#c2,#c3,#c4,#c5,#c6");
    }
}
