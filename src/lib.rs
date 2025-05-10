#![feature(array_ptr_get)]

mod node;
mod partial_dependence;
mod shapley;

// Modules
pub mod bin;
pub mod binning;
pub mod booster;
pub mod conformal;
pub mod constants;
pub mod constraints;
pub mod data;
pub mod errors;
pub mod grower;
pub mod histogram;
#[path = "metrics/mod.rs"]
pub mod performance_metrics;
pub mod objective;
pub mod prune;
pub mod sampler;
pub mod splitter;
pub mod tree;
pub mod utils;

// Individual classes, and functions
pub use booster::booster::PerpetualBooster;
pub use booster::multi_output::MultiOutputBooster;
pub use data::Matrix;
