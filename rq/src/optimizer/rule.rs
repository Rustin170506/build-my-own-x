use crate::{
    data_source::DataSource,
    logical_plan::{
        aggregate::Aggregate,
        expr::Expr,
        plan::{LogicalPlan, Plan},
        projection::Projection,
        scan::Scan,
        selection::Selection,
    },
};
use std::collections::HashSet;

/// Rule for optimizing a logical plan.
pub trait OptimizerRule {
    fn optimize(plan: &Plan) -> Plan;
}

/// Rule for pushing down projections.
pub struct ProjectionPushDownRule;

impl ProjectionPushDownRule {
    fn push_down(plan: &Plan, column_names: &mut HashSet<String>) -> Plan {
        match plan {
            Plan::Projection(p) => {
                extract_columns(&p.exprs, &p.input, column_names);
                let input = ProjectionPushDownRule::push_down(&p.input, column_names);
                Plan::Projection(Projection::new(input, p.exprs.clone()))
            }
            Plan::Selection(s) => {
                extract_column(&s.expr, &s.input, column_names);
                let input = ProjectionPushDownRule::push_down(&s.input, column_names);
                Plan::Selection(Selection::new(input, s.expr.clone()))
            }
            Plan::Aggregate(a) => {
                extract_columns(&a.group_exprs, &a.input, column_names);
                extract_columns(&a.aggregate_exprs, &a.input, column_names);
                let input = ProjectionPushDownRule::push_down(&a.input, column_names);
                Plan::Aggregate(Aggregate::new(
                    input,
                    a.group_exprs.clone(),
                    a.aggregate_exprs.clone(),
                ))
            }
            Plan::Scan(s) => {
                let valid_filed_names = s
                    .data_source
                    .get_schema()
                    .fields
                    .iter()
                    .map(|f| f.name.clone())
                    .collect::<HashSet<String>>();

                let mut push_down = valid_filed_names
                    .iter()
                    .filter(|&n| column_names.contains(n))
                    .cloned()
                    .collect::<Vec<String>>();
                push_down.sort();
                Plan::Scan(Scan::new(s.path.clone(), s.data_source.clone(), push_down))
            }
        }
    }
}

impl OptimizerRule for ProjectionPushDownRule {
    fn optimize(plan: &Plan) -> Plan {
        ProjectionPushDownRule::push_down(plan, &mut HashSet::new())
    }
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
        Expr::AggregateFunction(a) => {
            extract_column(&a.expr, input, accum);
        }
        Expr::Literal(_) => {}
        Expr::Not(_) => {}
        Expr::ScalarFunction(_) => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data_source::DataSource,
        logical_plan::{
            data_frame::DataFrame,
            expr_fn::{and, col, count, lit, max, min},
            plan::Plan,
            scan::Scan,
        },
        test_util::get_primitive_field_data_source,
    };
    use std::collections::HashSet;

    fn csv() -> DataFrame {
        let (_, csv_data_source) = get_primitive_field_data_source();
        let scan_plan = Scan::new(
            "push_down_test".to_string(),
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
    fn test_extract_columns() {
        let mut accum: HashSet<String> = HashSet::new();

        let expr = vec![col("c1"), col("c2"), lit(1), and(col("c3"), col("c4"))];
        let (path, csv_data_source) = get_primitive_field_data_source();
        let _schema = csv_data_source.get_schema().clone();
        let plan = Plan::Scan(Scan::new(path, csv_data_source, vec![]));
        assert_eq!(accum.len(), 0);
        extract_columns(&expr, &plan, &mut accum);
        assert_eq!(accum.len(), 4);
    }

    #[test]
    fn test_projection_push_down() {
        let df = csv().aggregate(
            vec![col("c1")],
            vec![min(col("c2")), max(col("c2")), count(col("c2"))],
        );

        let optimized_plan = ProjectionPushDownRule::optimize(&df.logical_plan());
        assert_eq!(
            "Aggregate: groupExpr=#c1, aggregateExpr=MIN(#c2),MAX(#c2),COUNT(#c2)\n\tScan: push_down_test; projection=[c1,c2]\n",
            optimized_plan.pretty(0)
        );
    }

    #[test]
    fn test_projection_push_down_with_selection() {
        let df = csv()
            .filter(col("c1").eq(lit(1)))
            .project(vec![col("c1"), col("c2"), col("c3")]);

        let optimized_plan = ProjectionPushDownRule::optimize(&df.logical_plan());
        assert_eq!(
            "Projection: #c1,#c2,#c3\n\tSelection: #c1 = 1\n\t\tScan: push_down_test; projection=[c1,c2,c3]\n",
            optimized_plan.pretty(0)
        );
    }
}
