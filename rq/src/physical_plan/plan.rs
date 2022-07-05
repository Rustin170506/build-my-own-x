use crate::data_types::{record_batch::RecordBatch, schema::Schema};

/// A physical plan represents an executable piece of code that will produce data.
trait PhysicalPlan: ToString {
    /// Return the schema.
    fn schema(&self) -> &Schema;

    /// Execute a physical plan and produce a series of record batches.
    fn execute(&self) -> Box<dyn Iterator<Item = RecordBatch>>;

    /// Returns the children (inputs) of this physical plan.
    /// This method is used to enable use of the visitor pattern to walk a query tree
    fn children(&self) -> Vec<&dyn PhysicalPlan>;

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
