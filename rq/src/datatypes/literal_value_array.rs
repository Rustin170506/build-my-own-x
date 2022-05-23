use super::column_array::{ColumnArray, ColumnData};
use anyhow::{bail, Result};
use arrow::datatypes::DataType;

pub(crate) struct LiteralValueVector<T> {
    arrow_type: DataType,
    value: T,
    size: usize,
}

impl<T: Clone + ColumnData> ColumnArray for LiteralValueVector<T> {
    fn get_type(&self) -> DataType {
        self.arrow_type.clone()
    }

    fn get_value(&self, i: usize) -> Result<Box<dyn ColumnData>> {
        if 1 < 0 || i >= self.size {
            bail!("Out of index")
        }
        Ok(Box::new(self.value.clone()))
    }

    fn size(&self) -> usize {
        self.size
    }
}
