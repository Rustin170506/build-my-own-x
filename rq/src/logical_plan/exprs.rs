use std::{fmt::Display, ops};

use super::{logical_expr::LogicalExpr, plan::LogicalPlan};
use crate::data_types::schema::Field;
use anyhow::{anyhow, Result};
use arrow::datatypes::DataType;

/// Logical expression representing a reference to a column by name.
pub(crate) struct Column {
    pub(crate) name: String,
}

impl LogicalExpr for Column {
    fn to_field(&self, input: Box<dyn LogicalPlan>) -> Result<Field> {
        if let Some(field) = input.schema().fields.iter().find(|f| f.name == self.name) {
            Ok(field.clone())
        } else {
            Err(anyhow!("No column named '{}'", self.name))
        }
    }
}

impl ToString for Column {
    fn to_string(&self) -> String {
        format!("#{}", self.name)
    }
}

/// Logical expression representing a reference to a column by index.
pub(crate) struct ColumnIndex {
    pub(crate) index: usize,
}

impl LogicalExpr for ColumnIndex {
    fn to_field(&self, input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(input.schema().fields[self.index].clone())
    }
}

impl ToString for ColumnIndex {
    fn to_string(&self) -> String {
        format!("#{}", self.index)
    }
}

pub(crate) struct LiteralString {
    pub(crate) value: String,
}

/// Logical expression representing a literal string value.
impl LogicalExpr for LiteralString {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.value.clone(), DataType::Utf8))
    }
}

impl ToString for LiteralString {
    fn to_string(&self) -> String {
        format!("'{}'", self.value)
    }
}

/// Logical expression representing a literal i64 value.
pub(crate) struct LiteralInt64 {
    pub(crate) value: i64,
}

impl LogicalExpr for LiteralInt64 {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.value.to_string(), DataType::Int64))
    }
}

impl ToString for LiteralInt64 {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

/// Logical expression representing a literal f32 value.
pub(crate) struct LiteralFloat32 {
    pub(crate) value: f32,
}

impl LogicalExpr for LiteralFloat32 {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.value.to_string(), DataType::Float32))
    }
}

impl ToString for LiteralFloat32 {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

/// Logical expression representing a literal f64 value.
pub(crate) struct LiteralFloat64 {
    pub(crate) value: f64,
}

impl LogicalExpr for LiteralFloat64 {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.value.to_string(), DataType::Float64))
    }
}

impl ToString for LiteralFloat64 {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

/// Cast a given expression to a given data type field.
pub(crate) struct CastExpr {
    pub(crate) expr: Box<dyn LogicalExpr>,
    pub(crate) data_type: DataType,
}

impl LogicalExpr for CastExpr {
    fn to_field(&self, input: Box<dyn LogicalPlan>) -> Result<Field> {
        let field = self.expr.to_field(input)?;
        Ok(Field::new(field.name, self.data_type.clone()))
    }
}

impl ToString for CastExpr {
    fn to_string(&self) -> String {
        format!("CAST({} AS {})", self.expr.to_string(), self.data_type)
    }
}

pub(crate) struct Not {
    name: String,
    op: String,
    pub(crate) expr: Box<dyn LogicalExpr>,
}

/// Logical expression representing a logical NOT.
impl Not {
    fn new(expr: Box<dyn LogicalExpr>) -> Self {
        Not {
            name: "not".to_string(),
            op: "NOT".to_string(),
            expr,
        }
    }
}

impl LogicalExpr for Not {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.name.clone(), DataType::Boolean))
    }
}

impl ToString for Not {
    fn to_string(&self) -> String {
        format!("{} {}", self.op, self.expr.to_string())
    }
}

/// Operators applied to expressions
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub(crate) enum Operator {
    And,
    Or,
    Eq,
    Neq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
}

