use super::{
    expr::{evaluate_from_values, Expr, PhysicalExpr},
    plan::{PhysicalPlan, Plan},
};
use crate::data_types::{
    arrow_field_array::ArrowFieldArray, column_array::ArrayRef, record_batch::RecordBatch,
    schema::Schema,
};
use anyhow::Result;
use arrow::array::BooleanArray;
use std::{fmt::Display, rc::Rc};

/// Execute a selection.
pub(crate) struct Selection {
    input: Plan,
    expr: Expr,
}

impl Selection {
    pub(crate) fn new(input: Plan, expr: Expr) -> Self {
        Self { input, expr }
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

impl PhysicalPlan for Selection {
    fn schema(&self) -> Schema {
        self.input.schema()
    }

    fn execute(&self) -> Result<Box<dyn Iterator<Item = RecordBatch> + '_>> {
        let batch = self.input.execute()?;
        Ok(Box::new(batch.map(|b| {
            let mut selection = &self.expr.evaluate(&b).expect("evaluate expr failed");
            let schema = self.input.schema();
            let fitered_fields = schema
                .fields
                .iter()
                .enumerate()
                .map(|(i, _)| self.filter(b.field(i), selection).expect("filter failed"))
                .collect();
            RecordBatch::new(schema, fitered_fields)
        })))
    }

    fn children(&self) -> Vec<&Plan> {
        vec![&self.input]
    }
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelectionExec: {}", self.expr)
    }
}
