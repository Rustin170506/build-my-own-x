use super::{expr_fn::binary_expr, logical_expr::LogicalExpr, plan::LogicalPlan};
use crate::data_types::schema::Field;
use anyhow::{anyhow, Result};
use arrow::datatypes::DataType;
use ordered_float::OrderedFloat;
use std::{cmp::Ordering, fmt::Display, hash::Hash, ops};

/// `Expr` represent logical expressions such as `A + 1`, or `CAST(c1 AS
/// int)`.
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
pub(crate) enum Expr {
    /// A named reference to a qualified filed in a schema.
    Column(Column),
    /// A indexed reference to a qualified filed in a schema.
    ColumnIndex(ColumnIndex),
    /// A constant value.
    Literal(ScalarValue),
    /// Negation of an expression. The expression's type must be a boolean to make sense.
    Not(Not),
    /// Casts the expression to a given type and will return a runtime error if the expression cannot be cast.
    /// This expression is guaranteed to have a fixed type.
    Cast(Cast),
    /// A binary expression such as "age > 21"
    BinaryExpr(BinaryExpr),
    /// An expression with a specific name.
    Alias(Alias),
    /// Represents the call of a built-in scalar function with a set of arguments.
    ScalarFunction(ScalarFunction),
    /// Represents the call of an aggregate built-in function with arguments.
    AggregateFunction(AggregateExpr),
}

impl LogicalExpr for Expr {
    fn to_field(&self, input: Box<dyn LogicalPlan>) -> Result<Field> {
        match self {
            Expr::Column(column) => column.to_field(input),
            Expr::ColumnIndex(column_index) => column_index.to_field(input),
            Expr::Literal(literal) => literal.to_field(input),
            Expr::Not(not) => not.to_field(input),
            Expr::Cast(cast) => cast.to_field(input),
            Expr::BinaryExpr(binary) => binary.to_field(input),
            Expr::Alias(alias) => alias.to_field(input),
            Expr::ScalarFunction(function) => function.to_field(input),
            Expr::AggregateFunction(function) => function.to_field(input),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Column(column) => column.to_string(),
            Expr::ColumnIndex(column_index) => column_index.to_string(),
            Expr::Literal(literal) => literal.to_string(),
            Expr::Not(not) => not.to_string(),
            Expr::Cast(cast) => cast.to_string(),
            Expr::BinaryExpr(binary) => binary.to_string(),
            Expr::Alias(alias) => alias.to_string(),
            Expr::ScalarFunction(function) => function.to_string(),
            Expr::AggregateFunction(function) => function.to_string(),
        }
    }
}

impl ops::Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        binary_expr(self, Operator::Add, rhs)
    }
}

impl ops::Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        binary_expr(self, Operator::Subtract, rhs)
    }
}

impl ops::Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        binary_expr(self, Operator::Multiply, rhs)
    }
}

impl ops::Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        binary_expr(self, Operator::Divide, rhs)
    }
}

impl ops::Rem for Expr {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        binary_expr(self, Operator::Modulus, rhs)
    }
}

impl ops::Not for Expr {
    type Output = Self;

    fn not(self) -> Self::Output {
        Expr::Not(Not::new(Box::new(self)))
    }
}

impl Expr {
    /// Return `self == other`
    pub fn eq(self, other: Expr) -> Expr {
        binary_expr(self, Operator::Eq, other)
    }

    /// Return `self != other`
    pub fn not_eq(self, other: Expr) -> Expr {
        binary_expr(self, Operator::Neq, other)
    }

    /// Return `self > other`
    pub fn gt(self, other: Expr) -> Expr {
        binary_expr(self, Operator::Gt, other)
    }

    /// Return `self >= other`
    pub fn gt_eq(self, other: Expr) -> Expr {
        binary_expr(self, Operator::GtEq, other)
    }

    /// Return `self < other`
    pub fn lt(self, other: Expr) -> Expr {
        binary_expr(self, Operator::Lt, other)
    }

    /// Return `self <= other`
    pub fn lt_eq(self, other: Expr) -> Expr {
        binary_expr(self, Operator::LtEq, other)
    }

    /// Return `self && other`
    pub fn and(self, other: Expr) -> Expr {
        binary_expr(self, Operator::And, other)
    }

    /// Return `self || other`
    pub fn or(self, other: Expr) -> Expr {
        binary_expr(self, Operator::Or, other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl From<&str> for Column {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
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

/// Represents a dynamically typed single value.
#[derive(Debug, Clone)]
pub(crate) enum ScalarValue {
    String(String),
    Int64(i64),
    Float32(f32),
    Float64(f64),
}

impl LogicalExpr for ScalarValue {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        match &self {
            ScalarValue::String(s) => Ok(Field::new(s.clone(), DataType::Utf8)),
            ScalarValue::Int64(i) => Ok(Field::new(i.to_string(), DataType::Int64)),
            ScalarValue::Float32(f) => Ok(Field::new(f.to_string(), DataType::Float32)),
            ScalarValue::Float64(f) => Ok(Field::new(f.to_string(), DataType::Float64)),
        }
    }
}

impl Display for ScalarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarValue::String(s) => write!(f, "{}", s),
            ScalarValue::Int64(i) => write!(f, "{}", i),
            ScalarValue::Float32(ft) => write!(f, "{}", ft),
            ScalarValue::Float64(ft) => write!(f, "{}", ft),
        }
    }
}

