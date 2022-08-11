use super::{
    expr::{evaluate_from_values, Expr, PhysicalExpr},
    plan::{PhysicalPlan, Plan},
};
use crate::data_types::{
    arrow_field_array::ArrowFieldArray, column_array::ArrayRef, record_batch::RecordBatch,
    schema::Schema,
};
use anyhow::{anyhow, Error, Result};
use arrow::array::BooleanArray;
use std::{fmt::Display, rc::Rc};

/// Execute a selection.
pub(crate) struct SelectionExec {
    input: Box<Plan>,
    expr: Expr,
}

impl SelectionExec {
    pub(crate) fn new(input: Plan, expr: Expr) -> Self {
        Self {
            input: Box::new(input),
            expr,
        }
    }

    fn filter(&self, array: &ArrayRef, selection: &ArrayRef) -> Result<ArrayRef> {
        let mut values = vec![];
        for i in 0..selection.size() {
            if *selection.get_value(i)?.downcast_ref::<bool>().unwrap() {
                values.push(array.get_value(i)?);
            }
        }

        evaluate_from_values(&values, &array.get_type())
    }
}

impl PhysicalPlan for SelectionExec {
    fn schema(&self) -> Schema {
        self.input.schema()
    }

    fn execute(&self) -> Result<Box<dyn Iterator<Item = RecordBatch> + '_>> {
        let batch = self.input.execute()?;
        Ok(Box::new(
            batch
                .map(|b| {
                    let mut selection = &self.expr.evaluate(&b)?;
                    let schema = self.input.schema();
                    let filtered_fields = schema
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, _)| self.filter(b.field(i), selection))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok::<RecordBatch, Error>(RecordBatch::new(schema, filtered_fields))
                })
                .collect::<Result<Vec<RecordBatch>, _>>()?
                .into_iter(),
        ))
    }

    fn children(&self) -> Vec<&Plan> {
        vec![&self.input]
    }
}

impl Display for SelectionExec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelectionExec: {}", self.expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data_source::{csv_data_source::CsvDataSource, Source},
        data_types::{
            column_array::DataType,
            schema::{Field, Schema},
        },
        logical_plan::expr::Operator,
        physical_plan::{
            expr::{BinaryExpr, Column, ScalarValue},
            scan::ScanExec,
        },
    };
    use std::path::PathBuf;

    #[test]
    fn test_selection_execute() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/f32_field.csv");
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
        let scan = ScanExec::new(Source::Csv(csv_data_source), vec!["c5".to_string()]);
        let filter = Expr::BinaryExpr(BinaryExpr::new(
            Operator::LtEq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Float32(1.1))),
        ));

        let selection = SelectionExec::new(Plan::Scan(scan), filter);
        let result = selection.execute().unwrap().next().unwrap();
        let field = result.field(0);
        assert_eq!(field.get_type(), DataType::Float32);
        assert_eq!(field.size(), 2);
        assert_eq!(
            field.get_value(0).unwrap().downcast_ref::<f32>().unwrap(),
            &1.0
        );
        assert_eq!(
            field.get_value(1).unwrap().downcast_ref::<f32>().unwrap(),
            &1.1
        );
    }

    #[test]
    fn test_selection_display() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/f32_field.csv");
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
        let scan = ScanExec::new(Source::Csv(csv_data_source), vec!["c5".to_string()]);
        let filter = Expr::BinaryExpr(BinaryExpr::new(
            Operator::LtEq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Float32(1.1))),
        ));
        let selection = SelectionExec::new(Plan::Scan(scan), filter);
        assert_eq!(selection.to_string(), "SelectionExec: #0 <= 1.1");
    }
}
