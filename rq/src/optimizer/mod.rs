pub(crate) mod rule;

use self::rule::{OptimizerRule, ProjectionPushDownRule};
use crate::logical_plan::plan::Plan;

/// Optimizer for logical plans.
pub(crate) struct Optimizer;

impl Optimizer {
    pub(crate) fn optimize(plan: &Plan) -> Plan {
        ProjectionPushDownRule::optimize(plan)
    }
}
