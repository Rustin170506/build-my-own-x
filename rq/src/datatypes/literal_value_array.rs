use std::any::Any;

use super::column_array::ColumnArray;
use anyhow::{bail, Result};
use arrow::datatypes::DataType;

/// Represents a literal value
#[derive(Clone)]
pub(crate) struct LiteralValueVector<T> {
    arrow_type: DataType,
    value: T,
    size: usize,
}

impl<T: Clone + Any> ColumnArray for LiteralValueVector<T> {
    fn get_type(&self) -> DataType {
        self.arrow_type.clone()
    }

    fn get_value(&self, i: usize) -> Result<Box<dyn Any>> {
        if i >= self.size {
            bail!("Out of index")
        }
        Ok(Box::new(self.value.clone()))
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T: Clone + Any> LiteralValueVector<T> {
    pub(crate) fn new(arrow_type: DataType, value: T, size: usize) -> Self {
        Self {
            arrow_type,
            value,
            size,
        }
    }
}
