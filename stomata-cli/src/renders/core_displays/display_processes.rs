use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::process::metrics::ProcessData;

use crate::{
    renders::{core_displays::traits::Display, render_widgets::render_table::render_table},
    structs::UIState,
};

impl Display for Vec<ProcessData> {
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
