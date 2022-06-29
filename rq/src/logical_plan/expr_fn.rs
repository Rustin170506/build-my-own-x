use super::expr::{AggregateExpr, AggregateFunction, BinaryExpr, Expr, Operator, ScalarValue};

/// Create a column expression based on a qualified or unqualified column name
pub(crate) fn col(ident: &str) -> Expr {
    Expr::Column(ident.into())
}

/// Return a new expression l <op> r
pub(crate) fn binary_expr(l: Expr, op: Operator, r: Expr) -> Expr {
    Expr::BinaryExpr(BinaryExpr {
        left: Box::new(l),
        op,
        right: Box::new(r),
    })
}

/// Return a new expression with a logical AND
pub(crate) fn and(left: Expr, right: Expr) -> Expr {
    Expr::BinaryExpr(BinaryExpr {
        left: Box::new(left),
        op: Operator::And,
        right: Box::new(right),
    })
}

/// Return a new expression with a logical OR
pub(crate) fn or(left: Expr, right: Expr) -> Expr {
    Expr::BinaryExpr(BinaryExpr {
        left: Box::new(left),
        op: Operator::Or,
        right: Box::new(right),
    })
}

/// Create an expression to represent the min() aggregate function
pub(crate) fn min(expr: Expr) -> Expr {
    Expr::AggregateFunction(AggregateExpr {
        fun: AggregateFunction::Min,
        is_distinct: false,
        expr: Box::new(expr),
    })
}

/// Create an expression to represent the max() aggregate function
pub(crate) fn max(expr: Expr) -> Expr {
    Expr::AggregateFunction(AggregateExpr {
        fun: AggregateFunction::Max,
        is_distinct: false,
        expr: Box::new(expr),
    })
}

/// Create an expression to represent the sum() aggregate function
pub(crate) fn sum(expr: Expr) -> Expr {
    Expr::AggregateFunction(AggregateExpr {
        fun: AggregateFunction::Sum,
        is_distinct: false,
        expr: Box::new(expr),
    })
}

/// Create an expression to represent the avg() aggregate function
pub(crate) fn avg(expr: Expr) -> Expr {
    Expr::AggregateFunction(AggregateExpr {
        fun: AggregateFunction::Avg,
        is_distinct: false,
        expr: Box::new(expr),
    })
}

/// Create an expression to represent the count() aggregate function
pub(crate) fn count(expr: Expr) -> Expr {
    Expr::AggregateFunction(AggregateExpr {
        fun: AggregateFunction::Count,
        is_distinct: false,
        expr: Box::new(expr),
    })
}

/// Create an expression to represent the count(distinct) aggregate function
pub(crate) fn count_distinct(expr: Expr) -> Expr {
    Expr::AggregateFunction(AggregateExpr {
        fun: AggregateFunction::CountDistinct,
        is_distinct: true,
        expr: Box::new(expr),
    })
}

/// Create a literal expression
pub(crate) fn lit<T: Literal>(n: T) -> Expr {
    n.lit()
}

/// Trait for converting a type to a [`Literal`] literal expression.
pub(crate) trait Literal {
    /// convert the value to a Literal expression
    fn lit(&self) -> Expr;
}

impl Literal for String {
    fn lit(&self) -> Expr {
        Expr::Literal(ScalarValue::String(self.clone()))
    }
}

impl Literal for i64 {
    fn lit(&self) -> Expr {
        Expr::Literal(ScalarValue::Int64(*self))
    }
}

impl Literal for f32 {
    fn lit(&self) -> Expr {
        Expr::Literal(ScalarValue::Float32(*self))
    }
}

impl Literal for f64 {
    fn lit(&self) -> Expr {
        Expr::Literal(ScalarValue::Float64(*self))
    }
}
