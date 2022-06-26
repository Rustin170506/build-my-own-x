use super::{
    column_array::{ArrayRef, ColumnArray},
    schema::Schema,
};

#[derive(Clone)]
pub(crate) struct RecordBatch {
    pub(crate) schema: Schema,
    pub(crate) fields: Vec<ArrayRef>,
}

/// Batch of data organized in columns.
impl RecordBatch {
    pub(crate) fn new(schema: Schema, fields: Vec<ArrayRef>) -> Self {
        Self { schema, fields }
    }
    /// Access one column by index.
    pub(crate) fn field(&self, index: usize) -> &ArrayRef {
        &self.fields[index]
    }

    pub(crate) fn row_count(&self) -> usize {
        self.fields[0].size()
    }

    pub(crate) fn column_count(&self) -> usize {
        self.fields.len()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::data_types::{
        arrow_field_array::ArrowFieldArray, literal_value_array::LiteralValueArray, schema::Field,
    };
    use arrow::{array::Int32Array, datatypes::DataType};

    #[test]
    fn test_new_with_arrow_field_array() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int32)]);
        let _ = RecordBatch::new(schema, id_arrary);
    }

    #[test]
    fn test_new_with_literal_value_array() {
        let id_arrary = LiteralValueArray::new(DataType::Int32, 1, 1);
        let id_arrary = vec![Rc::new(id_arrary) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int32)]);
        let _ = RecordBatch::new(schema, id_arrary);
    }

    #[test]
    fn test_field() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int32)]);
        let batch = RecordBatch::new(schema, id_arrary);
        let field = batch.field(0);
        assert_eq!(field.get_type(), DataType::Int32);
        assert_eq!(field.size(), 5);
        assert_eq!(
            field.get_value(0).unwrap().downcast_ref::<i32>().unwrap(),
            &1
        );
        assert_eq!(
            field.get_value(1).unwrap().downcast_ref::<i32>().unwrap(),
            &2
        );
        assert_eq!(
            field.get_value(2).unwrap().downcast_ref::<i32>().unwrap(),
            &3
        );
        assert_eq!(
            field.get_value(3).unwrap().downcast_ref::<i32>().unwrap(),
            &4
        );
        assert_eq!(
            field.get_value(4).unwrap().downcast_ref::<i32>().unwrap(),
            &5
        );
    }

    #[test]
    fn test_row_count() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int32)]);
        let batch = RecordBatch::new(schema, id_arrary);
        assert_eq!(batch.row_count(), 5);
    }

    #[test]
    fn test_column_count() {
        let id = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let id_arrary = vec![Rc::new(ArrowFieldArray::new(Box::new(id))) as ArrayRef];
        let schema = Schema::new(vec![Field::new("id".to_string(), DataType::Int32)]);
        let batch = RecordBatch::new(schema, id_arrary);
        assert_eq!(batch.column_count(), 1);
    }
}
