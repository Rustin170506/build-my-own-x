use std::{
    any::{self, Any},
    collections::{hash_map::DefaultHasher, BTreeMap},
    fmt::Display,
    hash::{Hash, Hasher},
    rc::Rc,
};

use super::{
    aggregate::{Accumulator, AggregateExpr},
    expr::{Expr, PhysicalExpr},
    plan::{PhysicalPlan, Plan},
};
use crate::data_types::{
    arrow_field_array::ArrowFieldArray,
    column_array::{ArrayRef, DataType},
    record_batch::RecordBatch,
    schema::Schema,
};

use anyhow::{anyhow, Error, Result};
use arrow::array::{
    Array, ArrayBuilder, Float32Builder, Float64Builder, Int32Builder, Int64Builder,
};
use ordered_float::OrderedFloat;

// AccumulatorMap is a map storing the accumulators for each group.
// GroupKey -> (GroupValues, Accumulators).
type AccumulatorMap = BTreeMap<u64, (Vec<Box<dyn Any>>, Vec<Accumulator>)>;

/// HashExec will hash the input record batches and group them by the hash value.
pub(crate) struct HashExec {
    input: Box<Plan>,
    schema: Schema,
    group_expr: Vec<Expr>,
    aggregate_expr: Vec<AggregateExpr>,
}

impl HashExec {
    pub(crate) fn new(
        input: Plan,
        schema: Schema,
        group_expr: Vec<Expr>,
        aggregate_expr: Vec<AggregateExpr>,
    ) -> Self {
        Self {
            input: Box::new(input),
            schema,
            group_expr,
            aggregate_expr,
        }
    }

    /// Create array builders by the schema.
    fn create_builders(&self, row_count: usize) -> Vec<Box<dyn ArrayBuilder>> {
        self.schema
            .fields
            .iter()
            .map(|f| match f.data_type {
                DataType::Int32 => Box::new(Int32Builder::new(row_count)) as Box<dyn ArrayBuilder>,
                DataType::Int64 => Box::new(Int64Builder::new(row_count)),
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
        let mut accumulator_map: AccumulatorMap = BTreeMap::new();

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
            for row_index in 0..b.row_count() {
                // Get the group values to calculate the hash.
                let values = group_keys
                    .iter()
                    .map(|a| a.get_value(row_index))
                    .collect::<Result<Vec<Box<dyn Any>>, _>>()?;
                let hash = create_hash(&values);
                // Get or insert the accumulators for the group.
                let accumulators = accumulator_map.entry(hash).or_insert_with(|| {
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
                    let value = aggr_input_values[i].get_value(row_index)?;
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
        if value.is::<i32>() {
            hasher.write_i32(*value.downcast_ref::<i32>().unwrap());
        } else if value.is::<i64>() {
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
    if build.as_any().is::<Int32Builder>() {
        build
            .as_any_mut()
            .downcast_mut::<Int32Builder>()
            .unwrap()
            .append_value(*value.downcast_ref::<i32>().unwrap());
    } else if build.as_any().is::<Int64Builder>() {
        build
            .as_any_mut()
            .downcast_mut::<Int64Builder>()
            .unwrap()
            .append_value(*value.downcast_ref::<i64>().unwrap());
    } else if build.as_any().is::<Float32Builder>() {
        build
            .as_any_mut()
            .downcast_mut::<Float32Builder>()
            .unwrap()
            .append_value(*value.downcast_ref::<f32>().unwrap());
    } else if build.as_any().is::<Float64Builder>() {
        build
            .as_any_mut()
            .downcast_mut::<Float64Builder>()
            .unwrap()
            .append_value(*value.downcast_ref::<f64>().unwrap());
    } else {
        unreachable!()
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        data_source::{csv_data_source::CsvDataSource, Source},
        data_types::schema::{self, Field},
        logical_plan::expr::AggregateFunction,
        physical_plan::{expr::Column, scan::ScanExec},
        test_util::rq_test_data,
    };

    fn get_hash_exec() -> HashExec {
        let data_path = rq_test_data("hash_test_filed.csv");
        let schema = Schema::new(vec![
            Field::new("c1".to_string(), DataType::Int32),
            Field::new("c2".to_string(), DataType::Int64),
            Field::new("c3".to_string(), DataType::Float32),
            Field::new("c4".to_string(), DataType::Float64),
        ]);
        let csv_data_source = CsvDataSource::new(data_path, schema, 4);
        let scan = ScanExec::new(
            Source::Csv(csv_data_source),
            vec![
                "c1".to_string(),
                "c2".to_string(),
                "c3".to_string(),
                "c4".to_string(),
            ],
        );
        let group_expr = vec![Expr::Column(Column::new(1)), Expr::Column(Column::new(2))];
        let aggregate_expr = vec![AggregateExpr::new(
            Expr::Column(Column::new(1)),
            AggregateFunction::Sum,
        )];

        // The final result should be:
        let schema = Schema::new(vec![
            Field::new("c2".to_string(), DataType::Int64),
            Field::new("c3".to_string(), DataType::Float32),
            Field::new("c5".to_string(), DataType::Int64),
        ]);

        HashExec::new(Plan::Scan(scan), schema, group_expr, aggregate_expr)
    }

    #[test]
    fn test_hash_execute() {
        let hash = get_hash_exec();
        let result = hash.execute().unwrap().next().unwrap();
        assert_eq!(result.row_count(), 3);
        assert_eq!(result.column_count(), 3);
        // Assert the first row.
        assert_eq!(
            result
                .field(0)
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &1
        );
        assert_eq!(
            result
                .field(0)
                .get_value(1)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &2
        );
        assert_eq!(
            result
                .field(0)
                .get_value(2)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &3
        );
        // Assert the aggregate result.
        assert_eq!(
            result
                .field(2)
                .get_value(0)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &2
        );
        assert_eq!(
            result
                .field(2)
                .get_value(1)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &2
        );
        assert_eq!(
            result
                .field(2)
                .get_value(2)
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap(),
            &3
        );
    }

    #[test]
    fn test_hash_display() {
        let hash = get_hash_exec();

        assert_eq!(
            format!("{}", hash),
            "HashAggregateExec: groupExpr=#1, #2, aggrExpr=SUM(#1)"
        );
    }
}
