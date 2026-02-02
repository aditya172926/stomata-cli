//! System information display implementation
//!
//! Provides a centered view of core system details including OS information,
//! kernel version, and hostname, along with keyboard navigation instructions
//! for the UI.

use ratatui::{Frame, layout::Rect};
use stomata_core::collectors::SystemInfo;

use crate::{
    renders::{core_displays::traits::Display, render_widgets::render_paragraph::paragraph_widget},
    structs::UIState,
};

/// Display implementation for system information.
///
/// Renders a centered paragraph containing:
/// - Operating system name and version
/// - Kernel version
/// - System hostname
/// - UI navigation helper text
///
/// The display uses vertical spacing for visual balance and center alignment
/// for improved readability.
impl Display<()> for SystemInfo {
    /// Renders the system information to the terminal UI.
    ///
    /// # Arguments
    ///
    /// * `frame` - Mutable reference to the ratatui frame for rendering
    /// * `area` - Screen area where the system info should be displayed
    /// * `_ui_state` - Unused UI state parameter (reserved for future use)
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful rendering, or an error if rendering fails.
    ///
    /// # Display Layout
    ///
    /// The output includes:
    /// - Top padding with newlines for vertical centering
    /// - System details (OS, kernel, hostname)
    /// - Bottom padding before helper text
    /// - Keyboard navigation instructions:
    ///   - Switch tabs: Number keys, Tab, or arrow keys (←/→)
    ///   - Move selector: Arrow keys (↑/↓)
    ///   - Select item: Enter key
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        _ui_state: Option<&mut ()>,
    ) -> anyhow::Result<()> {
        let logo = r#"
███████╗████████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗ █████╗ 
██╔════╝╚══██╔══╝██╔═══██╗████╗ ████║██╔══██╗╚══██╔══╝██╔══██╗
███████╗   ██║   ██║   ██║██╔████╔██║███████║   ██║   ███████║
╚════██║   ██║   ██║   ██║██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
███████║   ██║   ╚██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
╚══════╝   ╚═╝    ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝
"#;

        let mut system_info_str = format!(
            "\n{logo}\n\nOS name: {}\nOS version: {}\nKernel Version: {}\nHostname: {}",
            self.os_name, self.os_version, self.kernel_version, self.hostname
        );

        let helper_instructions = "\n\n\nSwitch Tabs: Use number keys OR Tab btn OR <-, -> arrow keys\nMove selector: Up. Down arrow keys\nSelect: Enter key";
        system_info_str.push_str(helper_instructions);
        let paragraph = paragraph_widget(&system_info_str, "System Info");
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            area,
        );
        Ok(())
    }
}
