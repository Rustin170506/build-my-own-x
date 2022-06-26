use arrow::datatypes::{DataType, Field as ArrowField, Schema as ArrowSchema};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) data_type: DataType,
}

impl Field {
    pub(crate) fn new(name: String, data_type: DataType) -> Self {
        Self { name, data_type }
    }
}

impl From<Field> for ArrowField {
    fn from(field: Field) -> Self {
        ArrowField::new(field.name.as_str(), field.data_type, false)
    }
}

/// A schema is a list of fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Schema {
    pub(crate) fields: Vec<Field>,
}

impl Schema {
    pub(crate) fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub(crate) fn select(&self, names: Vec<String>) -> Self {
        let mut filterd_fields = vec![];
        names.into_iter().for_each(|name| {
            let fields: Vec<&Field> = self.fields.iter().filter(|f| f.name == name).collect();
            assert!(fields.len() == 1);
            filterd_fields.push(fields[0].clone())
        });

        Self::new(filterd_fields)
    }
}

impl From<Schema> for ArrowSchema {
    fn from(schema: Schema) -> Self {
        let mut fields = vec![];
        schema.fields.into_iter().for_each(|field| {
            fields.push(field.into());
        });
        ArrowSchema::new(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let schema = Schema::new(vec![
            Field::new("id".to_string(), DataType::Int32),
            Field::new("name".to_string(), DataType::Utf8),
        ]);
        let selected_schema = schema.select(vec!["id".to_string()]);
        assert_eq!(selected_schema.fields.len(), 1);
        assert_eq!(selected_schema.fields[0].name, "id");
    }
}
