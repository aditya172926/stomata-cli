use ratatui::layout::{Constraint, Layout};
use stomata_web3::providers::portfolio::structs::Portfolio;

use crate::{
    features::web3::web3_feature::Web3UIState,
    renders::{core_displays::traits::Display, render_widgets::render_paragraph::paragraph_widget},
    structs::InputWidgetState,
};

impl Display<InputWidgetState> for Portfolio {
    fn display(
        &self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        ui_state: Option<&mut InputWidgetState>,
    ) -> anyhow::Result<()> {
        let input_field_widget = if let Some(state) = ui_state {
            state
        } else {
            &mut InputWidgetState::new()
        };

        let layout =
            Layout::vertical([Constraint::Percentage(20), Constraint::Min(30)]).split(area);

        input_field_widget.render_input(layout[0], frame);

        // paragraph to render messages
        let mut data;
        if !input_field_widget.messages.is_empty() {
            data = paragraph_widget(&input_field_widget.messages, "Input Message");
        } else {
            data = paragraph_widget("Input address", "Info");
        }

        frame.render_widget(data, layout[1]);
        Ok(())
    }
}
