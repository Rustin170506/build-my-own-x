use std::any::Any;

use super::column_array::{ColumnArray, DataType};

use anyhow::{bail, Result};

/// Represents a literal value
#[derive(Clone)]
pub(crate) struct LiteralValueArray<T> {
    arrow_type: DataType,
    value: T,
    size: usize,
}

impl<T: Clone + Any> ColumnArray for LiteralValueArray<T> {
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

impl<T: Clone + Any> LiteralValueArray<T> {
    pub(crate) fn new(arrow_type: DataType, value: T, size: usize) -> Self {
        Self {
            arrow_type,
            value,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_literal_value_array() {
        let _ = LiteralValueArray::new(DataType::Int32, 1, 1);
    }

    #[test]
    fn test_get_type() {
        let array = LiteralValueArray::new(DataType::Int32, 1, 1);
        assert_eq!(array.get_type(), DataType::Int32);
    }

    #[test]
    fn test_get_value() {
        let array = LiteralValueArray::new(DataType::Int32, 1, 1);
        assert_eq!(
            array.get_value(0).unwrap().downcast_ref::<i32>().unwrap(),
            &1
        );
    }

    #[test]
    fn test_size() {
        let array = LiteralValueArray::new(DataType::Int32, 1, 1);
        assert_eq!(array.size(), 1);
    }
}
