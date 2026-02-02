use std::time::{Duration, Instant};

use crate::{
    features::run_feature,
    renders::core_displays::display_app::App,
    structs::{AppState, Cli, StomataState},
};
use clap::Parser;
use ratatui::crossterm::event::{self, Event};

mod constants;
mod features;
mod renders;
mod stomata_state;
mod structs;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let enable_ui = cli.interactive;
    let mut app = StomataState::new();

    if app.available_features.is_empty() {
        eprintln!("Error: No features enabled. Build with at least one feature:");
        return Ok(());
    }

    if enable_ui {
        let mut terminal = ratatui::init();
        loop {
            match app.state {
                AppState::FeatureSelection => {
                    terminal.draw(|frame| app.render_feature_selection(frame))?;
                    if let Event::Key(key) = event::read()? {
                        if !app.handle_feature_selection(key) {
                            break; // User quit
                        }
                    }
                }
                AppState::RunningFeature(feature) => {
                    // Run the selected feature
                    match run_feature(feature, &cli, Some(&mut terminal)).await {
                        Ok(render) => {
                            if !render {
                                app.state = AppState::FeatureSelection;
                            }
                        }
                        Err(_) => {
                            eprint!("Error in rendering feature");
                            app.state = AppState::FeatureSelection;
                        }
                    }
                }
            }
        }
        ratatui::restore();
    } else {
        let cli_clone = cli.clone();
        let cli_feature = cli_clone.feature;
        match cli_feature {
            Some(feature) => {
                if let Some(feature) = app.available_features.get(&feature) {
                    run_feature(*feature, &cli, None);
                };
            }
            None => println!("No feature selected"),
        }
    }
    Ok(())
}
