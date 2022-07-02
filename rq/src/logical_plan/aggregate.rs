use super::{
    expr::{Expr, LogicalExpr},
    plan::{LogicalPlan, Plan},
};
use crate::data_types::schema::Schema;

/// Logical plan representing an aggregate query against an input.
#[derive(Clone)]
pub(crate) struct Aggregate {
    pub(crate) input: Box<Plan>,
    pub(crate) group_exprs: Vec<Expr>,
    pub(crate) aggregate_exprs: Vec<Expr>,
}

impl LogicalPlan for Aggregate {
    fn schema(&self) -> Schema {
        let fields = self
            .group_exprs
            .iter()
            .chain(self.aggregate_exprs.iter())
            .map(|e| e.to_field(&self.input).unwrap())
            .collect();
        Schema::new(fields)
    }

    fn children(&self) -> Vec<Plan> {
        vec![self.input.as_ref().clone()]
    }
}

impl ToString for Aggregate {
    fn to_string(&self) -> String {
        let group_exprs = self
            .group_exprs
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let aggregate_exprs = self
            .aggregate_exprs
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");

        format!(
            "Aggregate: groupExpr={}, aggregateExpr={}",
            group_exprs, aggregate_exprs,
        )
    }
}

impl Aggregate {
    pub(crate) fn new(
        input: Box<Plan>,
        group_exprs: Vec<Expr>,
        aggregate_exprs: Vec<Expr>,
    ) -> Self {
        Aggregate {
            input,
            group_exprs,
            aggregate_exprs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Aggregate;
    use crate::logical_plan::{
        expr_fn::{col, max},
        plan::Plan,
        scan::Scan,
        util::get_data_source,
    };

    #[test]
    fn test_to_string() {
        let (path, csv_data_source) = get_data_source();
        let scan_plan = Scan::new(path, csv_data_source, vec![]);
        let col1 = col("c1");
        let group_exprs = vec![col1.clone()];
        let aggregate_exprs = vec![max(col1)];
        let agg = Aggregate::new(
            Box::new(Plan::Scan(scan_plan)),
            group_exprs,
            aggregate_exprs,
        );
        assert_eq!(
            agg.to_string(),
            "Aggregate: groupExpr=#c1, aggregateExpr=MAX(#c1)"
        );
    }
}
