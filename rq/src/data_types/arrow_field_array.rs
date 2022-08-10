use std::any::Any;

use super::column_array::{ColumnArray, DataType};

use anyhow::Result;
use arrow::{
    array::{
        Array, BooleanArray, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array,
        Int8Array, StringArray, UInt16Array, UInt32Array, UInt64Array, UInt8Array,
    },
    datatypes::DataType as ArrayDataType,
};

/// Wrapper around Arrow Array.
pub(crate) struct ArrowFieldArray {
    field: Box<dyn Array>,
}

impl ColumnArray for ArrowFieldArray {
    fn get_type(&self) -> DataType {
        match self.field.data_type() {
            ArrayDataType::Boolean => DataType::Boolean,
            ArrayDataType::Int32 => DataType::Int32,
            ArrayDataType::Int64 => DataType::Int64,
            ArrayDataType::Float32 => DataType::Float32,
            ArrayDataType::Float64 => DataType::Float64,
            ArrayDataType::Utf8 => DataType::Utf8,
            _ => unreachable!(),
        }
    }

    fn get_value(&self, i: usize) -> Result<Box<dyn Any>> {
        match self.field.data_type() {
            ArrayDataType::Boolean => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            ArrayDataType::Int32 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            ArrayDataType::Int64 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            ArrayDataType::Float32 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Float32Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            ArrayDataType::Float64 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            ArrayDataType::Utf8 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .expect("Failed to downcast")
                    .value(i)
                    .to_string(),
            )),
            _ => unreachable!(),
        }
    }

    fn size(&self) -> usize {
        self.field.len()
    }
}

impl ArrowFieldArray {
    pub(crate) fn new(value: Box<dyn Array>) -> Self {
        ArrowFieldArray { field: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_arrow_field_array() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let _ = ArrowFieldArray::new(Box::new(id));
    }

    #[test]
    fn test_get_type() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let a = ArrowFieldArray::new(Box::new(id));
        assert_eq!(a.get_type(), DataType::Int32);
        let s = StringArray::from(vec!["a", "b", "c", "d", "e"]);
        let a = ArrowFieldArray::new(Box::new(s));
        assert_eq!(a.get_type(), DataType::Utf8);
    }

    #[test]
    fn test_get_value() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let a = ArrowFieldArray::new(Box::new(id));
        assert_eq!(a.get_value(0).unwrap().downcast_ref::<i32>().unwrap(), &1);
        let s = StringArray::from(vec!["a", "b", "c", "d", "e"]);
        let a = ArrowFieldArray::new(Box::new(s));
        assert_eq!(
            a.get_value(0).unwrap().downcast_ref::<String>().unwrap(),
            &"a".to_string()
        );
    }

    #[test]
    fn test_size() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let a = ArrowFieldArray::new(Box::new(id));
        assert_eq!(a.size(), 5);
    }
}
