use crate::{
    data_types::{
        arrow_field_array::ArrowFieldArray, column_array::ArrayRef,
        literal_value_array::LiteralValueArray, record_batch::RecordBatch,
    },
    logical_plan::expr::Operator,
};
use anyhow::Result;
use arrow::{
    array::{BooleanArray, Int64Array},
    datatypes::DataType,
};
use ordered_float::OrderedFloat;
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
                    let value = add(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Subtract => {
                for i in 0..left.size() {
                    let value = subtract(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Multiply => {
                for i in 0..left.size() {
                    let value = multiply(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Divide => {
                for i in 0..left.size() {
                    let value = divide(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &arrow_type)
            }
            Operator::Modulus => {
                for i in 0..left.size() {
                    let value = modulus(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
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
                    let value = eq(&left.get_value(i)?, &right.get_value(i)?, &arrow_type);
                    vals.push(value);
                }
                evaluate_from_values(&vals, &DataType::Boolean)
            }
            _ => unimplemented!(),
        }
    }
}

impl ToString for BinaryExpr {
    fn to_string(&self) -> String {
        match self.op {
            Operator::Add => format!("{} + {}", self.left.to_string(), self.right.to_string()),
            Operator::Subtract => format!("{} - {}", self.left.to_string(), self.right.to_string()),
            Operator::Multiply => format!("{} * {}", self.left.to_string(), self.right.to_string()),
            Operator::Divide => format!("{} / {}", self.left.to_string(), self.right.to_string()),
            Operator::Modulus => format!("{} % {}", self.left.to_string(), self.right.to_string()),
            Operator::And => format!("{} AND {}", self.left.to_string(), self.right.to_string()),
            Operator::Or => format!("{} OR {}", self.left.to_string(), self.right.to_string()),
            Operator::Eq => format!("{} == {}", self.left.to_string(), self.right.to_string()),
            _ => unimplemented!(),
        }
    }
}

impl BinaryExpr {
    pub(crate) fn new(op: Operator, left: Box<Expr>, right: Box<Expr>) -> Self {
        Self { op, left, right }
    }
}

// Build the arrow array from the values.
fn evaluate_from_values(array: &[Box<dyn Any>], data_type: &DataType) -> Result<ArrayRef> {
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

fn add(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
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
        _ => unreachable!(),
    }
}

fn subtract(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Int64 => {
            let l = l.downcast_ref::<i64>().unwrap();
            let r = r.downcast_ref::<i64>().unwrap();
            Box::new(*l - *r)
        }
        DataType::Float32 => {
            let l = l.downcast_ref::<f32>().unwrap();
            let r = r.downcast_ref::<f32>().unwrap();
            Box::new(*l - *r)
        }
        DataType::Float64 => {
            let l = l.downcast_ref::<f64>().unwrap();
            let r = r.downcast_ref::<f64>().unwrap();
            Box::new(*l - *r)
        }
        _ => unreachable!(),
    }
}

fn multiply(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Int64 => {
            let l = l.downcast_ref::<i64>().unwrap();
            let r = r.downcast_ref::<i64>().unwrap();
            Box::new(*l * *r)
        }
        DataType::Float32 => {
            let l = l.downcast_ref::<f32>().unwrap();
            let r = r.downcast_ref::<f32>().unwrap();
            Box::new(*l * *r)
        }
        DataType::Float64 => {
            let l = l.downcast_ref::<f64>().unwrap();
            let r = r.downcast_ref::<f64>().unwrap();
            Box::new(*l * *r)
        }
        _ => unreachable!(),
    }
}

fn divide(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Int64 => {
            let l = l.downcast_ref::<i64>().unwrap();
            let r = r.downcast_ref::<i64>().unwrap();
            Box::new(*l / *r)
        }
        DataType::Float32 => {
            let l = l.downcast_ref::<f32>().unwrap();
            let r = r.downcast_ref::<f32>().unwrap();
            Box::new(*l / *r)
        }
        DataType::Float64 => {
            let l = l.downcast_ref::<f64>().unwrap();
            let r = r.downcast_ref::<f64>().unwrap();
            Box::new(*l / *r)
        }
        _ => unreachable!(),
    }
}

fn modulus(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Int64 => {
            let l = l.downcast_ref::<i64>().unwrap();
            let r = r.downcast_ref::<i64>().unwrap();
            Box::new(*l % *r)
        }
        DataType::Float32 => {
            let l = l.downcast_ref::<f32>().unwrap();
            let r = r.downcast_ref::<f32>().unwrap();
            Box::new(*l % *r)
        }
        DataType::Float64 => {
            let l = l.downcast_ref::<f64>().unwrap();
            let r = r.downcast_ref::<f64>().unwrap();
            Box::new(*l % *r)
        }
        _ => unreachable!(),
    }
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

fn eq(l: &Box<dyn Any>, r: &Box<dyn Any>, data_type: &DataType) -> Box<dyn Any> {
    match data_type {
        DataType::Int64 => {
            let l = l.downcast_ref::<i64>().unwrap();
            let r = r.downcast_ref::<i64>().unwrap();
            Box::new(l == r)
        }
        DataType::Float32 => {
            let l = l.downcast_ref::<f32>().unwrap();
            let r = r.downcast_ref::<f32>().unwrap();
            let l = OrderedFloat(*l);
            let r = OrderedFloat(*r);
            Box::new(l.eq(&r))
        }
        DataType::Float64 => {
            let l = l.downcast_ref::<f64>().unwrap();
            let r = r.downcast_ref::<f64>().unwrap();
            let l = OrderedFloat(*l);
            let r = OrderedFloat(*r);
            Box::new(l.eq(&r))
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryExpr, Column, Expr, PhysicalExpr, ScalarValue};
    use crate::{
        data_types::{
            arrow_field_array::ArrowFieldArray,
            column_array::ArrayRef,
            record_batch::RecordBatch,
            schema::{Field, Schema},
        },
        logical_plan::expr::Operator,
    };
    use arrow::{
        array::{BooleanArray, Int64Array},
        datatypes::DataType,
    };
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
    fn test_column_expr_to_string() {
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
    fn test_scalar_value_expr_to_string() {
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
    fn test_add_expr_to_string() {
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
    fn test_subtract_expr_to_string() {
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
    fn test_multiply_expr_to_string() {
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
    fn test_divide_expr_to_string() {
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
    fn test_modulus_expr_to_string() {
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
    fn test_and_expr_to_string() {
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
    fn test_or_expr_to_string() {
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
    fn test_eq_expr_to_string() {
        let expr = BinaryExpr::new(
            Operator::Eq,
            Box::new(Expr::Column(Column::new(0))),
            Box::new(Expr::Literal(ScalarValue::Int64(1))),
        );
        assert_eq!(expr.to_string(), "#0 == 1");
    }
}
