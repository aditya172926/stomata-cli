//! Web3 feature execution and UI management
//!
//! This module handles both the interactive TUI (Terminal User Interface) and
//! CLI modes for Web3 tools. It provides a tabbed interface for interactive
//! mode and direct command execution for CLI mode.

use std::{
    io::Stdout,
    iter::once,
    process::exit,
    time::{Duration, Instant},
};

use clap::Parser;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};
use stomata_web3::providers::{
    portfolio::{service::get_portfolio, structs::Portfolio},
    rpc::structs::EVMProvider,
};

use crate::{
    features::web3::cli::{KeySubCommands, Web3Cli, Web3Tool},
    renders::{
        core_displays::traits::Display,
        render_widgets::render_paragraph::paragraph_widget,
        web3_displays::{
            address_validation::validate_address,
            key_encryption::{decrypt_key, delete_encrypted_key, encrypt_key, list_all_keys},
        },
    },
    structs::{Cli, InputWidgetState},
};

/// Available pages in the Web3 TUI
///
/// Each variant represents a different feature page that can be
/// displayed in the interactive terminal interface.
pub enum Web3Page {
    /// Page for validating Ethereum addresses
    AddressValidation,
    Portfolio,
}

impl Web3Page {
    /// Returns the titles of all available pages
    ///
    /// Used for rendering the tab bar in the TUI.
    pub fn titles() -> Vec<&'static str> {
        vec!["Address Validation", "Portfolio"]
    }

    /// Converts a tab index to the corresponding page
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based tab index
    ///
    /// # Returns
    ///
    /// The corresponding `Web3Page`, defaulting to `AddressValidation`
    /// for out-of-range indices.
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Web3Page::AddressValidation,
            1 => Web3Page::Portfolio,
            _ => Web3Page::AddressValidation,
        }
    }
}

/// UI-specific state for the Web3 interactive interface
///
/// Currently a placeholder for future UI state management.
#[derive(Default)]
pub struct Web3UIState {
    pub input_area_state: Option<InputWidgetState>,
    pub portfolio: Option<Portfolio>,
}

/// State manager for the Web3 feature
///
/// Manages the interactive TUI state including current page,
/// tab selection, and rendering lifecycle.
pub struct Web3State {
    /// Whether the UI should continue rendering
    pub render: bool,

    /// The currently active page
    pub current_page: Web3Page,

    /// Index of the currently selected tab
    pub tab_index: usize,

    /// Optional UI-specific state
    pub ui_state: Web3UIState,
}

impl Web3State {
    /// Creates a new Web3State with default values
    ///
    /// Initializes to the Address Validation page with rendering enabled.
    pub fn new() -> Self {
        Self {
            render: true,
            current_page: Web3Page::AddressValidation,
            tab_index: 0,
            ui_state: Web3UIState::default(),
        }
    }

    /// Advances to the next tab, wrapping around to the first tab
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % Web3Page::titles().len();
        self.current_page = Web3Page::from_index(self.tab_index);
    }

    /// Moves to the previous tab, wrapping around to the last tab
    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = Web3Page::titles().len() - 1;
        }
        self.current_page = Web3Page::from_index(self.tab_index);
    }

    /// Renders the current page to the terminal frame
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

        // render tabs
        self.render_tabs(frame, chunks[0]);

        match &self.current_page {
            Web3Page::AddressValidation => {
                let para = paragraph_widget(
                    "Hi! We are adding more interactive features to Stomata Web3",
                    "About",
                );
                frame.render_widget(para, chunks[1]);
            }
            Web3Page::Portfolio => {
                // rendering from ui_state
                let portfolio = self.ui_state.portfolio.as_ref();
                if let Some(portfolio) = portfolio {
                    let input_widget = self
                        .ui_state
                        .input_area_state
                        .get_or_insert_with(|| InputWidgetState::new());
                    portfolio.display(frame, chunks[1], Some(input_widget));
                }
            }
        }
    }

    /// Renders the tab bar at the top of the interface
    ///
    /// # Arguments
    ///
    /// * `frame` - The ratatui frame to render into
    /// * `area` - The rectangular area to render the tabs in
    pub fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = Web3Page::titles().iter().map(|t| Line::from(*t)).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Stomata"))
            .select(self.tab_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(tabs, area);
    }

    /// Processes keyboard events from the user
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to process
    ///
    /// # Errors
    ///
    /// Returns an error if event processing fails.
    pub async fn handle_events(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if key.kind == KeyEventKind::Press {
            let mut handled = false;

            match self.current_page {
                Web3Page::Portfolio => {
                    match &mut self.ui_state.input_area_state {
                        Some(input_widget_state) => {
                            handled = input_widget_state.handle_input_events(key);
                        }
                        None => {
                            // initialize the portfolio struct
                        }
                    }
                }
                _ => {}
            }

            if !handled {
                self.process_global_events(key).await;
            }
        }
        Ok(())
    }

    /// Processes global keyboard shortcuts
    ///
    /// Handles navigation keys (Tab, arrows), quit command (q),
    /// and direct tab selection (number keys).
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to process
    async fn process_global_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.render = false;
            }
            KeyCode::Tab => {
                self.next_tab();
            }
            KeyCode::BackTab => {
                self.previous_tab();
            }
            KeyCode::Char('1') => {
                self.tab_index = 0;
                self.current_page = Web3Page::AddressValidation;
            }
            KeyCode::Char('2') => {
                self.tab_index = 1;
                // fetch the pre-requisit data

                // TODO: This becomes a UI freeze logic while the async code runs on main thread. Might have to use tokio::spawn
                // let provider = EVMProvider::new(
                //     "0xdadB0d80178819F2319190D340ce9A924f783711".to_string(),
                //     "https://rpc.fullsend.to".to_string(),
                // );
                // let portfolio = get_portfolio(provider).await.unwrap();
                let portfolio = Portfolio::default();
                self.ui_state.portfolio = Some(portfolio);
                self.current_page = Web3Page::Portfolio;
            }
            _ => {}
        }
    }
}

