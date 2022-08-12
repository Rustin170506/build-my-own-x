use crate::{
    data_types::{
        arrow_field_array::ArrowFieldArray,
        column_array::{ArrayRef, DataType},
        literal_value_array::LiteralValueArray,
        record_batch::RecordBatch,
    },
    logical_plan::expr::Operator,
};
use anyhow::{Error, Result};
use arrow::array::{BooleanArray, Int32Array, Int64Array};
use ordered_float::OrderedFloat;
use std::{any::Any, fmt::Display, rc::Rc};

/// Physical representation of an expression.
pub(crate) trait PhysicalExpr: Display {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef>;
}

pub(crate) enum Expr {
    Column(Column),
    Literal(ScalarValue),
    BinaryExpr(BinaryExpr),
    Cast(Cast),
}

impl PhysicalExpr for Expr {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        match self {
            Expr::Column(column) => column.evaluate(input),
            Expr::Literal(literal) => literal.evaluate(input),
            Expr::BinaryExpr(binary_expr) => binary_expr.evaluate(input),
            Expr::Cast(cast) => cast.evaluate(input),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Column(column) => column.fmt(f),
            Expr::Literal(literal) => literal.fmt(f),
            Expr::BinaryExpr(binary_expr) => binary_expr.fmt(f),
            Expr::Cast(cast) => cast.fmt(f),
        }
    }
}

pub(crate) struct Column {
    pub(crate) i: usize,
}

impl Column {
    pub(crate) fn new(i: usize) -> Self {
        Self { i }
    }
}

impl PhysicalExpr for Column {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        Ok(input.field(self.i).clone())
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.i)
    }
}

