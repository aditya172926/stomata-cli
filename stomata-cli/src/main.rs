use std::time::{Duration, Instant};

use crate::{renders::core_displays::display_app::App, structs::Cli};
use clap::Parser;
use ratatui::crossterm::event::{self, Event};

mod constants;
mod renders;
mod structs;
mod utils;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let store_metrics_data = cli.store;
    let mut app = App::new(store_metrics_data);
    let mut terminal = ratatui::init();

    // get the refresh interval from the cli arg. Default 1000 ms
    let refresh_interval = Duration::from_millis(cli.interval);
    let mut last_tick = Instant::now();

    // main render loop
    while app.render {
        let timeout = refresh_interval
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        // poll for inputs only until timeout
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // handle events
                app.handle_events(key)?;
                // redraw immediately after an event
                terminal.draw(|frame| app.render(frame))?;
            }
        }

        if last_tick.elapsed() >= refresh_interval {
            // draw
            terminal.draw(|frame| app.render(frame))?;
            last_tick = Instant::now();
        }
    }

    ratatui::restore();
    Ok(())
}
