use super::{column_array::ColumnArray, schema::Schema};

pub(crate) struct RecordBatch {
    pub(crate) schema: Schema,
    pub(crate) fields: Vec<Box<dyn ColumnArray>>,
}

impl RecordBatch {
    pub(crate) fn field(&self, index: usize) -> Box<dyn ColumnArray> {
        dyn_clone::clone_box(&*self.fields[index])
    }
    fn row_count(&self) -> usize {
        self.fields[0].size()
    }

    fn column_count(&self) -> usize {
        self.fields.len()
    }

    fn to_csv(&self) -> String {
        let mut s = String::new();
        for row in 0..self.row_count() {
            for col in 0..self.column_count() {
                if col > 0 {
                    s.push(',');
                }
                let field = &self.fields[col];
                let value = &field.get_value(row).unwrap();
                s.push_str(value.to_string().as_str());
            }
            s.push('\n')
        }
        s
    }
}

impl ToString for RecordBatch {
    fn to_string(&self) -> String {
        self.to_csv()
    }
}
