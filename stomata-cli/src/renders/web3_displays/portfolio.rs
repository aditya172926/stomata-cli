use stomata_web3::providers::portfolio::structs::Portfolio;

use crate::renders::{
    core_displays::traits::Display, render_widgets::render_paragraph::paragraph_widget,
};

impl Display for Portfolio {
    fn display(
        &self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        ui_state: Option<&mut crate::structs::UIState>,
    ) -> anyhow::Result<()> {
        let para = paragraph_widget(
            "Hi! We are adding more interactive features to Stomata Web3",
            "About",
        );
        frame.render_widget(para, area);

        Ok(())
    }
}
