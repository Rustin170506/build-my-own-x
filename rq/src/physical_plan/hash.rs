use super::{
    aggregate_expr::{Accumulator, AggregateExpr},
    expr::{Expr, PhysicalExpr},
    plan::{PhysicalPlan, Plan},
};
use crate::data_types::{
    arrow_field_array::ArrowFieldArray, column_array::ArrayRef, record_batch::RecordBatch,
    schema::Schema,
};
use anyhow::{anyhow, Error, Result};
use arrow::{
    array::{Array, ArrayBuilder, Float32Builder, Float64Builder, Int64Builder},
    datatypes::DataType,
};
use ordered_float::OrderedFloat;
use std::{
    any::{self, Any},
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Display,
    hash::{Hash, Hasher},
    rc::Rc,
};

// AccumulatorMap is a map storing the accumulators for each group.
// GroupKey -> (GroupValues, Accumulators).
type AccumulatorMap = HashMap<u64, (Vec<Box<dyn Any>>, Vec<Accumulator>)>;

/// HashExec will hash the input record batches and group them by the hash value.
pub(crate) struct HashExec {
    input: Box<Plan>,
    schema: Schema,
    group_expr: Vec<Expr>,
    aggregate_expr: Vec<AggregateExpr>,
}

impl HashExec {
    /// Create array builders by the schema.
    fn create_builders(&self, row_count: usize) -> Vec<Box<dyn ArrayBuilder>> {
        self.schema
            .fields
            .iter()
            .map(|f| match f.data_type {
                DataType::Int64 => Box::new(Int64Builder::new(row_count)) as Box<dyn ArrayBuilder>,
                DataType::Float32 => Box::new(Float32Builder::new(row_count)),
                DataType::Float64 => Box::new(Float64Builder::new(row_count)),
                _ => unreachable!(),
            })
            .collect()
    }
}

impl PhysicalPlan for HashExec {
    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn execute(&self) -> anyhow::Result<Box<dyn Iterator<Item = RecordBatch> + '_>> {
        let mut accumulator_map: AccumulatorMap = HashMap::new();

        // For each batch from the input executor.
        for b in self.input.execute()? {
            // Evaluate the group expressions.
            let group_keys: Vec<ArrayRef> = self
                .group_expr
                .iter()
                .map(|e| e.evaluate(&b))
                .collect::<Result<Vec<ArrayRef>, _>>()?;
            // Evaluate the aggregate expressions.
            let aggr_input_values: Vec<ArrayRef> = self
                .aggregate_expr
                .iter()
                .map(|e| e.input_expr().evaluate(&b))
                .collect::<Result<Vec<ArrayRef>, _>>()?;
            // For each row in the batch.
            for i in 0..b.row_count() {
                // Get the group values to calculate the hash.
                let values = group_keys
                    .iter()
                    .map(|a| a.get_value(i))
                    .collect::<Result<Vec<Box<dyn Any>>, _>>()?;
                let hash = create_hash(&values);
                // Get or insert the accumulators for the group.
                let accumulators = accumulator_map.entry(1).or_insert_with(|| {
                    (
                        values,
                        self.aggregate_expr
                            .iter()
                            .map(|a| a.create_accumulator())
                            .collect(),
                    )
                });
                // Preform the aggregate operation.
                for (i, acc) in accumulators.1.iter_mut().enumerate() {
                    let value = aggr_input_values[i].get_value(i)?;
                    acc.accumulate(Some(value));
                }
            }
        }
        // Create the output record batches.
        let mut builders = self.create_builders(accumulator_map.len());

        accumulator_map
            .iter()
            .enumerate()
            .for_each(|(row_index, (_, (values, accumulators)))| {
                self.group_expr
                    .iter()
                    .enumerate()
                    .for_each(|(i, _)| append_value(&mut builders[i], &values[i]));
                self.aggregate_expr.iter().enumerate().for_each(|(i, _)| {
                    append_value(
                        &mut builders[self.group_expr.len() + i],
                        accumulators[i].final_value().as_ref().unwrap(),
                    )
                });
            });
        let fields: Vec<ArrayRef> = builders
            .iter_mut()
            .map(|b| Rc::new(ArrowFieldArray::new(Box::new(b.finish().clone()))) as ArrayRef)
            .collect();
        Ok(Box::new(
            vec![RecordBatch::new(self.schema.clone(), fields)].into_iter(),
        ))
    }

    fn children(&self) -> Vec<&Plan> {
        vec![&self.input]
    }
}

/// Create a hash value for the group key.
fn create_hash(values: &Vec<Box<dyn Any>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for value in values {
        if value.is::<i64>() {
            hasher.write_i64(*value.downcast_ref::<i64>().unwrap());
        } else if value.is::<f32>() {
            let ft = OrderedFloat(*value.downcast_ref::<f32>().unwrap());
            ft.hash(&mut hasher);
        } else if value.is::<f64>() {
            let ft = OrderedFloat(*value.downcast_ref::<f64>().unwrap());
            ft.hash(&mut hasher);
        } else {
            unreachable!()
        }
    }
    hasher.finish()
}

// Append the value to the array builder.
fn append_value(build: &mut Box<dyn ArrayBuilder>, value: &Box<dyn Any>) {
    if build.as_any().is::<Int64Builder>() {
        build
            .as_any_mut()
            .downcast_mut::<Int64Builder>()
            .unwrap()
            .append_value(*value.downcast_ref::<i64>().unwrap());
    }
}

impl Display for HashExec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HashAggregateExec: groupExpr={}, aggrExpr={}",
            self.group_expr
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.aggregate_expr
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}
