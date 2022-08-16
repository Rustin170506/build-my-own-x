use arrow::datatypes::ArrowPrimitiveType;
use arrow::datatypes::*;

/// Specialized parsing implementations
/// used by csv reader
pub trait Parser: ArrowPrimitiveType {
    fn parse(string: &str) -> Option<Self::Native> {
        string.parse::<Self::Native>().ok()
    }
}

impl Parser for Float32Type {
    fn parse(string: &str) -> Option<f32> {
        lexical_core::parse(string.as_bytes()).ok()
    }
}

impl Parser for Float64Type {
    fn parse(string: &str) -> Option<f64> {
        lexical_core::parse(string.as_bytes()).ok()
    }
}

impl Parser for UInt64Type {}

impl Parser for UInt32Type {}

impl Parser for UInt16Type {}

impl Parser for UInt8Type {}

impl Parser for Int64Type {}

impl Parser for Int32Type {}

impl Parser for Int16Type {}

impl Parser for Int8Type {}
