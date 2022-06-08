use anyhow::Result;
use arrow::datatypes::DataType;
use std::{any::Any, rc::Rc};

/// Abstraction over different implementations of a column vector.
pub(crate) trait ColumnArray {
    /// Return the type of the column.
    fn get_type(&self) -> DataType;
    /// Return the value at the given index.
    fn get_value(&self, i: usize) -> Result<Box<dyn Any>>;
    /// Return the number of elements in the column.
    fn size(&self) -> usize;
}

pub(crate) type ArrayRef = Rc<dyn ColumnArray>;

impl ColumnArray for ArrayRef {
    fn get_type(&self) -> DataType {
        self.as_ref().get_type()
    }

    fn get_value(&self, i: usize) -> Result<Box<dyn Any>> {
        self.as_ref().get_value(i)
    }

    fn size(&self) -> usize {
        self.as_ref().size()
    }
}
