pub(crate) mod rule;

use self::rule::{OptimizerRule, ProjectionPushDownRule};
use crate::logical_plan::plan::Plan;

/// Optimizer for logical plans.
pub(crate) struct Optimizer {}

impl Optimizer {
    fn optimize(&self, plan: Plan) -> Plan {
        let rule = ProjectionPushDownRule;
        rule.optimize(&plan)
    }
}
