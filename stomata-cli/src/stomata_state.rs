//! Application state management and feature selection UI
//!
//! Manages the main application state including feature availability,
//! user selection, and navigation between the feature selection menu
//! and running features. Provides the interactive menu interface for
//! choosing which monitoring or utility feature to run.

use std::collections::HashMap;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

#[cfg(feature = "web3")]
use crate::structs::Feature;
use crate::{
    renders::render_widgets::render_paragraph::paragraph_widget,
    structs::{AppState, StomataState},
};

impl StomataState {
    /// Creates a new application state with available features.
    ///
    /// Initializes the application state and builds a map of enabled features
    /// based on compile-time feature flags. Only features that are compiled
    /// into the binary will appear in the feature selection menu.
    ///
    /// # Available Features
    ///
    /// - **core** (requires `feature = "core"`): System monitoring capabilities
    ///   including CPU, memory, disk, and network metrics
    /// - **web3** (requires `feature = "web3"`): Web3 utilities for address
    ///   validation, key management, and blockchain interactions
    ///
    /// # Returns
    ///
    /// A new `StomataState` initialized to the feature selection screen with
    /// no feature selected.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::structs::StomataState;
    ///
    /// let app = StomataState::new();
    /// // app.available_features contains only compiled features
    /// // app.state is AppState::FeatureSelection
    /// // app.selected_feature is 0
    /// ```
    pub fn new() -> Self {
        let mut available_features = HashMap::new();

        #[cfg(feature = "core")]
        available_features.insert("core".to_string(), Feature::Core);

        #[cfg(feature = "web3")]
        available_features.insert("web3".to_string(), Feature::Web3);

        Self {
            state: AppState::FeatureSelection,
            selected_feature: 0,
            available_features,
        }
    }

    /// Renders the feature selection menu to the terminal UI.
    ///
    /// Displays a two-section layout with a title area and a selectable list
    /// of available features. Each feature shows a name and description to
    /// help users choose the appropriate tool for their needs.
    ///
    /// # Arguments
    ///
    /// * `frame` - Mutable reference to the ratatui frame for rendering
    ///
    ///
    /// # Error Handling
    ///
    /// If no features are compiled, displays an error message instructing
    /// the user to rebuild with feature flags enabled.
    ///
    pub fn render_feature_selection(&self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(5), Constraint::Min(10)]).split(frame.area());

        let title = paragraph_widget("Select a feature", "Stomata ClI");
        frame.render_widget(title, chunks[0]);

        // feature list
        if self.available_features.is_empty() {
            let msg = paragraph_widget(
                "No features available. Please install with a feature flag",
                "Error",
            );
            frame.render_widget(msg, chunks[1]);
        } else {
            let items: Vec<ListItem> = self
                .available_features
                .iter()
                .map(|(_key, feature)| {
                    let (name, desc) = match feature {
                        Feature::Core => (
                            "System Monitor",
                            "Monitor CPU, Memory and Swap, Disk and Network metrics",
                        ),
                        Feature::Web3 => (
                            "Web3 Tools",
                            "Access web3 tools for address validation and more...",
                        ),
                    };

                    ListItem::new(vec![
                        Line::from(Span::styled(
                            name,
                            Style::default().add_modifier(Modifier::BOLD),
                        )),
                        Line::from(Span::styled(desc, Style::default().fg(Color::Gray))),
                    ])
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Available Features"),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("► ");

            let mut state = ListState::default();
            state.select(Some(self.selected_feature));
            frame.render_stateful_widget(list, chunks[1], &mut state);
        }
    }

    /// Handles keyboard input for feature selection navigation.
    ///
    /// Processes key events to navigate the feature list, select features,
    /// or quit the application. Updates the application state based on
    /// user input.
    pub fn handle_feature_selection(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => {
                if self.selected_feature > 0 {
                    self.selected_feature -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_feature < self.available_features.len().saturating_sub(1) {
                    self.selected_feature += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(&feature) = self.available_features.values().nth(self.selected_feature)
                {
                    log!("Selected feature {:?}", feature);
                    self.state = AppState::RunningFeature(feature);
                }
            }
            KeyCode::Char('q') => {
                return false; // Exit app
            }
            _ => {}
        }
        true // Continue running
    }
}