/// Represents a dynamically typed single value.
pub(crate) enum ScalarValue {
    String(String),
    Int32(i32),
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
            ScalarValue::Int32(i) => Ok(Rc::new(LiteralValueArray::new(
                DataType::Int32,
                *i,
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

impl Display for ScalarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarValue::String(s) => write!(f, "'{}'", s),
            ScalarValue::Int32(i) => write!(f, "{}", i),
            ScalarValue::Int64(i) => write!(f, "{}", i),
            ScalarValue::Float32(fv) => write!(f, "{}", fv),
            ScalarValue::Float64(fv) => write!(f, "{}", fv),
        }
    }
}

/// For binary expressions we need to evaluate the left and right input expressions
/// and then evaluate the specific binary operator against those input values.
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
        let mut vals = vec![];
        match self.op {
            Operator::Add => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::math_binary_op!(&l,&r, &arrow_type,+);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Subtract => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::math_binary_op!(&l,&r, &arrow_type,-);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Multiply => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::math_binary_op!(&l,&r, &arrow_type,*);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Divide => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::math_binary_op!(&l,&r, &arrow_type,/);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Modulus => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::math_binary_op!(&l,&r, &arrow_type,%);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::And => {
                for i in 0..left.size() {
                    let value = and(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::Or => {
                for i in 0..left.size() {
                    let value = or(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::Eq => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::bool_binary_op!(&l, &r, &arrow_type, eq);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::Neq => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::bool_binary_op!(&l, &r, &arrow_type, ne);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::Lt => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::bool_binary_op!(&l, &r, &arrow_type, lt);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::LtEq => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::bool_binary_op!(&l, &r, &arrow_type, le);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::Gt => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::bool_binary_op!(&l, &r, &arrow_type, gt);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            Operator::GtEq => {
                for i in 0..left.size() {
                    let l = left.get_value(i)?;
                    let r = right.get_value(i)?;
                    let value = crate::bool_binary_op!(&l, &r, &arrow_type, ge);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
        }
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            Operator::Add => write!(f, "{} + {}", self.left, self.right),
            Operator::Subtract => {
                write!(f, "{} - {}", self.left, self.right)
            }
            Operator::Multiply => {
                write!(f, "{} * {}", self.left, self.right)
            }
            Operator::Divide => write!(f, "{} / {}", self.left, self.right),
            Operator::Modulus => {
                write!(f, "{} % {}", self.left, self.right)
            }
            Operator::And => write!(f, "{} AND {}", self.left, self.right),
            Operator::Or => write!(f, "{} OR {}", self.left, self.right),
            Operator::Eq => write!(f, "{} == {}", self.left, self.right),
            Operator::Neq => write!(f, "{} != {}", self.left, self.right),
            Operator::Lt => write!(f, "{} < {}", self.left, self.right),
            Operator::LtEq => write!(f, "{} <= {}", self.left, self.right),
            Operator::Gt => write!(f, "{} > {}", self.left, self.right),
            Operator::GtEq => write!(f, "{} >= {}", self.left, self.right),
        }
    }
}

impl BinaryExpr {
    pub(crate) fn new(op: Operator, left: Box<Expr>, right: Box<Expr>) -> Self {
        Self { op, left, right }
    }
}

// Build the arrow array from the values.
pub(crate) fn evaluate_from_values(
    array: &[Box<dyn Any>],
    data_type: &DataType,
) -> Result<ArrayRef> {
    match data_type {
        DataType::Int32 => {
            let arrow_array = Int32Array::from(
                array
                    .iter()
                    .map(|v| *v.downcast_ref::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
            Ok(Rc::new(ArrowFieldArray::new(Box::new(arrow_array))))
        }
        DataType::Int64 => {
            let arrow_array = Int64Array::from(
                array
                    .iter()
                    .map(|v| *v.downcast_ref::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            );
            Ok(Rc::new(ArrowFieldArray::new(Box::new(arrow_array))))
        }
        DataType::Float32 => {
            let arrow_array = arrow::array::Float32Array::from(
                array
                    .iter()
                    .map(|v| *v.downcast_ref::<f32>().unwrap())
                    .collect::<Vec<f32>>(),
            );
            Ok(Rc::new(ArrowFieldArray::new(Box::new(arrow_array))))
        }
        DataType::Float64 => {
            let arrow_array = arrow::array::Float64Array::from(
                array
                    .iter()
                    .map(|v| *v.downcast_ref::<f64>().unwrap())
                    .collect::<Vec<f64>>(),
            );
            Ok(Rc::new(ArrowFieldArray::new(Box::new(arrow_array))))
        }
        DataType::Boolean => {
            let arrow_array = BooleanArray::from(
                array
                    .iter()
                    .map(|v| *v.downcast_ref::<bool>().unwrap())
                    .collect::<Vec<bool>>(),
            );
            Ok(Rc::new(ArrowFieldArray::new(Box::new(arrow_array))))
        }
        _ => unreachable!(),
    }
}

#[macro_export]
macro_rules! math_binary_op {
    ($LEFT: expr, $RIGHT: expr, $DATA_TYPE: expr, $OP: tt) => {
        match $DATA_TYPE {
            DataType::Int64 => {
                let l = $LEFT.downcast_ref::<i64>().unwrap();
                let r = $RIGHT.downcast_ref::<i64>().unwrap();
                Box::new(*l $OP *r) as Box<dyn Any>
            }
            DataType::Float32 => {
                let l = $LEFT.downcast_ref::<f32>().unwrap();
                let r = $RIGHT.downcast_ref::<f32>().unwrap();
                Box::new(*l $OP *r) as Box<dyn Any>
            }
            DataType::Float64 => {
                let l = $LEFT.downcast_ref::<f64>().unwrap();
                let r = $RIGHT.downcast_ref::<f64>().unwrap();
                Box::new(*l $OP *r) as Box<dyn Any>
            }
            _ => unreachable!(),
        }
    };
}

fn and(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Boolean => {
            let l = l.downcast_ref::<bool>().unwrap();
            let r = r.downcast_ref::<bool>().unwrap();
            Box::new(*l && *r)
        }
        _ => unreachable!(),
    }
}

fn or(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Boolean => {
            let l = l.downcast_ref::<bool>().unwrap();
            let r = r.downcast_ref::<bool>().unwrap();
            Box::new(*l || *r)
        }
        _ => unreachable!(),
    }
}

#[macro_export]
macro_rules! bool_binary_op {
    ($LEFT: expr, $RIGHT: expr, $DATA_TYPE: expr, $OP: ident) => {
        match $DATA_TYPE {
            DataType::Int32 => {
                let l = $LEFT.downcast_ref::<i32>().unwrap();
                let r = $RIGHT.downcast_ref::<i32>().unwrap();
                Box::new(l.$OP(r)) as Box<dyn Any>
            }
            DataType::Int64 => {
                let l = $LEFT.downcast_ref::<i64>().unwrap();
                let r = $RIGHT.downcast_ref::<i64>().unwrap();
                Box::new(l.$OP(r)) as Box<dyn Any>
            }
            DataType::Float32 => {
                let l = $LEFT.downcast_ref::<f32>().unwrap();
                let r = $RIGHT.downcast_ref::<f32>().unwrap();
                let l = OrderedFloat(*l);
                let r = OrderedFloat(*r);
                Box::new(l.$OP(&r)) as Box<dyn Any>
            }
            DataType::Float64 => {
                let l = $LEFT.downcast_ref::<f64>().unwrap();
                let r = $RIGHT.downcast_ref::<f64>().unwrap();
                let l = OrderedFloat(*l);
                let r = OrderedFloat(*r);
                Box::new(l.$OP(&r)) as Box<dyn Any>
            }
            _ => unreachable!(),
        }
    };
}

pub(crate) struct Cast {
    expr: Box<Expr>,
    data_type: DataType,
}

impl Cast {
    pub(crate) fn new(expr: Expr, data_type: DataType) -> Self {
        Self {
            expr: Box::new(expr),
            data_type,
        }
    }
}

impl PhysicalExpr for Cast {
    fn evaluate(&self, input: &RecordBatch) -> Result<ArrayRef> {
        let value = self.expr.evaluate(input)?;
        let values = cast(&value, &self.data_type)?;
        evaluate_from_values(&values, &self.data_type)
    }
}

impl Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CAST({} AS {})", self.expr, self.data_type)
    }
}

fn cast(value: &ArrayRef, data_type: &DataType) -> Result<Vec<Box<dyn Any>>> {
    Ok(match value.get_type() {
        DataType::Int64 => (0..value.size())
            .map(|i| Ok(*value.get_value(i)?.downcast_ref::<i64>().unwrap()))
            .map(|v: Result<i64, Error>| match v {
                Ok(v) => match data_type {
                    DataType::Int64 => Ok(Box::new(v) as Box<dyn Any>),
                    DataType::Float32 => Ok(Box::new(v as f32) as Box<dyn Any>),
                    DataType::Float64 => Ok(Box::new(v as f64) as Box<dyn Any>),
                    _ => unreachable!(),
                },
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<Box<dyn Any>>, _>>()?,
        DataType::Float32 => (0..value.size())
            .map(|i| Ok(*value.get_value(i)?.downcast_ref::<f32>().unwrap()))
            .map(|v: Result<f32, Error>| match v {
                Ok(v) => match data_type {
                    DataType::Int64 => Ok(Box::new(v as i64) as Box<dyn Any>),
                    DataType::Float32 => Ok(Box::new(v) as Box<dyn Any>),
                    DataType::Float64 => Ok(Box::new(v as f64) as Box<dyn Any>),
                    _ => unreachable!(),
                },
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<Box<dyn Any>>, _>>()?,
        DataType::Float64 => (0..value.size())
            .map(|i| Ok(*value.get_value(i)?.downcast_ref::<f64>().unwrap()))
            .map(|v: Result<f64, Error>| match v {
                Ok(v) => match data_type {
                    DataType::Int64 => Ok(Box::new(v as i64) as Box<dyn Any>),
                    DataType::Float32 => Ok(Box::new(v as f32) as Box<dyn Any>),
                    DataType::Float64 => Ok(Box::new(v) as Box<dyn Any>),
                    _ => unreachable!(),
                },
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<Box<dyn Any>>, _>>()?,
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use super::{BinaryExpr, Cast, Column, Expr, PhysicalExpr, ScalarValue};
    use crate::{
        data_types::{
            arrow_field_array::ArrowFieldArray,
            column_array::{ArrayRef, DataType},
            record_batch::RecordBatch,
            schema::{Field, Schema},
        },
        logical_plan::expr::Operator,
    };
    use arrow::array::{BooleanArray, Int64Array};
    use std::rc::Rc;

    #[test]
    fn test_column_expr_evaluate() {
        let id = Int64Array::from(vec![1, 2, 3, 4, 5]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = Column::new(0);
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &1
        );
    }

    #[test]
    fn test_column_expr_display() {
        let expr = Column::new(0);
        assert_eq!(expr.to_string(), "#0");
    }

    #[test]
    fn test_scalar_value_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = ScalarValue::Int64(1);
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &1
        );
    }

    #[test]
    fn test_scalar_value_expr_display() {
        let expr = ScalarValue::Int64(1);
        assert_eq!(expr.to_string(), "1");
    }

    #[test]
    fn test_add_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Add,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &2
        );
    }

    #[test]
    fn test_add_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Add,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 + 1");
    }

    #[test]
    fn test_subtract_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Subtract,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &0
        );
    }

    #[test]
    fn test_subtract_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Subtract,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 - 1");
    }

    #[test]
    fn test_multiply_expr_evaluate() {
        let id = Int64Array::from(vec![2]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Multiply,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &2
        );
    }

    #[test]
    fn test_multiply_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Multiply,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 * 1");
    }

    #[test]
    fn test_divide_expr_evaluate() {
        let id = Int64Array::from(vec![2]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Divide,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &2
        );
    }

    #[test]
    fn test_divide_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Divide,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 / 1");
    }

    #[test]
    fn test_modulus_expr_evaluate() {
        let id = Int64Array::from(vec![3]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Modulus,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap()
                == &1
        );
    }

    #[test]
    fn test_modulus_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Modulus,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert_eq!(expr.to_string(), "#0 % 2");
    }

    #[test]
    fn test_and_expr_evaluate() {
        let bool = BooleanArray::from(vec![false]);
        let bool_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(bool))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("bool".to_string(), DataType::Boolean)]);
        let input = RecordBatch::new(schema, bool_arrary);
        let expr = BinaryExpr::new(
            Operator::And,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Column(Column::new(0))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &false
        );
    }

    #[test]
    fn test_and_expr_display() {
        let expr = BinaryExpr::new(
            Operator::And,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Column(Column::new(0))),
        );
        assert_eq!(expr.to_string(), "#0 AND #0");
    }

    #[test]
    fn test_or_expr_evaluate() {
        let bool = BooleanArray::from(vec![false]);
        let bool_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(bool))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("bool".to_string(), DataType::Boolean)]);
        let input = RecordBatch::new(schema, bool_arrary);
        let expr = BinaryExpr::new(
            Operator::Or,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Column(Column::new(0))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &false
        );
    }

    #[test]
    fn test_or_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Or,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Column(Column::new(0))),
        );
        assert_eq!(expr.to_string(), "#0 OR #0");
    }

    #[test]
    fn test_eq_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Eq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &true
        );
    }

    #[test]
    fn test_eq_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Eq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 == 1");
    }

    #[test]
    fn test_neq_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Neq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &false
        );
    }

    #[test]
    fn test_neq_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Neq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 != 1");
    }

    #[test]
    fn test_lt_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Lt,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &true
        );
    }

    #[test]
    fn test_lt_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Lt,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert_eq!(expr.to_string(), "#0 < 2");
    }

    #[test]
    fn test_lt_eq_expr_evaluate() {
        let id = Int64Array::from(vec![2]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::LtEq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &true
        );
    }

    #[test]
    fn test_lt_eq_expr_display() {
        let expr = BinaryExpr::new(
            Operator::LtEq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert_eq!(expr.to_string(), "#0 <= 2");
    }

    #[test]
    fn test_gt_expr_evaluate() {
        let id = Int64Array::from(vec![1]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::Gt,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &false
        );
    }

    #[test]
    fn test_gt_expr_display() {
        let expr = BinaryExpr::new(
            Operator::Gt,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 > 1");
    }

    #[test]
    fn test_gt_eq_expr_evaluate() {
        let id = Int64Array::from(vec![2]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = BinaryExpr::new(
            Operator::GtEq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<bool>()
                .unwrap()
                == &true
        );
    }

    #[test]
    fn test_gt_eq_expr_display() {
        let expr = BinaryExpr::new(
            Operator::GtEq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(2))),
        );
        assert_eq!(expr.to_string(), "#0 >= 2");
    }

    #[test]
    fn test_cast_expr_evaluate() {
        let id = Int64Array::from(vec![2]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int64)]);
        let input = RecordBatch::new(schema, id_arrary);
        let expr = Cast::new(Expr::Column(Column::new(0)), DataType::Float64);
        assert!(expr.evaluate(&input).is_ok());
        assert!(
            expr.evaluate(&input)
                .unwrap()
                .get_value(0)
                .unwrap()
                .downcast_ref::<f64>()
                .unwrap()
                == &2.0
        );
    }

    #[test]
    fn test_cast_expr_display() {
        let expr = Cast::new(Expr::Column(Column::new(0)), DataType::Float64);
        assert_eq!(expr.to_string(), "CAST(#0 AS Float64)");
    }
}
