use crate::data_types::schema::Schema;

use super::{projection::Projection, scan::Scan};

/// A logical plan represents a data transformation
/// or action that returns a relation(a set of tuples).
pub(crate) trait LogicalPlan: ToString {
    /// Returns the schema of the data that will be produced by this logical plan.
    fn schema(&self) -> Schema;
    /// Returns the children (inputs) of this logical plan.
    /// This method is used to enable use of the visitor pattern to walk a query tree.
    fn children(&self) -> Vec<Plan>;

    fn pretty(&self, indent: usize) -> String {
        let mut result = String::new();
        for _ in 0..indent {
            result.push('\t');
        }
        result.push_str(&self.to_string());
        self.children()
            .iter()
            .for_each(|child| result.push_str(child.pretty(indent + 1).as_str()));

        result
    }
}

#[derive(Clone)]
pub(crate) enum Plan {
    Scan(Scan),
    Projection(Projection),
}

impl LogicalPlan for Plan {
    fn schema(&self) -> Schema {
        match self {
            Plan::Scan(scan) => scan.schema(),
            Plan::Projection(projection) => projection.schema(),
        }
    }

    fn children(&self) -> Vec<Plan> {
        match self {
            Plan::Scan(scan) => vec![],
            Plan::Projection(projection) => vec![*projection.input.clone()],
        }
    }
}

impl ToString for Plan {
    fn to_string(&self) -> String {
        match self {
            Plan::Scan(scan) => scan.to_string(),
            Plan::Projection(projection) => projection.to_string(),
        }
    }
}
