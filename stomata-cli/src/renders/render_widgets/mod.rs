//! Reusable widget rendering utilities
//!
//! Provides helper functions and builders for creating styled ratatui widgets.
//! These modules abstract common widget configurations and styling patterns
//! to ensure visual consistency across the application.
//!
//! # Modules
//!
//! - `render_bar` - Bar chart widgets for categorical data visualization
//! - `render_gauge` - Progress gauges for percentage-based metrics
//! - `render_paragraph` - Text paragraph widgets with borders and titles
//! - `render_sparkline` - Compact line charts for time-series data
//! - `render_table` - Tabular data display with sortable columns

pub mod render_bar;
pub mod render_gauge;
pub mod render_input;
pub mod render_paragraph;
pub mod render_sparkline;
pub mod render_table;