impl Operator {
    fn get_name(&self) -> String {
        match self {
            Operator::And => "and".to_string(),
            Operator::Or => "or".to_string(),
            Operator::Eq => "eq".to_string(),
            Operator::Neq => "neq".to_string(),
            Operator::Gt => "gt".to_string(),
            Operator::GtEq => "gteq".to_string(),
            Operator::Lt => "lt".to_string(),
            Operator::LtEq => "lteq".to_string(),
            Operator::Add => "add".to_string(),
            Operator::Subtract => "subtract".to_string(),
            Operator::Multiply => "mult".to_string(),
            Operator::Divide => "div".to_string(),
            Operator::Modulus => "mod".to_string(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match &self {
            Operator::And => "AND",
            Operator::Or => "OR",
            Operator::Eq => "=",
            Operator::Neq => "!=",
            Operator::Gt => ">",
            Operator::GtEq => ">=",
            Operator::Lt => "<",
            Operator::LtEq => "<=",
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulus => "%",
        };
        write!(f, "{}", display)
    }
}

/// Binary expressions that return a boolean type.
pub(crate) struct BinaryExpr {
    pub(crate) op: Operator,
    pub(crate) left: Box<dyn LogicalExpr>,
    pub(crate) right: Box<dyn LogicalExpr>,
}

impl LogicalExpr for BinaryExpr {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.op.get_name(), DataType::Boolean))
    }
}

impl ToString for BinaryExpr {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.left.to_string(),
            self.op,
            self.right.to_string()
        )
    }
}

pub(crate) struct Alias {
    pub(crate) expr: Box<dyn LogicalExpr>,
    pub(crate) alias: String,
}

impl LogicalExpr for Alias {
    fn to_field(&self, input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(
            self.alias.clone(),
            self.expr.to_field(input)?.data_type,
        ))
    }
}

impl ToString for Alias {
    fn to_string(&self) -> String {
        format!("{} as {}", self.expr.to_string(), self.alias)
    }
}

pub(crate) struct ScalarFunction {
    pub(crate) name: String,
    pub(crate) args: Vec<Box<dyn LogicalExpr>>,
    pub(crate) return_type: DataType,
}

impl LogicalExpr for ScalarFunction {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(self.name.clone(), self.return_type.clone()))
    }
}

impl ToString for ScalarFunction {
    fn to_string(&self) -> String {
        format!(
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub(crate) enum AggregateFunction {
    Sum,
    Min,
    Max,
    Avg,
    Count,
    CountDistinct,
}

impl AggregateFunction {
    fn get_name(&self) -> String {
        match self {
            AggregateFunction::Sum => "sum".to_string(),
            AggregateFunction::Min => "min".to_string(),
            AggregateFunction::Max => "max".to_string(),
            AggregateFunction::Avg => "avg".to_string(),
            AggregateFunction::Count => "count".to_string(),
            AggregateFunction::CountDistinct => "count_distinct".to_string(),
        }
    }
}

impl Display for AggregateFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match &self {
            AggregateFunction::Sum => "SUM",
            AggregateFunction::Min => "MIN",
            AggregateFunction::Max => "MAX",
            AggregateFunction::Avg => "AVG",
            AggregateFunction::Count => "COUNT",
            AggregateFunction::CountDistinct => "COUNT DISTINCT",
        };
        write!(f, "{}", display)
    }
}

pub(crate) struct AggregateExpr {
    pub(crate) op: AggregateFunction,
    pub(crate) expr: Box<dyn LogicalExpr>,
    pub(crate) is_distinct: bool,
}

impl LogicalExpr for AggregateExpr {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(
            self.op.get_name(),
            self.expr.to_field(_input)?.data_type,
        ))
    }
}

impl ToString for AggregateExpr {
    fn to_string(&self) -> String {
        if self.is_distinct {
            format!("{}(DISTINCT {})", self.op, self.expr.to_string())
        } else {
            format!("{}({})", self.op, self.expr.to_string())
        }
    }
}
