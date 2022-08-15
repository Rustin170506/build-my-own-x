use std::fmt::Display;

use super::plan::{LogicalPlan, Plan};
use crate::{
    data_source::{DataSource, Source},
    data_types::schema::Schema,
};

#[derive(Clone)]
pub(crate) struct Scan {
    pub(crate) path: String,
    pub(crate) data_source: Source,
    pub(crate) projection: Vec<String>,
}

impl LogicalPlan for Scan {
    fn schema(&self) -> Schema {
        if self.projection.is_empty() {
            self.data_source.get_schema().clone()
        } else {
            self.data_source
                .get_schema()
                .select(self.projection.iter().map(|s| s.as_str()).collect())
        }
    }

    fn children(&self) -> Vec<Plan> {
        vec![]
    }
}

impl Display for Scan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.projection.is_empty() {
            write!(f, "Scan: {}; projection=None", self.path)
        } else {
            write!(
                f,
                "Scan: {}; projection=[{}]",
                self.path,
                self.projection
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
    }
}

impl Scan {
    pub(crate) fn new(path: String, data_source: Source, projection: Vec<String>) -> Self {
        Scan {
            path,
            data_source,
            projection,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scan;
    use crate::{data_source::DataSource, logical_plan::plan::LogicalPlan, util::get_data_source};

    #[test]
    fn test_schema_without_projection() {
        let (path, csv_data_source) = get_data_source();
        let schema = csv_data_source.get_schema().clone();
        let plan = Scan::new(path, csv_data_source, vec![]);
        assert_eq!(plan.schema(), schema);
    }

    #[test]
    fn test_schema_with_projection() {
        let (path, csv_data_source) = get_data_source();
        let schema = csv_data_source.get_schema().select(vec!["c1", "c2"]);
        let plan = Scan::new(
            path,
            csv_data_source,
            vec!["c1".to_string(), "c2".to_string()],
        );
        assert_eq!(plan.schema(), schema);
    }

    #[test]
    fn test_children() {
        let (path, csv_data_source) = get_data_source();
        let plan = Scan::new(path, csv_data_source, vec![]);
        assert_eq!(plan.children().len(), 0);
    }

    #[test]
    fn test_display_without_projection() {
        let (path, csv_data_source) = get_data_source();
        let plan = Scan::new(path.clone(), csv_data_source, vec![]);
        assert_eq!(plan.to_string(), format!("Scan: {}; projection=None", path));
    }

    #[test]
    fn test_display_with_projection() {
        let (path, csv_data_source) = get_data_source();
        let plan = Scan::new(
            path.clone(),
            csv_data_source,
            vec!["c1".to_string(), "c2".to_string()],
        );
        assert_eq!(
            plan.to_string(),
            format!("Scan: {}; projection=[c1,c2]", path)
        );
    }
}
