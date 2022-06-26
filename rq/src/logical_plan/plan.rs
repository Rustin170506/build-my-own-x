use crate::data_types::schema::Schema;

/// A logical plan represents a data transformation
/// or action that returns a relation(a set of tuples).
trait LogicalPlan: ToString {
    /// Returns the schema of the data that will be produced by this logical plan.
    fn schema(&self) -> Schema;
    /// Returns the children (inputs) of this logical plan.
    /// This method is used to enable use of the visitor pattern to walk a query tree.
    fn children(&self) -> Vec<Box<dyn LogicalPlan>>;

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
