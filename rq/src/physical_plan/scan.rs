use super::plan::{PhysicalPlan, Plan};
use crate::{
    data_source::{DataSource, Source},
    data_types::{record_batch::RecordBatch, schema::Schema},
};
use anyhow::Result;
use std::fmt::Display;

// Scan a data source with optional push-down projection.
pub(crate) struct ScanExec {
    data_source: Source,
    projection: Vec<String>,
}

impl ScanExec {
    pub(crate) fn new(data_source: Source, projection: Vec<String>) -> Self {
        ScanExec {
            data_source,
            projection,
        }
    }
}

impl PhysicalPlan for ScanExec {
    fn schema(&self) -> Schema {
        self.data_source
            .get_schema()
            .select(self.projection.iter().map(|s| s.as_str()).collect())
    }

    fn execute(&self) -> Result<Box<dyn Iterator<Item = RecordBatch> + '_>> {
        self.data_source
            .scan(self.projection.iter().map(|s| s.as_str()).collect())
    }

    fn children(&self) -> Vec<&Plan> {
        vec![]
    }
}

impl Display for ScanExec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScanExec: projection={}",
            self.projection
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ScanExec;
    use crate::{
        data_source::{csv_data_source::CsvDataSource, DataSource, Source},
        data_types::{
            column_array::DataType,
            schema::{Field, Schema},
        },
    };
    use std::path::PathBuf;

    #[test]
    fn test_scan_display() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/boolean_field.csv");
        let schema = Schema::new(vec![Field::new("c1".to_string(), DataType::Boolean)]);
        let csv_data_source =
            CsvDataSource::new(data_path.into_os_string().into_string().unwrap(), schema, 3);
        let scan = ScanExec::new(Source::Csv(csv_data_source), vec!["c1".to_string()]);
        assert_eq!(scan.to_string(), "ScanExec: projection=c1");
    }
}
