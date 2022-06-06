use anyhow::Result;
use arrow::datatypes::DataType;
use dyn_clone::DynClone;
use std::any::Any;

pub(crate) trait ColumnData: ToString + Any {}

impl<T: ToString + Any> ColumnData for T {}

pub(crate) trait ColumnArray: DynClone {
    fn get_type(&self) -> DataType;
    fn get_value(&self, i: usize) -> Result<Box<dyn ColumnData>>;
    fn size(&self) -> usize;
}
