use crate::{
    data_types::schema::{Field, Schema},
    logical_plan::{
        expr::{
            AggregateFunction, Expr as LogicalExpr, LogicalExpr as _, Operator,
            ScalarValue as LogicalScalarValue,
        },
        plan::{LogicalPlan as _, Plan as LogicalPlan},
        projection,
    },
    physical_plan::{
        aggregate_expr::AggregateExpr,
        expr::{BinaryExpr, Column, Expr as PhysicalExpr, ScalarValue as PhysicalScalarValue},
        hash::HashExec,
        plan::Plan as PhysicalPlan,
        projection::ProjectionExec,
        scan::ScanExec,
        selection::SelectionExec,
    },
};
use anyhow::{anyhow, Error, Result};

/// The query planner creates a physical query plan from a logical query plan.
pub(crate) struct QueryPlanner {}

impl QueryPlanner {
    /// Create a physical plan from a logical plan.
    fn create_physical_plan(&self, plan: &LogicalPlan) -> Result<PhysicalPlan> {
        match plan {
            LogicalPlan::Scan(scan) => {
                let scan = ScanExec::new(*scan.data_source.clone(), scan.projection.clone());
                Ok(PhysicalPlan::Scan(scan))
            }
            LogicalPlan::Projection(projection) => {
                let input = self.create_physical_plan(projection.input.as_ref())?;
                let projection_exprs = projection
                    .exprs
                    .iter()
                    .map(|expr| self.create_physical_expr(expr, projection.input.as_ref()))
                    .collect::<Result<Vec<PhysicalExpr>, _>>()?;
                let projection_schema = Schema::new(
                    projection
                        .exprs
                        .iter()
                        .map(|expr| expr.to_field(projection.input.as_ref()))
                        .collect::<Result<Vec<Field>, _>>()?,
                );
                let projection_exec =
                    ProjectionExec::new(input, projection_schema, projection_exprs);
                Ok(PhysicalPlan::Projection(projection_exec))
            }
            LogicalPlan::Selection(s) => {
                let input = self.create_physical_plan(s.input.as_ref())?;
                let filer_expr = self.create_physical_expr(&s.expr, s.input.as_ref())?;
                let selection_exec = SelectionExec::new(input, filer_expr);
                Ok(PhysicalPlan::Selection(selection_exec))
            }
            LogicalPlan::Aggregate(a) => {
                let input = self.create_physical_plan(a.input.as_ref())?;
                let group_exprs = a
                    .group_exprs
                    .iter()
                    .map(|expr| self.create_physical_expr(expr, a.input.as_ref()))
                    .collect::<Result<Vec<PhysicalExpr>, _>>()?;
                let aggr_exprs = a
                    .aggregate_exprs
                    .iter()
                    .map(|expr| match expr {
                        LogicalExpr::AggregateFunction(agg) => {
                            let expr =
                                self.create_physical_expr(agg.expr.as_ref(), a.input.as_ref())?;
                            Ok::<_, Error>(AggregateExpr::new(expr, agg.fun.clone()))
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Result<Vec<AggregateExpr>, _>>()?;
                let hash_exec = HashExec::new(input, a.schema(), group_exprs, aggr_exprs);
                Ok(PhysicalPlan::Hash(hash_exec))
            }
        }
    }

    /// Create a physical expression from a logical expression.
    fn create_physical_expr(
        &self,
        expr: &LogicalExpr,
        input: &LogicalPlan,
    ) -> Result<PhysicalExpr> {
        match expr {
            LogicalExpr::Column(c) => {
                let index = input.schema().fields.iter().position(|f| f.name == c.name);
                match index {
                    Some(index) => {
                        let column = Column::new(index);
                        Ok(PhysicalExpr::Column(column))
                    }
                    None => Err(anyhow!("No column named {}", c.name)),
                }
            }
            LogicalExpr::ColumnIndex(cl) => {
                let column = Column::new(cl.index);
                Ok(PhysicalExpr::Column(column))
            }
            LogicalExpr::Literal(l) => {
                let l = match l {
                    LogicalScalarValue::String(s) => PhysicalScalarValue::String(s.clone()),
                    LogicalScalarValue::Int64(i) => PhysicalScalarValue::Int64(*i),
                    LogicalScalarValue::Float32(f) => PhysicalScalarValue::Float32(*f),
                    LogicalScalarValue::Float64(b) => PhysicalScalarValue::Float64(*b),
                };
                Ok(PhysicalExpr::Literal(l))
            }
            LogicalExpr::Cast(_) => todo!(),
            LogicalExpr::BinaryExpr(b) => {
                let l = Box::new(self.create_physical_expr(b.left.as_ref(), input)?);
                let r = Box::new(self.create_physical_expr(b.right.as_ref(), input)?);
                let binary_expr = BinaryExpr::new(b.op, l, r);
                Ok(PhysicalExpr::BinaryExpr(binary_expr))
            }
            LogicalExpr::Alias(a) => {
                // Note that there is no physical expression for an alias since the alias
                // only affects the name using in the planning phase and not how the aliased
                // expression is executed
                return self.create_physical_expr(a.expr.as_ref(), input);
            }
            LogicalExpr::Not(_) => unreachable!(),
            LogicalExpr::ScalarFunction(s) => unreachable!(),
            LogicalExpr::AggregateFunction(_) => unreachable!(),
        }
    }
}
