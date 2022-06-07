use anyhow::Result;
use arrow::datatypes::DataType;
use std::any::Any;

/// Abstraction over different implementations of a column vector.
pub(crate) trait ColumnArray {
    /// Return the type of the column.
    fn get_type(&self) -> DataType;
    /// Return the value at the given index.
    fn get_value(&self, i: usize) -> Result<Box<dyn Any>>;
    /// Return the number of elements in the column.
    fn size(&self) -> usize;
}
