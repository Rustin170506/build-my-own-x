use arrow::datatypes::{DataType, Field as ArrowField, Schema as ArrowSchema};

#[derive(Debug, Clone)]
pub(crate) struct Field {
    pub(crate) name: String,
    data_type: DataType,
}

impl From<ArrowField> for Field {
    fn from(field: ArrowField) -> Self {
        Self {
            name: field.name().clone(),
            data_type: field.data_type().clone(),
        }
    }
}

/// A schema is a list of fields.
#[derive(Debug, Clone)]
pub(crate) struct Schema {
    pub(crate) fields: Vec<Field>,
}

impl Schema {
    fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    fn select(&self, names: Vec<String>) -> Self {
        let mut filterd_fields = vec![];
        names.into_iter().for_each(|name| {
            let fields: Vec<&Field> = self.fields.iter().filter(|f| f.name == name).collect();
            assert!(fields.len() == 1);
            filterd_fields.push(fields[0].clone())
        });

        Self::new(filterd_fields)
    }
}

impl From<ArrowSchema> for Schema {
    fn from(schema: ArrowSchema) -> Self {
        let fields = schema
            .fields()
            .iter()
            .map(|f| Field::from(f.clone()))
            .collect();
        Self { fields }
    }
}
