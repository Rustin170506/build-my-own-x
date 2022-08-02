use crate::logical_plan::{
    expr::Expr,
    plan::{LogicalPlan, Plan},
};
use std::collections::HashSet;

pub(crate) trait OptimizerRule {
    fn optimize(&self, plan: Plan) -> Plan;
}

/// Extracts the set of columns that are referenced in the given query.
fn extract_columns(expr: &[Expr], input: &Plan, accum: &mut HashSet<String>) {
    expr.iter().for_each(|e| {
        extract_column(e, input, accum);
    })
}

fn extract_column(expr: &Expr, input: &Plan, accum: &mut HashSet<String>) {
    match expr {
        Expr::Column(c) => {
            accum.insert(c.name.clone());
        }
        Expr::ColumnIndex(cl) => {
            accum.insert(input.schema().fields[cl.index].name.clone());
        }
        Expr::BinaryExpr(e) => {
            extract_column(&e.left, input, accum);
            extract_column(&e.right, input, accum);
        }
        Expr::Alias(e) => extract_column(&e.expr, input, accum),
        Expr::Cast(c) => extract_column(&c.expr, input, accum),
        Expr::Literal(_) => {}
        Expr::Not(_) => {}
        Expr::ScalarFunction(_) => {}
        Expr::AggregateFunction(_) => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data_source::DataSource,
        logical_plan::{
            expr_fn::{and, col, lit},
            plan::Plan,
            scan::Scan,
        },
        util::get_data_source,
    };
    use std::collections::HashSet;

    #[test]
    fn test_extract_columns() {
        let mut accum: HashSet<String> = HashSet::new();

        let expr = vec![col("c1"), col("c2"), lit(1), and(col("c3"), col("c4"))];
        let (path, csv_data_source) = get_data_source();
        let schema = csv_data_source.get_schema().clone();
        let plan = Plan::Scan(Scan::new(path, csv_data_source, vec![]));
        assert_eq!(accum.len(), 0);
        extract_columns(&expr, &plan, &mut accum);
        assert_eq!(accum.len(), 4);
    }
}
