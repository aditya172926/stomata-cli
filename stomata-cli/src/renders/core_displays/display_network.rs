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

impl Display for NetworkMetrics {
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
                    paragraph_widget(&interface_metadata_info, &interface.name);

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
