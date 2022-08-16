pub mod rule;

use self::rule::{OptimizerRule, ProjectionPushDownRule};
use crate::logical_plan::plan::Plan;

/// Optimizer for logical plans.
pub struct Optimizer;

impl Optimizer {
    pub fn optimize(plan: &Plan) -> Plan {
        ProjectionPushDownRule::optimize(plan)
    }
}
