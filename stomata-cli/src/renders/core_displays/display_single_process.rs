//! Single process detailed view implementation
//!
//! Provides a comprehensive, multi-panel view of a single process including
//! basic information, resource usage gauges, disk I/O trends, and associated
//! tasks/threads. This is the detailed view accessible by pressing Enter on
//! a process in the process list.

use crate::{
    renders::{
        core_displays::traits::SingleProcessDisplay,
        render_widgets::{
            render_gauge::render_gauge, render_paragraph::paragraph_widget,
            render_sparkline::render_sparkline, render_table::render_table,
        },
    },
    structs::{SingleProcessUI, UIState},
    utils::bytes_to_mb,
};
use chrono::DateTime;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

/// Display implementation for detailed single process view
///
/// Creates a comprehensive dashboard for a single process with multiple panels
/// showing real-time metrics, historical trends, and task information. The
/// layout adapts based on whether the process has associated tasks/threads.
///
/// # Layout Variants
///
/// ## With Tasks (3 columns)
/// ```text
/// ┌─────────────┬─────────────┬─────────────┐
/// │ Basic Info  │ Extra Info  │   Tasks     │
/// │   (30%)     │             │   Table     │
/// ├─────────────┤ Disk Read   │             │
/// │ CPU Gauge   │ Sparkline   │             │
/// │   (35%)     │             │             │
/// │ Memory      │ Disk Write  │             │
/// │ Gauge (35%) │ Sparkline   │             │
/// └─────────────┴─────────────┴─────────────┘
///    33%            33%            33%
/// ```
///
/// ## Without Tasks (2 columns)
/// ```text
/// ┌─────────────┬─────────────┐
/// │ Basic Info  │ Extra Info  │
/// │   (30%)     │             │
/// ├─────────────┤ Disk Read   │
/// │ CPU Gauge   │ Sparkline   │
/// │   (35%)     │             │
/// │ Memory      │ Disk Write  │
/// │ Gauge (35%) │ Sparkline   │
/// └─────────────┴─────────────┘
///      50%            50%
/// ```
impl SingleProcessDisplay for SingleProcessUI<'_> {
    /// Renders detailed metrics for a single process
    ///
    /// Creates an adaptive layout with 2-3 columns depending on whether the
    /// process has tasks (linux only). Displays comprehensive information including basic
    /// metadata, resource usage, disk I/O trends, and associated threads.
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    /// * `area` - The rectangular area allocated for the process view
    /// * `total_memory` - Total system memory in MB (for memory gauge scaling)
    /// * `ui_state` - UI state containing disk I/O history buffers for sparklines
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Rendering completed successfully
    ///
    /// # State Management
    ///
    /// The `ui_state.single_process_disk_usage` maintains rolling buffers of:
    /// - `disk_read_usage`: Historical read byte rates for sparkline
    /// - `disk_write_usage`: Historical write byte rates for sparkline
    ///
    /// These buffers are automatically updated when the display is rendered,
    /// providing smooth animated sparkline charts of disk activity.
    ///
    /// # Memory Calculation
    ///
    /// The memory gauge shows:
    /// - Used: Process memory consumption in MB
    /// - Total: System total memory (passed as parameter)
    /// - This provides context for how much of system memory the process uses
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use stomata::structs::SingleProcessUI;
    /// use stomata::renders::core_displays::traits::SingleProcessDisplay;
    ///
    /// let process_ui = SingleProcessUI { data: process_data };
    /// let total_memory_mb = bytes_to_mb(system.total_memory());
    ///
    /// process_ui.display_process_metrics(
    ///     frame,
    ///     area,
    ///     total_memory_mb,
    ///     &mut ui_state
    /// )?;
    /// ```
    ///
    /// # Navigation
    ///
    /// Users typically reach this view by:
    /// 1. Navigating the process list (Page::Processes)
    /// 2. Selecting a process with Up/Down arrows
    /// 3. Pressing Enter to view detailed metrics
    fn display_process_metrics(
        &self,
        frame: &mut Frame,
        area: Rect,
        total_memory: f64,
        ui_state: &mut UIState,
    ) -> anyhow::Result<()> {
        let constraints: Vec<Constraint>;

        let tasks = &self.data.tasks;
        if tasks.len() > 0 {
            constraints = vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ];
        } else {
            constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        }

        let primary_layout = Layout::horizontal(&constraints).split(area);
        let secondary_layout =
            Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(primary_layout[0]);

        let p_info = format!(
            "PID: {}\nName: {}\nStatus: {}",
            self.data.basic_process_data.pid,
            self.data.basic_process_data.name,
            self.data.basic_process_data.status
        );

        let basic_info_paragraph = paragraph_widget(p_info, "Basic Task info");
        let start_timestamp = DateTime::from_timestamp_secs(self.data.start_time as i64).unwrap();
        let mut extra_info = format!(
            "Start time: {:?}\nRunning time: {}\nCWD: {}\nTotal written bytes: {}\nTotal read bytes: {}\nLatest Read bytes: {}\nLatest write bytes: {}",
            start_timestamp,
            self.data.running_time,
            self.data
                .current_working_dir
                .clone()
                .unwrap_or(String::new()),
            self.data.disk_usage.total_written_bytes,
            self.data.disk_usage.total_read_bytes,
            self.data.disk_usage.read_bytes,
            self.data.disk_usage.written_bytes
        );
        if let Some(parent_pid) = self.data.parent_pid {
            extra_info.push_str(&format!("\nParent PID: {}", parent_pid.as_u32()));
        };
        let extra_info_paragraph = paragraph_widget(extra_info, "More info");
        let cpu_gauge = render_gauge(
            self.data.basic_process_data.cpu_usage.into(),
            100.0,
            "CPU",
            "%",
        );

        frame.render_widget(
            basic_info_paragraph.alignment(ratatui::layout::Alignment::Left),
            secondary_layout[0],
        );

        // ---- Primary 1 layout -----
        let primary_1_layout = Layout::vertical([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(primary_layout[1]);

        let disk_read_data = ui_state
            .single_process_disk_usage
            .disk_read_usage
            .make_contiguous();
        let disk_write_data = ui_state
            .single_process_disk_usage
            .disk_write_usage
            .make_contiguous();
        let disk_read_sparkline = render_sparkline(disk_read_data, "Disk Read Bytes");
        let disk_write_sparkline = render_sparkline(disk_write_data, "Disk Write Bytes");

        frame.render_widget(extra_info_paragraph, primary_1_layout[0]);
        frame.render_widget(disk_read_sparkline, primary_1_layout[1]);
        frame.render_widget(disk_write_sparkline, primary_1_layout[2]);

        //---- Conditional Render ----

        let tertiary_constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let process_memory_use = self.data.basic_process_data.memory;
        let memory_gauge = render_gauge(
            bytes_to_mb(process_memory_use),
            total_memory,
            "Memory",
            "MB",
        );

        let tertiary_layout = Layout::vertical(tertiary_constraints).split(secondary_layout[1]);
        frame.render_widget(cpu_gauge, tertiary_layout[0]);
        frame.render_widget(memory_gauge, tertiary_layout[1]);

        if tasks.len() > 0 {
            let task_headers = vec!["PID", "Name", "CPU", "Memory", "Status"];
            let task_widget = render_table(task_headers, &self.data.tasks, "Tasks");
            frame.render_widget(task_widget, primary_layout[2]);
        }
        Ok(())
    }
}
