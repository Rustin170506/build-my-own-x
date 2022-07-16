use std::fmt::Display;

use super::{
    expr::Expr,
    plan::{LogicalPlan, Plan},
};
use crate::data_types::schema::Schema;

/// Logical plan representing a selection (a.k.a. filter) against an input.
#[derive(Clone)]
pub(crate) struct Selection {
    pub(crate) input: Box<Plan>,
    pub(crate) expr: Expr,
}

impl LogicalPlan for Selection {
    fn schema(&self) -> Schema {
        self.input.schema()
    }

    fn children(&self) -> Vec<Plan> {
        vec![self.input.as_ref().clone()]
    }
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Selection: {}", self.expr)
    }
}

impl Selection {
    pub(crate) fn new(input: Box<Plan>, expr: Expr) -> Self {
        Selection { input, expr }
    }
}

#[cfg(test)]
mod tests {
    use crate::logical_plan::{
        expr_fn::col, plan::Plan, scan::Scan, selection::Selection, util::get_data_source,
    };

    #[test]
    fn test_display() {
        let (path, csv_data_source) = get_data_source();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = col("c1");
        let plan = Selection::new(Box::new(Plan::Scan(scan_plan)), c1);
        assert_eq!(plan.to_string(), "Selection: #c1");
    }
}
