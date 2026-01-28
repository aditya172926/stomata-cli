//! Feature module dispatcher
//!
//! This module serves as the central router for all application features.
//! Each feature can be conditionally compiled using cargo feature flags
//! and can run in either interactive TUI mode or direct CLI mode.
//!
//! # Feature Flags
//!
//! - `core` - Enables core functionality for observability features
//! - `web3` - Enables Web3 development tools
//!
//! # Architecture
//!
//! Features are organized as submodules, each providing:
//! - A `run()` function that accepts CLI args and an optional terminal
//! - Support for both interactive (TUI) and non-interactive (CLI) modes
//! - Independent functionality that can be enabled/disabled at compile time

use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::structs::{Cli, Feature};

/// Core feature functionality
///
/// Only available when compiled with the `core` feature flag.
#[cfg(feature = "core")]
pub mod core;

/// Web3 development tools
///
/// Only available when compiled with the `web3` feature flag.
/// Provides utilities for blockchain development including address
/// validation and secure key management.
#[cfg(feature = "web3")]
pub mod web3;

/// Executes the requested feature in either interactive (TUI) or CLI mode
///
/// This is the main entry point for feature execution. It routes to the
/// appropriate feature module based on the selected feature and handles
/// both terminal-based interactive mode and direct command-line execution.
///
/// # Arguments
///
/// * `feature` - The feature to execute (from the CLI or main menu)
/// * `cli` - Parsed command-line arguments containing feature-specific options
/// * `terminal` - Optional terminal for interactive TUI mode:
///   - `Some(terminal)` - Run in interactive mode with TUI
///   - `None` - Run in CLI mode, execute command and exit
///
/// # Returns
///
/// * `Ok(true)` - Interactive mode exited normally (user quit)
/// * `Ok(false)` - CLI mode completed successfully
///
/// # Errors
///
/// Returns an error if:
/// - The feature execution encounters an error
/// - Terminal rendering fails (interactive mode)
/// - Command parsing or execution fails (CLI mode)
///
/// # Examples
///
/// ```rust,no_run
/// use stomata::features::run_feature;
/// use stomata::structs::{Feature, Cli};
///
/// // Run in CLI mode (no terminal)
/// let cli = Cli::parse();
/// run_feature(Feature::Web3, &cli, None)?;
///
/// // Run in interactive mode (with terminal)
/// let mut terminal = setup_terminal()?;
/// run_feature(Feature::Web3, &cli, Some(&mut terminal))?;
/// ```
///
/// # Feature Availability
///
/// Only features compiled with their corresponding feature flags will be
/// available. Attempting to run a disabled feature will result in a
/// compile-time error.
pub async fn run_feature(
    feature: Feature,
    cli: &Cli,
    terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>,
) -> anyhow::Result<bool> {
    match feature {
        #[cfg(feature = "core")]
        Feature::Core => core::core_feature::run(cli, terminal),
        #[cfg(feature = "web3")]
        Feature::Web3 => web3::web3_feature::run(cli, terminal).await,
    }
}