impl std::hash::Hash for ScalarValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ScalarValue::String(s) => s.hash(state),
            ScalarValue::Int64(i) => i.hash(state),
            ScalarValue::Float32(ft) => {
                let ft = OrderedFloat(*ft);
                ft.hash(state)
            }
            ScalarValue::Float64(ft) => {
                let ft = OrderedFloat(*ft);
                ft.hash(state)
            }
        }
    }
}

impl PartialEq for ScalarValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScalarValue::String(s), ScalarValue::String(o)) => s == o,
            (ScalarValue::Int64(i), ScalarValue::Int64(o)) => i == o,
            (ScalarValue::Float32(f), ScalarValue::Float32(o)) => {
                let v1 = OrderedFloat(*f);
                let v2 = OrderedFloat(*o);
                v1.eq(&v2)
            }
            (ScalarValue::Float64(f), ScalarValue::Float64(o)) => {
                let v1 = OrderedFloat(*f);
                let v2 = OrderedFloat(*o);
                v1.eq(&v2)
            }
            _ => false,
        }
    }
}

impl PartialOrd for ScalarValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ScalarValue::String(s), ScalarValue::String(o)) => s.partial_cmp(o),
            (ScalarValue::Int64(i), ScalarValue::Int64(o)) => i.partial_cmp(o),
            (ScalarValue::Float32(f), ScalarValue::Float32(o)) => {
                let v1 = OrderedFloat(*f);
                let v2 = OrderedFloat(*o);
                v1.partial_cmp(&v2)
            }
            (ScalarValue::Float64(f), ScalarValue::Float64(o)) => {
                let v1 = OrderedFloat(*f);
                let v2 = OrderedFloat(*o);
                v1.partial_cmp(&v2)
            }
            _ => None,
        }
    }
}

impl Eq for ScalarValue {}

/// Cast a given expression to a given data type field.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub(crate) struct Cast {
    pub(crate) expr: Box<Expr>,
    pub(crate) data_type: DataType,
}

impl LogicalExpr for Cast {
    fn to_field(&self, input: Box<dyn LogicalPlan>) -> Result<Field> {
        let field = self.expr.to_field(input)?;
        Ok(Field::new(field.name, self.data_type.clone()))
    }
}

impl ToString for Cast {
    fn to_string(&self) -> String {
        format!("CAST({} AS {})", self.expr.to_string(), self.data_type)
    }
}

/// Logical expression representing a logical NOT.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub(crate) struct Not {
    name: String,
    op: String,
    pub(crate) expr: Box<Expr>,
}

impl Not {
    fn new(expr: Box<Expr>) -> Self {
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub(crate) struct BinaryExpr {
    pub(crate) op: Operator,
    pub(crate) left: Box<Expr>,
    pub(crate) right: Box<Expr>,
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub(crate) struct Alias {
    pub(crate) expr: Box<Expr>,
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub(crate) struct ScalarFunction {
    pub(crate) name: String,
    pub(crate) args: Vec<Expr>,
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
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

/// AggregateFunction is a logical expression that represents an aggregate function.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub(crate) struct AggregateExpr {
    pub(crate) fun: AggregateFunction,
    pub(crate) expr: Box<Expr>,
    pub(crate) is_distinct: bool,
}

impl LogicalExpr for AggregateExpr {
    fn to_field(&self, _input: Box<dyn LogicalPlan>) -> Result<Field> {
        Ok(Field::new(
            self.fun.get_name(),
            self.expr.to_field(_input)?.data_type,
        ))
    }
}

impl ToString for AggregateExpr {
    fn to_string(&self) -> String {
        if self.is_distinct {
            format!("{}(DISTINCT {})", self.fun, self.expr.to_string())
        } else {
            format!("{}({})", self.fun, self.expr.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::logical_plan::expr_fn::{col, lit};
    use std::ops::{Add, Not};

    #[test]
    fn test_add() {
        assert_eq!(col("a").add(col("b")), col("a") + col("b"));
    }

    #[test]
    fn test_not() {
        assert_eq!(lit(1).not(), !lit(1));
    }

    #[test]
    fn test_partial_ord() {
        // Test validates that partial ord is defined for Expr using hashes, not
        // intended to exhaustively test all possibilities
        let exp1 = col("a") + lit(1);
        let exp2 = col("a") + lit(2);
        let exp3 = !(col("a") + lit(2));
        assert!(exp1 < exp2);
        assert!(exp2 > exp1);
        assert!(exp2 > exp3);
        assert!(exp3 < exp2);
        assert!(lit(1.2) < lit(1.3));
    }
}