/// Runs the Web3 feature in either interactive TUI or CLI mode
///
/// If a terminal is provided, runs in interactive mode with a tabbed UI.
/// Otherwise, parses CLI arguments and executes the requested command directly.
///
/// # Arguments
///
/// * `cli` - The parsed CLI arguments
/// * `terminal` - Optional terminal for interactive mode. If `None`, runs in CLI mode.
///
/// # Returns
///
/// * `Ok(true)` - Interactive mode exited normally
/// * `Ok(false)` - CLI command executed successfully
///
/// # Errors
///
/// Returns an error if:
/// - Terminal event polling fails
/// - Terminal rendering fails
/// - CLI command execution fails
///
/// # Interactive Mode Keybindings
///
/// - `q` - Quit the application
/// - `Tab` or `Right Arrow` - Next tab
/// - `Left Arrow` - Previous tab
/// - `1` - Jump to Address Validation tab
///
/// # Examples
///
/// ```rust,no_run
/// // CLI mode (no terminal provided)
/// let cli = Cli::parse();
/// run(&cli, None)?;
///
/// // Interactive mode (terminal provided)
/// let mut terminal = setup_terminal()?;
/// run(&cli, Some(&mut terminal))?;
/// ```
pub async fn run(
    cli: &Cli,
    terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>,
) -> anyhow::Result<bool> {
    let mut web3_state = Web3State::new();

    match terminal {
        Some(terminal) => {
            // get the refresh interval from the cli arg. Default 1000 ms
            let refresh_interval = Duration::from_millis(cli.interval);
            let mut last_tick = Instant::now();

            /// interactive mode
            while web3_state.render {
                let timeout = refresh_interval
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::from_secs(0));

                // poll for inputs only until timeout
                if event::poll(timeout)? {
                    if let Event::Key(key) = event::read()? {
                        // handle events
                        web3_state.handle_events(key).await?;
                        // redraw immediately after an event
                        terminal.draw(|frame| {
                            web3_state.render(frame);
                        })?;
                    }
                }

                if last_tick.elapsed() >= refresh_interval {
                    // draw
                    terminal.draw(|frame| {
                        web3_state.render(frame);
                    })?;
                    last_tick = Instant::now();
                }
            }
            Ok(web3_state.render)
        }
        None => {
            let web3_cli =
                Web3Cli::try_parse_from(once("web3".to_string()).chain(cli.args.iter().cloned()));
            match web3_cli {
                Ok(cli) => {
                    match cli.tool {
                        Web3Tool::AddressValidator { address } => validate_address(&address),
                        Web3Tool::Key(key_cmd) => match key_cmd {
                            KeySubCommands::Encrypt { name } => encrypt_key(name),
                            KeySubCommands::Decrypt { name, format } => decrypt_key(name, format),
                            KeySubCommands::List {} => list_all_keys(),
                            KeySubCommands::Delete { name } => delete_encrypted_key(name),
                        },
                    };
                }
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            };
            Ok(false)
        }
    }
}
