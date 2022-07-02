use super::plan::Plan;
use crate::data_types::schema::Field;
use anyhow::Result;

/// Logical Expression for use in logical query plans.
/// The logical expression provides information needed
/// during the planning phase such as the name and data type of the expression.
pub(crate) trait LogicalExpr: ToString {
    /// Return meta-data about the value that will be produced by this expression when evaluated
    /// against a particular input.
    fn to_field(&self, input: &Plan) -> Result<Field>;
}
