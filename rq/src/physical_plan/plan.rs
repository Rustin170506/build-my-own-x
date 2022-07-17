use crate::{
    data_source::{DataSource, Source},
    data_types::{record_batch::RecordBatch, schema::Schema},
    logical_plan::expr::Expr,
};
use anyhow::Result;
use std::fmt::Display;

use super::scan::Scan;

/// A physical plan represents an executable piece of code that will produce data.
pub(crate) trait PhysicalPlan: Display {
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
