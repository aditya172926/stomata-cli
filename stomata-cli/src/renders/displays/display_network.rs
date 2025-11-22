use ratatui::{Frame, layout::Rect};

use crate::renders::render_widgets::render_paragraph::paragraph_widget;

pub fn display_network_stats(frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
    let networks_string = "Displaying networks information";
    let paragraph_widget = paragraph_widget(networks_string, "Networks info");
    frame.render_widget(paragraph_widget, area);
    Ok(())
}
