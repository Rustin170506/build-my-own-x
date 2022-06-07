use super::{column_array::ColumnArray, schema::Schema};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct RecordBatch {
    pub(crate) schema: Schema,
    pub(crate) fields: Vec<Rc<dyn ColumnArray>>,
}

/// Batch of data organized in columns.
impl RecordBatch {
    /// Access one column by index.
    pub(crate) fn field(&self, index: usize) -> &Rc<dyn ColumnArray> {
        &self.fields[index]
    }

    fn row_count(&self) -> usize {
        self.fields[0].size()
    }

    fn column_count(&self) -> usize {
        self.fields.len()
    }
}
