use crate::{
    data_types::{
        arrow_field_array::ArrowFieldArray, column_array::ArrayRef,
        literal_value_array::LiteralValueArray, record_batch::RecordBatch,
    },
    logical_plan::expr::Operator,
};
use anyhow::Result;
use arrow::{array::Int64Array, datatypes::DataType};
use std::{any::Any, rc::Rc};

/// Physical representation of an expression.
pub(crate) trait PhysicalExpr: ToString {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef>;
}

pub(crate) enum Expr {
    Column(Column),
    Literal(ScalarValue),
    BinaryExpr(BinaryExpr),
}

impl PhysicalExpr for Expr {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        match self {
            Expr::Column(column) => column.evaluate(input),
            Expr::Literal(literal) => literal.evaluate(input),
            Expr::BinaryExpr(binary_expr) => binary_expr.evaluate(input),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Column(column) => column.to_string(),
            Expr::Literal(literal) => literal.to_string(),
            Expr::BinaryExpr(binary_expr) => binary_expr.to_string(),
        }
    }
}

pub(crate) struct Column {
    pub(crate) i: usize,
}

impl PhysicalExpr for Column {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        Ok(input.field(self.i).clone())
    }
}

impl ToString for Column {
    fn to_string(&self) -> String {
        format!("#{}", self.i)
    }
}

/// Represents a dynamically typed single value.
pub(crate) enum ScalarValue {
    String(String),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

impl PhysicalExpr for ScalarValue {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        match self {
            ScalarValue::String(s) => Ok(Rc::new(LiteralValueArray::new(
                DataType::Utf8,
                s.clone(),
                input.row_count(),
            ))),
            ScalarValue::Int64(i) => Ok(Rc::new(LiteralValueArray::new(
                DataType::Int64,
                *i,
                input.row_count(),
            ))),
            ScalarValue::Float32(f) => Ok(Rc::new(LiteralValueArray::new(
                DataType::Float32,
                *f,
                input.row_count(),
            ))),
            ScalarValue::Float64(f) => Ok(Rc::new(LiteralValueArray::new(
                DataType::Float64,
                *f,
                input.row_count(),
            ))),
        }
    }
}

impl ToString for ScalarValue {
    fn to_string(&self) -> String {
        match self {
            ScalarValue::String(s) => format!("'{}'", s),
            ScalarValue::Int64(i) => format!("{}", i),
            ScalarValue::Float32(f) => format!("{}", f),
            ScalarValue::Float64(f) => format!("{}", f),
        }
    }
}

pub(crate) struct BinaryExpr {
    pub(crate) op: Operator,
    pub(crate) left: Box<Expr>,
    pub(crate) right: Box<Expr>,
}

impl PhysicalExpr for BinaryExpr {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        let left = self.left.evaluate(input)?;
        let right = self.right.evaluate(input)?;
        assert!(left.get_type() == right.get_type());
        let arrow_type = left.get_type();
        let mut array = vec![];
        match self.op {
            Operator::Add => {
                for i in 0..left.size() {
                    let value = add(left.get_value(i)?, right.get_value(i)?, left.get_type());
                    array.push(value);
                }
            }
            _ => unimplemented!(),
        }

        build_from_values(&array, arrow_type)
    }
}

impl ToString for BinaryExpr {
    fn to_string(&self) -> String {
        todo!()
    }
}

fn build_from_values(array: &[Box<dyn Any>], data_type: DataType) -> Result<ArrayRef> {
    match data_type {
        DataType::Int64 => {
            let arrow_array = Int64Array::from(
                array
                    .iter()
                    .map(|v| *v.downcast_ref::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            );
            Ok(Rc::new(ArrowFieldArray::new(Box::new(arrow_array))))
        }
        _ => unimplemented!(),
    }
}

fn add(l: Box<dyn Any>, r: Box<dyn Any>, data_type: DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Int64 => {
            let l = l.downcast_ref::<i64>().unwrap();
            let r = r.downcast_ref::<i64>().unwrap();
            Box::new(*l + *r)
        }
        DataType::Float32 => {
            let l = l.downcast_ref::<f32>().unwrap();
            let r = r.downcast_ref::<f32>().unwrap();
            Box::new(*l + *r)
        }
        DataType::Float64 => {
            let l = l.downcast_ref::<f64>().unwrap();
            let r = r.downcast_ref::<f64>().unwrap();
            Box::new(*l + *r)
        }
        _ => unimplemented!(),
    }
}
