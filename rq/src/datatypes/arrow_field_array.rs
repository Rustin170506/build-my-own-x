use std::any::Any;

use super::column_array::ColumnArray;
use anyhow::Result;
use arrow::{
    array::{
        Array, BooleanArray, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array,
        Int8Array, StringArray, UInt16Array, UInt32Array, UInt64Array, UInt8Array,
    },
    datatypes::DataType,
};

/// Wrapper around Arrow Array.
struct ArrowFieldArray {
    field: Box<dyn Array>,
}

impl ColumnArray for ArrowFieldArray {
    fn get_type(&self) -> DataType {
        match self.field.data_type() {
            DataType::Boolean => DataType::Boolean,
            DataType::Int8 => DataType::Int8,
            DataType::Int16 => DataType::Int16,
            DataType::Int32 => DataType::Int32,
            DataType::Int64 => DataType::Int64,
            DataType::UInt8 => DataType::UInt8,
            DataType::UInt16 => DataType::UInt16,
            DataType::UInt32 => DataType::UInt32,
            DataType::UInt64 => DataType::UInt64,
            DataType::Float16 => DataType::Float16,
            DataType::Float32 => DataType::Float32,
            DataType::Float64 => DataType::Float64,
            DataType::Utf8 => DataType::Utf8,
            _ => unreachable!(),
        }
    }

    fn get_value(&self, i: usize) -> Result<Box<dyn Any>> {
        match self.field.data_type() {
            DataType::Boolean => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Int8 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Int8Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Int16 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Int16Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Int32 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Int64 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::UInt8 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<UInt8Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::UInt16 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<UInt16Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::UInt32 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<UInt32Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::UInt64 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<UInt64Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Float32 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Float32Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Float64 => Ok(Box::new(
                self.field
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .expect("Failed to downcast")
                    .value(i),
            )),
            DataType::Utf8 => Ok(Box::new(
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
    fn new(value: Box<dyn Array>) -> Self {
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
