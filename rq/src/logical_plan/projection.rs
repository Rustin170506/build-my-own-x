use std::fmt::Display;

use super::{
    expr::{Expr, LogicalExpr},
    plan::{LogicalPlan, Plan},
};
use crate::data_types::schema::Schema;

#[derive(Clone)]
pub(crate) struct Projection {
    pub(crate) input: Box<Plan>,
    pub(crate) exprs: Vec<Expr>,
}

impl LogicalPlan for Projection {
    fn schema(&self) -> Schema {
        let fields = self
            .exprs
            .iter()
            .map(|e| e.to_field(&self.input).unwrap())
            .collect();
        Schema::new(fields)
    }

    fn children(&self) -> Vec<Plan> {
        vec![self.input.as_ref().clone()]
    }
}

impl Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Projection: {}",
            self.exprs
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Projection {
    pub(crate) fn new(input: Plan, exprs: Vec<Expr>) -> Self {
        Projection {
            input: Box::new(input),
            exprs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Projection;
    use crate::{
        data_source::DataSource,
        logical_plan::{
            expr_fn::col,
            plan::{LogicalPlan, Plan},
            scan::Scan,
        },
        test_util::get_primitive_field_data_source,
    };

    #[test]
    fn test_test_schema() {
        let (path, csv_data_source) = get_primitive_field_data_source();
        let schema = csv_data_source.get_schema().clone();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = col("c1");
        let c2 = col("c2");
        let c3 = col("c3");
        let c4 = col("c4");
        let c5 = col("c5");
        let c6 = col("c6");
        let plan = Projection::new(Plan::Scan(scan_plan), vec![c1, c2, c3, c4, c5, c6]);
        assert_eq!(plan.schema(), schema);
    }

    #[test]
    fn test_children() {
        let (path, csv_data_source) = get_primitive_field_data_source();
        let schema = csv_data_source.get_schema().clone();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = col("c1");
        let c2 = col("c2");
        let c3 = col("c3");
        let c4 = col("c4");
        let c5 = col("c5");
        let c6 = col("c6");
        let plan = Projection::new(Plan::Scan(scan_plan), vec![c1, c2, c3, c4, c5, c6]);
        assert_eq!(plan.children().len(), 1);
        assert_eq!(plan.children()[0].schema(), schema);
    }

    #[test]
    fn test_display() {
        let (path, csv_data_source) = get_primitive_field_data_source();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let c1 = col("c1");
        let c2 = col("c2");
        let c3 = col("c3");
        let c4 = col("c4");
        let c5 = col("c5");
        let c6 = col("c6");
        let plan = Projection::new(Plan::Scan(scan_plan), vec![c1, c2, c3, c4, c5, c6]);
        assert_eq!(plan.to_string(), "Projection: #c1,#c2,#c3,#c4,#c5,#c6");
    }
}
