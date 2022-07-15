use super::expr::Expr;
use crate::logical_plan::expr::AggregateFunction;
use std::{
    any::{Any, TypeId},
    mem,
    ops::DerefMut,
};

/// Accumulator for aggregate functions.
pub(crate) struct Accumulator {
    pub(crate) fun: AggregateFunction,
    pub(crate) value: Option<Box<dyn Any>>,
}

impl Accumulator {
    pub(crate) fn new(fun: AggregateFunction) -> Self {
        Self { fun, value: None }
    }
}

impl Accumulator {
    pub(crate) fn accumulate(&mut self, value: Option<Box<dyn Any>>) {
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
                    _ => unreachable!(),
                }
            }
        }
    }

    pub(crate) fn final_value(&self) -> &Option<Box<dyn Any>> {
        &self.value
    }
}

fn is_max(l: &Box<dyn Any>, r: &Box<dyn Any>) -> bool {
    if l.is::<&i64>() {
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
    if l.is::<i64>() {
        let sum = *l.downcast_mut::<i64>().unwrap() + r.downcast_ref::<i64>().unwrap();
        mem::replace(l, Box::new(sum));
        return;
    }
    if l.is::<f32>() {
        let sum = *l.downcast_mut::<f32>().unwrap() + r.downcast_ref::<f32>().unwrap();
        mem::replace(l, Box::new(sum));
        return;
    }
    if l.is::<f64>() {
        let sum = *l.downcast_mut::<f64>().unwrap() + r.downcast_ref::<f64>().unwrap();
        mem::replace(l, Box::new(sum));
        return;
    }
    unreachable!()
}

/// AggregateExpr is an expression that aggregates a group of rows.
pub(crate) struct AggregateExpr {
    pub(crate) expr: Expr,
    pub(crate) fun: AggregateFunction,
}

impl AggregateExpr {
    pub(crate) fn new(expr: Expr, fun: AggregateFunction) -> Self {
        Self { expr, fun }
    }

    pub(crate) fn input_expr(&self) -> &Expr {
        &self.expr
    }

    pub(crate) fn create_accumulator(&self) -> Accumulator {
        Accumulator::new(self.fun.clone())
    }
}

impl ToString for AggregateExpr {
    fn to_string(&self) -> String {
        format!("{}({})", self.fun, self.expr.to_string())
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
    fn test_aggregate_expr_to_string() {
        let agg_expr = AggregateExpr::new(Expr::Column(Column::new(0)), AggregateFunction::Max);
        assert_eq!(agg_expr.to_string(), "MAX(#0)");
    }
}
