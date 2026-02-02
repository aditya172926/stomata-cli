//! Display traits for system monitoring UI components
//!
//! Defines the core trait interfaces that all display implementations must
//! follow. These traits ensure consistent rendering behavior across different
//! system information views.

use crate::structs::UIState;
use ratatui::{Frame, layout::Rect};

/// Core display trait for rendering system information to the terminal UI.
///
/// All display components implement this trait to provide a consistent
/// interface for rendering content to a ratatui frame. Implementations
/// handle their own layout and styling within the provided screen area.
///
/// # Examples
///
/// ```ignore
/// use crate::renders::core_displays::traits::Display;
///
/// impl Display for MyComponent {
///     fn display(
///         &self,
///         frame: &mut Frame,
///         area: Rect,
///         ui_state: Option<&mut UIState>,
///     ) -> anyhow::Result<()> {
///         // Render component to frame
///         Ok(())
///     }
/// }
/// ```
pub trait Display<S> {
    /// Renders the component to the given frame area.
    ///
    /// # Arguments
    ///
    /// * `frame` - Mutable reference to the ratatui frame for rendering
    /// * `area` - Screen rectangle defining where to render the component
    /// * `ui_state` - Optional mutable UI state for managing interactions
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful rendering, or an error if rendering fails.
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut S>,
    ) -> anyhow::Result<()>;
}

/// Specialized display trait for rendering individual process metrics.
///
/// This trait extends the basic display functionality with process-specific
/// rendering that requires total system memory for calculating percentages
/// and relative usage statistics.
///
/// # Use Case
///
/// Implemented by components that show detailed metrics for a single process,
/// including memory usage percentages, CPU utilization, and other per-process
/// statistics that need system-wide context.
pub trait SingleProcessDisplay {
    /// Renders detailed metrics for a single process.
    ///
    /// # Arguments
    ///
    /// * `frame` - Mutable reference to the ratatui frame for rendering
    /// * `area` - Screen rectangle defining where to render the metrics
    /// * `total_memory` - Total system memory in bytes, used for percentage calculations
    /// * `ui_state` - Mutable UI state for managing view state and interactions
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful rendering, or an error if rendering fails.
    ///
    /// # Notes
    ///
    /// Unlike the general `Display` trait, this requires `ui_state` to be present
    /// (not optional) since process detail views always need state management.
    fn display_process_metrics(
        &self,
        frame: &mut Frame,
        area: Rect,
        total_memory: f64,
        ui_state: &mut UIState,
    ) -> anyhow::Result<()>;
}
