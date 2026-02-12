//! System metrics display implementation
//!
//! Provides the visual rendering logic for real-time system resource metrics
//! including CPU, memory, and swap usage. This module implements the `Display`
//! trait for `SystemCollector` to render gauges and detailed statistics.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use stomata_core::collectors::system::metrics::SystemCollector;

use crate::{
    renders::{
        core_displays::traits::Display,
        render_widgets::{render_gauge::render_gauge, render_paragraph::paragraph_widget},
    },
    structs::UIState,
    utils::bytes_to_mb,
};

// Display implementation for system resource metrics
///
/// Renders a comprehensive view of system resources divided into four sections:
/// 1. Memory usage gauge
/// 2. Swap usage gauge
/// 3. CPU usage gauge
/// 4. Detailed statistics panels
///
/// The detailed statistics section is horizontally divided into three equal panels
/// showing memory info, swap info, and CPU count.
impl Display<()> for SystemCollector {
    /// Renders system metrics to the terminal frame
    ///
    /// Creates a vertical layout with visual gauges for quick assessment
    /// and detailed text panels for precise values. All memory values are
    /// converted from bytes to megabytes for better readability.
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    /// * `area` - The rectangular area allocated for system metrics display
    /// * `_ui_state` - Unused for this display (no interactive state needed)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Rendering completed successfully
    ///
    /// # Gauge Details
    ///
    /// - **Memory Gauge**: Shows used vs total memory in MB with percentage
    /// - **Swap Gauge**: Shows used vs total swap space in MB with percentage
    /// - **CPU Gauge**: Shows overall CPU utilization as a percentage (0-100%)
    ///
    /// # Statistics Panels
    ///
    /// - **Memory Info**: Exact bytes used/total and usage percentage
    /// - **Swap Info**: Exact bytes used/total and usage percentage
    /// - **CPU Count**: Number of logical CPU cores available
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use stomata_core::collectors::system::metrics::SystemCollector;
    /// use stomata::renders::core_displays::traits::Display;
    ///
    /// let collector = SystemCollector::new();
    /// collector.display(frame, area, None)?;
    /// ```
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        _ui_state: Option<&mut ()>,
    ) -> anyhow::Result<()> {
        let layout = Layout::vertical([
            Constraint::Percentage(23),
            Constraint::Percentage(23),
            Constraint::Percentage(24),
            Constraint::Percentage(30),
        ])
        .split(area);

        // render memory usage gauge
        frame.render_widget(
            render_gauge(
                bytes_to_mb(self.system_metrics.memory_used),
                bytes_to_mb(self.system_metrics.memory_total),
                "Memory Usage",
                "MB",
            ),
            layout[0],
        );

        // render swap usage gauge
        frame.render_widget(
            render_gauge(
                bytes_to_mb(self.system_metrics.swap_used),
                bytes_to_mb(self.system_metrics.swap_total),
                "Swap Usage",
                "MB",
            ),
            layout[1],
        );

        // render cpu usage gauge
        frame.render_widget(
            render_gauge(
                self.system_metrics.cpu_usage as f64,
                100.0,
                "CPU Usage",
                "%",
            ),
            layout[2],
        );

        // --- PARAGRAPH ---
        let memory_used = self.system_metrics.memory_used as f64
            / self.system_metrics.memory_total as f64
            * 100.0;

        let text = format!(
            "Memory Used: {:.2} Bytes\nTotal Memory: {:.2} Bytes\nUsage: {:.2}%",
            self.system_metrics.memory_used, self.system_metrics.memory_total, memory_used,
        );

        let swap_used =
            self.system_metrics.swap_used as f64 / self.system_metrics.swap_total as f64 * 100.0;
        let text_swap = format!(
            "Swap Used: {:.2} Bytes\nTotal Swap: {:.2} Bytes\nUsage: {:.2}%",
            self.system_metrics.swap_used, self.system_metrics.swap_total, swap_used,
        );

        let processes_count_text = format!("CPU count: {}", self.system_metrics.cpu_count);
        let process_paragraph = paragraph_widget(processes_count_text, "Processes Count");

        let paragraph = paragraph_widget(text, "Memory Info");
        let swap_paragraph = paragraph_widget(text_swap, "Swap Info");

        let layout_paragraph = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(layout[3]);

        frame.render_widget(paragraph, layout_paragraph[0]);
        frame.render_widget(swap_paragraph, layout_paragraph[1]);
        frame.render_widget(process_paragraph, layout_paragraph[2]);

        Ok(())
    }
}
