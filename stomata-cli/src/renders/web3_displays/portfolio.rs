use anyhow::{Result, anyhow};
use ratatui::layout::{Constraint, Layout};
use stomata_web3::providers::{
    address::{AddressValidator, ValidationResult},
    portfolio::{service::get_portfolio, structs::Portfolio},
    rpc::structs::EVMProvider,
};

use crate::{
    features::web3::web3_feature::Web3UIState,
    renders::{core_displays::traits::Display, render_widgets::render_paragraph::paragraph_widget},
    structs::InputWidgetState,
};

impl Display<InputWidgetState> for Portfolio {
    fn display(
        &self,
        frame: &mut ratatui::Frame<'_>,
        area: ratatui::prelude::Rect,
        ui_state: Option<&mut InputWidgetState>,
    ) -> anyhow::Result<()> {
        let input_field_widget = if let Some(state) = ui_state {
            state
        } else {
            &mut InputWidgetState::new()
        };

        let layout =
            Layout::vertical([Constraint::Length(3), Constraint::Min(30)]).split(area);

        input_field_widget.render_input(layout[0], frame);

        // paragraph to render messages
        let mut data;
        if !input_field_widget.messages.is_empty() {
            data = paragraph_widget("stuff", "Portfolio");
            // let portfolio_data = get_portfolio_data(&input_field_widget.messages).await;
            // if let Ok(portfolio) = portfolio_data {
            //     let portfolio_string = format!("Account Type: {:?}, Native Balance: {:?}, Transaction count: {:?}", portfolio.account_type, portfolio.native_balance, portfolio.transaction_count);
            //     data = paragraph_widget("stuff", "Portfolio");
            // } else {
            //     data = paragraph_widget("Data not found", "Error");
            // }
        } else {
            data = paragraph_widget("Input address", "Info");
        }

        frame.render_widget(data, layout[1]);
        Ok(())
    }
}

pub async fn get_portfolio_data(address: &str) -> Result<Portfolio> {
    let validated_address = AddressValidator::validate(address);
    match validated_address {
        ValidationResult::Valid { checksummed } => {
            let provider = EVMProvider::new(checksummed, String::from("https://rpc.fullsend.to"));
            let portfolio = get_portfolio(provider).await;
            portfolio
        }
        _ => Err(anyhow!("Error in validating address")),
    }
}
