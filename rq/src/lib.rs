// FIXME: This is a temporary solution to get the tests working.
#![allow(dead_code)]
#![allow(unused)]
pub(crate) mod data_source;
pub(crate) mod data_types;
pub(crate) mod execution;
pub(crate) mod logical_plan;
pub(crate) mod optimizer;
pub(crate) mod physical_plan;
pub(crate) mod query_planner;
#[cfg(test)]
mod util;
