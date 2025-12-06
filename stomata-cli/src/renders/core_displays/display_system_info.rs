use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::SystemInfo;

use crate::{
    renders::{core_displays::traits::Display, render_widgets::render_paragraph::paragraph_widget},
    structs::UIState,
};

impl Display for SystemInfo {
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        _ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()> {
        let mut system_info_str = format!(
            "\n\n\n\n\nOS name: {}\nOS version: {}\nKernel Version: {}\nHostname: {}",
            self.os_name, self.os_version, self.kernel_version, self.hostname
        );

        let helper_instructions = "\n\n\n\n\n\n\nSwitch Tabs: Use number keys OR Tab btn OR <-, -> arrow keys\nMove selector: Up. Down arrow keys\nSelect: Enter key";
        system_info_str.push_str(helper_instructions);
        let paragraph = paragraph_widget(&system_info_str, "System Info");
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            area,
        );
        Ok(())
    }
}
