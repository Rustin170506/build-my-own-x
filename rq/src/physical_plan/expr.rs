use std::{fmt::format, rc::Rc};

use arrow::datatypes::DataType;

use crate::data_types::{
    column_array::ArrayRef, literal_value_array::LiteralValueArray, record_batch::RecordBatch,
};

/// Physical representation of an expression.
pub(crate) trait PhysicalExpr: ToString {
    fn evaluate(&self, input: RecordBatch) -> ArrayRef;
}

pub(crate) struct Column {
    pub(crate) i: usize,
}

impl PhysicalExpr for Column {
    fn evaluate(&self, input: RecordBatch) -> ArrayRef {
        input.field(self.i).clone()
    }
}

impl ToString for Column {
    fn to_string(&self) -> String {
        format!("#{}", self.i)
    }
}

pub(crate) enum ScalarValue {
    String(String),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

impl PhysicalExpr for ScalarValue {
    fn evaluate(&self, input: RecordBatch) -> ArrayRef {
        match self {
            ScalarValue::String(s) => Rc::new(LiteralValueArray::new(
                DataType::Utf8,
                s.clone(),
                input.row_count(),
            )),
            ScalarValue::Int64(i) => Rc::new(LiteralValueArray::new(
                DataType::Int64,
                *i,
                input.row_count(),
            )),
            ScalarValue::Float32(f) => Rc::new(LiteralValueArray::new(
                DataType::Float32,
                *f,
                input.row_count(),
            )),
            ScalarValue::Float64(f) => Rc::new(LiteralValueArray::new(
                DataType::Float64,
                *f,
                input.row_count(),
            )),
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
