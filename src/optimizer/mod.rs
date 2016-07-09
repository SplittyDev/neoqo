pub mod optimizer;
pub use optimizer::optimizer::{Optimizer, OptimizerPass};
pub const OPTIMIZED_VALUE: &'static str = "__optimizer_generated";
