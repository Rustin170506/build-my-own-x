use std::{any::Any, fmt::Display, rc::Rc};

use anyhow::Result;
use arrow::datatypes::DataType as ArrowDataType;

// Data type of the column.
// We only support the following types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataType {
    Boolean,
    Int32,
    Int64,
    Float32,
    Float64,
    Utf8,
}

impl From<DataType> for ArrowDataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Boolean => ArrowDataType::Boolean,
            DataType::Int32 => ArrowDataType::Int32,
            DataType::Int64 => ArrowDataType::Int64,
            DataType::Float32 => ArrowDataType::Float32,
            DataType::Float64 => ArrowDataType::Float64,
            DataType::Utf8 => ArrowDataType::Utf8,
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Int32 => write!(f, "Int32"),
            DataType::Int64 => write!(f, "Int64"),
            DataType::Float32 => write!(f, "Float32"),
            DataType::Float64 => write!(f, "Float64"),
            DataType::Utf8 => write!(f, "Utf8"),
        }
    }
}

/// Abstraction over different implementations of a column vector.
pub trait ColumnArray {
    /// Return the type of the column.
    fn get_type(&self) -> DataType;
    /// Return the value at the given index.
    fn get_value(&self, i: usize) -> Result<Box<dyn Any>>;
    /// Return the number of elements in the column.
    fn size(&self) -> usize;
}

pub type ArrayRef = Rc<dyn ColumnArray>;

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
