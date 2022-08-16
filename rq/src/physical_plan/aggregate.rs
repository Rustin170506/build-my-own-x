use std::{any::Any, fmt::Display};

use super::expr::Expr;
use crate::logical_plan::expr::AggregateFunction;

/// Accumulator for aggregate functions.
pub struct Accumulator {
    pub fun: AggregateFunction,
    pub value: Option<Box<dyn Any>>,
}

impl Accumulator {
    pub fn new(fun: AggregateFunction) -> Self {
        Self { fun, value: None }
    }
}

impl Accumulator {
    pub fn accumulate(&mut self, value: Option<Box<dyn Any>>) {
        if let Some(value) = value {
            if self.value.is_none() {
                self.value = Some(value);
            } else {
                match self.fun {
                    AggregateFunction::Sum => sum(self.value.as_mut().unwrap(), &value),
                    AggregateFunction::Min => {
                        if is_min(&value, self.value.as_ref().unwrap()) {
                            self.value = Some(value);
                        }
                    }
                    AggregateFunction::Max => {
                        if is_max(&value, self.value.as_ref().unwrap()) {
                            self.value = Some(value);
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }

    pub fn final_value(&self) -> &Option<Box<dyn Any>> {
        &self.value
    }
}

fn is_max(l: &Box<dyn Any>, r: &Box<dyn Any>) -> bool {
    if l.is::<i32>() {
        return l.downcast_ref::<i32>().unwrap() > r.downcast_ref::<i32>().unwrap();
    }
    if l.is::<i64>() {
        return l.downcast_ref::<i64>().unwrap() > r.downcast_ref::<i64>().unwrap();
    }
    if l.is::<f32>() {
        return l.downcast_ref::<f32>().unwrap() > r.downcast_ref::<f32>().unwrap();
    }
    if l.is::<f64>() {
        return l.downcast_ref::<f64>().unwrap() > r.downcast_ref::<f64>().unwrap();
    }
    unreachable!()
}

fn is_min(l: &Box<dyn Any>, r: &Box<dyn Any>) -> bool {
    if l.is::<i32>() {
        return l.downcast_ref::<i32>().unwrap() < r.downcast_ref::<i32>().unwrap();
    }
    if l.is::<i64>() {
        return l.downcast_ref::<i64>().unwrap() < r.downcast_ref::<i64>().unwrap();
    }
    if l.is::<f32>() {
        return l.downcast_ref::<f32>().unwrap() < r.downcast_ref::<f32>().unwrap();
    }
    if l.is::<f64>() {
        return l.downcast_ref::<f64>().unwrap() < r.downcast_ref::<f64>().unwrap();
    }
    unreachable!()
}

fn sum(l: &mut Box<dyn Any>, r: &Box<dyn Any>) {
    if l.is::<i32>() {
        let sum = *l.downcast_mut::<i32>().unwrap() + r.downcast_ref::<i32>().unwrap();
        *l = Box::new(sum);
        return;
    }
    if l.is::<i64>() {
        let sum = *l.downcast_mut::<i64>().unwrap() + r.downcast_ref::<i64>().unwrap();
        *l = Box::new(sum);
        return;
    }
    if l.is::<f32>() {
        let sum = *l.downcast_mut::<f32>().unwrap() + r.downcast_ref::<f32>().unwrap();
        *l = Box::new(sum);
        return;
    }
    if l.is::<f64>() {
        let sum = *l.downcast_mut::<f64>().unwrap() + r.downcast_ref::<f64>().unwrap();
        *l = Box::new(sum);
        return;
    }
    unreachable!()
}

/// AggregateExpr is an expression that aggregates a group of rows.
pub struct AggregateExpr {
    pub expr: Expr,
    pub fun: AggregateFunction,
}

impl AggregateExpr {
    pub fn new(expr: Expr, fun: AggregateFunction) -> Self {
        Self { expr, fun }
    }

    pub fn input_expr(&self) -> &Expr {
        &self.expr
    }

    pub fn create_accumulator(&self) -> Accumulator {
        Accumulator::new(self.fun.clone())
    }
}

impl Display for AggregateExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.fun, self.expr)
    }
}

#[cfg(test)]
mod tests {
    use super::{Accumulator, AggregateExpr};
    use crate::{
        logical_plan::expr::AggregateFunction,
        physical_plan::expr::{Column, Expr},
    };

    #[test]
    fn test_max_accumulator_i32() {
        let mut acc = Accumulator::new(AggregateFunction::Max);
        acc.accumulate(Some(Box::new(1i32)));
        assert!(acc.final_value().is_some());
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap(),
            &1
        );
        acc.accumulate(Some(Box::new(10i32)));
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap(),
            &10
        );
    }

    #[test]
    fn test_max_accumulator_i64() {
        let mut acc = Accumulator::new(AggregateFunction::Max);
        acc.accumulate(Some(Box::new(1i64)));
        assert!(acc.final_value().is_some());
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &1
        );
        acc.accumulate(Some(Box::new(10i64)));
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &10
        );
    }

    #[test]
    fn test_max_accumulator_f32() {
        let mut acc = Accumulator::new(AggregateFunction::Max);
        acc.accumulate(Some(Box::new(1f32)));
        assert!(acc.final_value().is_some());
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<f32>()
                .unwrap(),
            &1.0
        );
        acc.accumulate(Some(Box::new(10f32)));
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<f32>()
                .unwrap(),
            &10.0
        );
    }

    #[test]
    fn test_min_accumulator() {
        let mut acc = Accumulator::new(AggregateFunction::Min);
        acc.accumulate(Some(Box::new(1i64)));
        assert!(acc.final_value().is_some());
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &1
        );
        acc.accumulate(Some(Box::new(10i64)));
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &1
        );
    }

    #[test]
    fn test_sum_accumulator() {
        let mut acc = Accumulator::new(AggregateFunction::Sum);
        acc.accumulate(Some(Box::new(1i64)));
        assert!(acc.final_value().is_some());
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &1
        );
        acc.accumulate(Some(Box::new(10i64)));
        assert_eq!(
            acc.final_value()
                .as_ref()
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &11
        );
    }

    #[test]
    fn test_aggregate_expr_display() {
        let agg_expr = AggregateExpr::new(Expr::Column(Column::new(0)), AggregateFunction::Max);
        assert_eq!(agg_expr.to_string(), "MAX(#0)");
    }
}
