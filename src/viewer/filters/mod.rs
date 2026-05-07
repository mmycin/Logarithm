/// Log entry filtering logic.
/// 
/// Contains functions for filtering and processing log entries.

mod apply_filters;
mod level_propagation;

pub use apply_filters::apply_filters;
pub use level_propagation::propagate_levels;
