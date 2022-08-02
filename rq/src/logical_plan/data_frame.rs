use super::{
    aggregate::Aggregate,
    expr::Expr,
    plan::{LogicalPlan, Plan},
    projection::Projection,
    selection::Selection,
};
use crate::data_types::schema::Schema;

#[derive(Clone)]
pub(crate) struct DataFrame {
    plan: Plan,
}

impl DataFrame {
    fn new(plan: Plan) -> Self {
        DataFrame { plan }
    }
    /// Apply a projection.
    fn project(&self, expr: Vec<Expr>) -> Self {
        let plan = Plan::Projection(Projection::new(Box::new(self.plan.clone()), expr));
        DataFrame::new(plan)
    }

    /// Apply a selection.
    fn filter(&self, expr: Expr) -> Self {
        let plan = Plan::Selection(Selection::new(Box::new(self.plan.clone()), expr));
        DataFrame::new(plan)
    }

    /// Apply an aggregation.
    fn aggregate(&self, group_by: Vec<Expr>, aggregates: Vec<Expr>) -> Self {
        let plan = Plan::Aggregate(Aggregate::new(
            Box::new(self.plan.clone()),
            group_by,
            aggregates,
        ));
        DataFrame::new(plan)
    }

    /// Returns the schema of the data that will be produced by this DataFrame.
    fn schema(&self) -> Schema {
        self.plan.schema()
    }

    /// Get the logical plan.
    fn logical_plan(&self) -> Plan {
        self.plan.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::DataFrame;
    use crate::{
        logical_plan::{
            expr_fn::{col, lit, max, min},
            plan::{LogicalPlan, Plan},
            scan::Scan,
        },
        util::get_data_source,
    };

    fn csv() -> (DataFrame) {
        let (_, csv_data_source) = get_data_source();
        let scan_plan = Scan::new(
            "data_frame_test".to_string(),
            csv_data_source,
            vec![
                "c1".to_string(),
                "c2".to_string(),
                "c3".to_string(),
                "c4".to_string(),
                "c5".to_string(),
                "c6".to_string(),
            ],
        );
        DataFrame::new(Plan::Scan(scan_plan))
    }

    #[test]
    fn test_new_data_frame() {
        let df = csv()
            .filter(col("c1").eq(lit(1)))
            .project(vec![
                col("c1"),
                col("c2"),
                col("c3"),
                col("c4"),
                col("c5"),
                (col("c6") * lit(1)).alias("bonus".to_string()),
            ])
            .filter(col("bonus").gt(lit(1)));
        let expected = "Selection: #bonus > 1
\tProjection: #c1,#c2,#c3,#c4,#c5,#c6 * 1 as bonus
\t\tSelection: #c1 = 1
\t\t\tScan: data_frame_test; projection=[c1,c2,c3,c4,c5,c6]
";
        assert_eq!(expected, df.plan.pretty(0));
    }

    #[test]
    fn test_aggregate_query() {
        let df = csv().aggregate(vec![col("c1")], vec![max(col("c2")), min(col("c3"))]);
        let expected = "Aggregate: groupExpr=#c1, aggregateExpr=MAX(#c2),MIN(#c3)
\tScan: data_frame_test; projection=[c1,c2,c3,c4,c5,c6]
";
        assert_eq!(expected, df.plan.pretty(0));
    }
}
