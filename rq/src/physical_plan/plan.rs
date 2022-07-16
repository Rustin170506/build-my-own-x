use crate::{
    data_source::{DataSource, Source},
    data_types::{record_batch::RecordBatch, schema::Schema},
    logical_plan::expr::Expr,
};
use anyhow::Result;
use std::fmt::Display;

/// A physical plan represents an executable piece of code that will produce data.
trait PhysicalPlan: Display {
    /// Return the schema.
    fn schema(&self) -> Schema;

    /// Execute a physical plan and produce a series of record batches.
    fn execute(&self) -> Result<Box<dyn Iterator<Item = RecordBatch> + '_>>;

    /// Returns the children (inputs) of this physical plan.
    /// This method is used to enable use of the visitor pattern to walk a query tree
    fn children(&self) -> Vec<&Plan>;

    fn pretty(&self, indent: usize) -> String {
        let mut result = String::new();
        for _ in 0..indent {
            result.push('\t');
        }
        result.push_str(&self.to_string());
        result.push('\n');
        self.children()
            .iter()
            .for_each(|child| result.push_str(child.pretty(indent + 1).as_str()));

        result
    }
}

pub(crate) enum Plan {
    Scan(Scan),
}

impl PhysicalPlan for Plan {
    fn schema(&self) -> Schema {
        match self {
            Plan::Scan(scan) => scan.schema(),
        }
    }

    fn execute(&self) -> Result<Box<dyn Iterator<Item = RecordBatch> + '_>> {
        match self {
            Plan::Scan(scan) => scan.execute(),
        }
    }

    fn children(&self) -> Vec<&Plan> {
        match self {
            Plan::Scan(scan) => scan.children(),
        }
    }
}

impl Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Plan::Scan(scan) => scan.fmt(f),
        }
    }
}

// Scan a data source with optional push-down projection.
pub(crate) struct Scan {
    data_source: Source,
    projection: Vec<String>,
}

impl Scan {
    pub(crate) fn new(data_source: Source, projection: Vec<String>) -> Self {
        Scan {
            data_source,
            projection,
        }
    }
}

impl PhysicalPlan for Scan {
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

impl Display for Scan {
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
    use super::Scan;
    use crate::{
        data_source::{csv_data_source::CsvDataSource, DataSource, Source},
        data_types::schema::{Field, Schema},
    };
    use arrow::datatypes::DataType;
    use std::path::PathBuf;

    #[test]
    fn test_scan_display() {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("tests/data/boolean_field.csv");
        let schema = Schema::new(vec![Field::new("c1".to_string(), DataType::Boolean)]);
        let csv_data_source =
            CsvDataSource::new(data_path.into_os_string().into_string().unwrap(), schema, 3);
        let scan = Scan::new(Source::Csv(csv_data_source), vec!["c1".to_string()]);
        assert_eq!(scan.to_string(), "ScanExec: projection=c1");
    }
}
