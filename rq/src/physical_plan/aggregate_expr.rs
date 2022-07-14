use super::expr::Expr;
use crate::logical_plan::expr::AggregateFunction;
use std::any::{Any, TypeId};

pub(crate) struct Accumulator {
    pub(crate) fun: AggregateFunction,
    pub(crate) value: Option<Box<dyn Any>>,
}

const I64_ID: TypeId = TypeId::of::<i64>();
const F32_ID: TypeId = TypeId::of::<f32>();
const F64_ID: TypeId = TypeId::of::<f64>();

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
    match l.type_id() {
        i64_id => l.downcast_ref::<i64>().unwrap() > r.downcast_ref::<i64>().unwrap(),
        f32_id => l.downcast_ref::<f32>().unwrap() > r.downcast_ref::<f32>().unwrap(),
        f64_id => l.downcast_ref::<f64>().unwrap() > r.downcast_ref::<f64>().unwrap(),
        _ => unreachable!(),
    }
}

fn is_min(l: &Box<dyn Any>, r: &Box<dyn Any>) -> bool {
    match l.type_id() {
        i64_id => l.downcast_ref::<i64>().unwrap() < r.downcast_ref::<i64>().unwrap(),
        f32_id => l.downcast_ref::<f32>().unwrap() < r.downcast_ref::<f32>().unwrap(),
        f64_id => l.downcast_ref::<f64>().unwrap() < r.downcast_ref::<f64>().unwrap(),
        _ => unreachable!(),
    }
}

fn sum(l: &mut Box<dyn Any>, r: &Box<dyn Any>) {
    match l.as_ref().type_id() {
        i64_id => *l.downcast_mut::<i64>().unwrap() += r.downcast_ref::<i64>().unwrap(),
        f32_id => *l.downcast_mut::<f32>().unwrap() += r.downcast_ref::<f32>().unwrap(),
        f64_id => *l.downcast_mut::<f64>().unwrap() += r.downcast_ref::<f64>().unwrap(),
        _ => unreachable!(),
    }
}
