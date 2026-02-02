//! Process list display implementation
//!
//! Provides an interactive table view of all running processes with sortable
//! columns and keyboard navigation. Users can select processes to view detailed
//! information about individual processes.

use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::process::metrics::ProcessData;

use crate::{
    renders::{core_displays::traits::Display, render_widgets::render_table::render_table},
    structs::UIState,
};

/// Display implementation for process list
///
/// Renders all running processes as an interactive table with columns for
/// PID, name, CPU usage, memory usage, and status. The table supports
/// keyboard navigation and selection tracking for drilling down into
/// individual process details.
impl Display<UIState> for Vec<ProcessData> {
    /// Renders the process list as an interactive table
    ///
    /// Creates a scrollable, selectable table showing all processes with
    /// key metrics. The currently selected process PID is tracked in the
    /// UI state for navigation to detailed process views.
    ///
    /// # Table Structure
    ///
    /// ```text
    /// ┌─────────────────────────────────────────┐
    /// │               Processes                  │
    /// ├─────┬────────────┬─────┬────────┬───────┤
    /// │ PID │ Name       │ CPU │ Memory │Status │
    /// ├─────┼────────────┼─────┼────────┼───────┤
    /// │ 1   │ systemd    │ 0.1 │  45 MB │Running│
    /// │ 123 │ firefox    │ 5.2 │ 850 MB │Running│
    /// │ 456 │ code       │ 3.1 │ 420 MB │Sleeping│
    /// │ ... │ ...        │ ... │ ...    │ ...   │
    /// └─────┴────────────┴─────┴────────┴───────┘
    /// ```
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    /// * `area` - The rectangular area allocated for the process table
    /// * `ui_state` - Required UI state for selection tracking. Must be `Some`.
    ///   Contains the table selection state and stores the selected process PID.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Rendering completed successfully
    ///
    /// # Table Columns
    ///
    /// - **PID**: Process ID (unique system identifier)
    /// - **Name**: Process name/command
    /// - **CPU**: Current CPU usage percentage
    /// - **Memory**: Current memory consumption
    /// - **Status**: Process state (Running, Sleeping, Stopped, Zombie, etc.)
    ///
    /// # Interactive Features
    ///
    /// - **Keyboard Navigation**: Up/Down arrow keys to select processes
    /// - **Selection Tracking**: Selected PID is stored in `ui_state.process_table.selected_pid`
    /// - **Enter Key**: Press Enter on a selected process to view detailed metrics
    ///
    /// # State Management
    ///
    /// The UI state maintains:
    /// - `process_table.process_list`: Ratatui's `TableState` for row selection
    /// - `process_table.selected_pid`: The PID of the currently selected process
    /// - `process_table.process_count`: Total number of processes for bounds checking
    ///
    /// When a process is selected, its PID is stored for navigation to the
    /// detailed single-process view (accessible via Enter key).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use stomata_core::collectors::process::metrics::ProcessData;
    /// use stomata::renders::core_displays::traits::Display;
    ///
    /// let processes: Vec<ProcessData> = fetch_all_processes();
    /// processes.display(frame, area, Some(&mut ui_state))?;
    ///
    /// // After user navigates and presses Enter:
    /// if let Some(selected_pid) = ui_state.process_table.selected_pid {
    ///     // Navigate to detailed process view
    /// }
    /// ```
    ///
    /// # Behavior Without UI State
    ///
    /// If `ui_state` is `None`, the table will not be rendered. The UI state
    /// is required for maintaining selection state across render cycles.
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()> {
        let headers = vec!["PID", "Name", "CPU", "Memory", "Status"];
        let table_widget = render_table(headers, &self, "Processes");
        if let Some(ui_state) = ui_state {
            if let Some(selected_index) = ui_state.process_table.process_list.selected() {
                ui_state.process_table.selected_pid = Some(self[selected_index].pid);
            };
            frame.render_stateful_widget(
                table_widget,
                area,
                &mut ui_state.process_table.process_list,
            );
        }
        Ok(())
    }
}
