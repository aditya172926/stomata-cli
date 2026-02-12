//! Network metrics display implementation
//!
//! Provides real-time visualization of network interface statistics including
//! traffic rates, packet counts, and error rates. Each network interface gets
//! its own column with metadata and sparkline charts showing traffic trends.

use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use stomata_core::NetworkMetrics;

use crate::{
    renders::{
        core_displays::traits::Display,
        render_widgets::{render_paragraph::paragraph_widget, render_sparkline::render_sparkline},
    },
    structs::{NetworkInterfaceData, UIState},
};

/// Display implementation for network interface metrics
///
/// Renders a dynamic multi-column layout where each network interface
/// (eth0, wlan0, etc.) gets its own vertical section containing:
/// - Interface metadata panel (cumulative stats)
/// - Four sparkline charts showing traffic history
///
/// The display automatically adapts to the number of active interfaces,
/// distributing screen space equally among them.
impl Display<UIState> for NetworkMetrics {
    /// Renders network metrics for all active interfaces
    ///
    /// Creates a dynamic layout that scales with the number of network interfaces.
    /// Each interface displays both current statistics and historical trends.
    ///
    /// # Layout Structure
    ///
    /// ```text
    /// ┌────────────────────────────────────────┐
    /// │  Interface 1  │  Interface 2  │  ...   │ (8 lines)
    /// │   Metadata    │   Metadata    │  ...   │
    /// ├────────────────────────────────────────┤
    /// │  Bytes RX     │  Bytes RX     │  ...   │
    /// │  ▁▂▃▅▇█      │  ▁▂▃▅▇█      │  ...   │
    /// ├────────────────────────────────────────┤
    /// │  Bytes TX     │  Bytes TX     │  ...   │
    /// │  ▁▂▃▅▇█      │  ▁▂▃▅▇█      │  ...   │
    /// ├────────────────────────────────────────┤
    /// │  Packets RX   │  Packets RX   │  ...   │
    /// │  ▁▂▃▅▇█      │  ▁▂▃▅▇█      │  ...   │
    /// ├────────────────────────────────────────┤
    /// │  Packets TX   │  Packets TX   │  ...   │
    /// │  ▁▂▃▅▇█      │  ▁▂▃▅▇█      │  ...   │
    /// └────────────────────────────────────────┘
    /// ```
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    /// * `area` - The rectangular area allocated for network metrics
    /// * `ui_state` - Required UI state for storing historical data. Must be `Some`.
    ///   The state maintains a `HashMap` of interface data for sparkline history.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Rendering completed successfully
    ///
    /// # Metadata Panel Contents
    ///
    /// Each interface's metadata panel shows cumulative statistics:
    /// - Total bytes received (since boot/interface up)
    /// - Total bytes transmitted
    /// - Total packets received
    /// - Total packets transmitted
    /// - Total receive errors
    /// - Total transmit errors
    ///
    /// # Sparkline Charts
    ///
    /// Four sparkline charts per interface showing recent trends:
    /// 1. **Bytes Received**: Current receive rate with history
    /// 2. **Bytes Transmitted**: Current transmit rate with history
    /// 3. **Packets Received**: Current packet receive rate with history
    /// 4. **Packets Transmitted**: Current packet transmit rate with history
    ///
    /// Each sparkline displays the most recent data point in the title
    /// and shows historical trend as a mini ASCII chart.
    ///
    /// # State Management
    ///
    /// Historical data for sparklines is maintained in `ui_state.networks_state`,
    /// which is a `HashMap<String, NetworkInterfaceData>` keyed by interface name.
    /// Each interface maintains a rolling buffer of recent values for smooth
    /// trend visualization.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use stomata_core::NetworkMetrics;
    /// use stomata::renders::core_displays::traits::Display;
    ///
    /// let network_metrics = NetworkMetrics::fetch();
    /// network_metrics.display(frame, area, Some(&mut ui_state))?;
    /// ```
    ///
    /// # Panics
    ///
    /// This implementation expects `ui_state` to be `Some`. If `None` is passed,
    /// no rendering will occur (gracefully handles the case but won't display anything).
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()> {
        let parent_layout =
            Layout::vertical([Constraint::Length(8), Constraint::Min(1)]).split(area);

        let number_of_interfaces: u16 = self.interfaces.len().try_into().unwrap_or(5);
        let constraints =
            vec![Constraint::Percentage(100 / number_of_interfaces); number_of_interfaces.into()];

        let para_layout = Layout::horizontal(&constraints).split(parent_layout[0]);
        let sparkline_layout = Layout::horizontal(&constraints).split(parent_layout[1]);

        if let Some(ui_state) = ui_state {
            let map = ui_state.networks_state.get_or_insert(HashMap::new());

            for (index, interface) in self.interfaces.iter().enumerate() {
                let iface = map
                    .entry(interface.name.clone())
                    .or_insert_with(NetworkInterfaceData::default);

                iface.update_network_history(interface);

                // -- para widgets --
                let interface_metadata_info = format!(
                    "Total Bytes received: {}\nTotal Bytes Transmitted: {}\nTotal Packets Received: {}\nTotal Packets Transmitted: {}\nTotal Errors on receive: {}\nTotal Errors on transmit: {}",
                    interface.total_bytes_received,
                    interface.total_bytes_transmitted,
                    interface.total_packets_received,
                    interface.total_packets_transmitted,
                    interface.total_errors_on_received,
                    interface.total_errors_on_transmitted
                );
                let metadata_para_widget =
                    paragraph_widget(interface_metadata_info, interface.name.clone());

                // -- sparkline widgets --
                let received_bytes_sparkline_title =
                    format!("Bytes received: {}", interface.bytes_received);

                let transmitted_bytes_sparkline_title =
                    format!("Bytes transmitted: {}", interface.bytes_transmitted);

                let packets_received_sparkline_title =
                    format!("Packets received: {}", interface.packets_received);

                let packets_transmitted_sparkline_title =
                    format!("Packets transmitted: {}", interface.packets_transmitted);

                //-- widgets --
                let sparkline_widgets = vec![
                    render_sparkline(
                        iface.received_bytes.make_contiguous(),
                        &received_bytes_sparkline_title,
                    ),
                    render_sparkline(
                        iface.transmitted_bytes.make_contiguous(),
                        &transmitted_bytes_sparkline_title,
                    ),
                    render_sparkline(
                        iface.packets_received.make_contiguous(),
                        &packets_received_sparkline_title,
                    ),
                    render_sparkline(
                        iface.packets_transmitted.make_contiguous(),
                        &packets_transmitted_sparkline_title,
                    ),
                ];

                let secondart_constraints =
                    vec![
                        Constraint::Percentage(100 / sparkline_widgets.len() as u16);
                        sparkline_widgets.len()
                    ];
                let secondary_layout =
                    Layout::vertical(&secondart_constraints).split(sparkline_layout[index]);

                for (widget_index, widget) in sparkline_widgets.iter().enumerate() {
                    frame.render_widget(widget, secondary_layout[widget_index]);
                }
                frame.render_widget(metadata_para_widget, para_layout[index]);
            }
        }
        Ok(())
    }
}
