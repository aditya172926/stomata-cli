//! Data structures for application state and UI management
//!
//! Defines the primary types used throughout the application including
//! feature enums, application state, CLI arguments, page navigation,
//! UI state management, and ring buffers for time-series data storage.

use std::collections::{HashMap, VecDeque};

use clap::Parser;
use ratatui::{
    Frame,
    layout::Constraint,
    widgets::{Cell, TableState},
};
use stomata_core::collectors::{
    network::metrics::NetworkInterfaces, process::metrics::SingleProcessData,
};
use sysinfo::DiskUsage;

use crate::constants::{CLAMP_TREND_VALUE, MAX_HISTORY_IN_MEMORY, MAX_NETWORK_IN_MEMORY};

/// Available application features determined by compile-time flags.
///
/// Each variant corresponds to a major feature set that can be enabled
/// or disabled at build time using Cargo feature flags.
#[derive(Debug, Clone, Copy)]
pub enum Feature {
    /// Core system monitoring features (CPU, memory, disk, network, processes)
    #[cfg(feature = "core")]
    Core,

    /// Web3 utilities (address validation, key management)
    #[cfg(feature = "web3")]
    Web3,
}

/// Application state machine for managing UI flow.
///
/// Tracks whether the user is selecting a feature or currently running one.
/// Used to control the main event loop and determine what to render.
pub enum AppState {
    /// User is viewing the feature selection menu
    FeatureSelection,

    /// User is running a specific feature
    RunningFeature(Feature),
}

/// Top-level application state container.
///
/// Manages the current application state, feature selection, and the list
/// of available features based on compile-time configuration.
pub struct StomataState {
    /// Current state in the application flow
    pub state: AppState,

    /// Index of the currently selected feature in the menu
    pub selected_feature: usize,

    /// Map of available features (feature name -> Feature enum)
    pub available_features: HashMap<String, Feature>,
}

/// Command-line interface arguments.
///
/// Parsed using clap to configure application behavior and feature execution.
/// Supports both interactive TUI mode and direct CLI feature execution.
///
/// # Examples
///
/// ```bash
/// # Interactive mode
/// stomata --interactive
///
/// # CLI mode with specific feature
/// stomata web3 av --address 0x...
///
/// # Custom refresh interval
/// stomata -i --interval 500
/// ```
#[derive(Parser, Debug, Clone)]
#[command(name = "stomata")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable interactive TUI mode with feature selection menu
    #[arg(short = 'i', long, default_value_t = false)]
    pub interactive: bool,

    /// Refresh interval in milliseconds for system monitoring
    #[arg(short = 't', long, default_value_t = 1000)]
    pub interval: u64,

    /// Enable data storage/persistence (feature-dependent behavior)
    #[arg(short, long, default_value_t = false)]
    pub store: bool,

    /// Feature to run in CLI mode (ignored in interactive mode)
    pub feature: Option<String>,

    /// Arguments passed to the feature
    /// Allows arbitrary arguments including those starting with hyphens
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

/// Navigation pages in the system monitoring UI.
///
/// Represents different views available in the core monitoring feature.
/// Users can switch between pages using number keys or arrow keys.
#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    /// System information overview (OS, kernel, hostname)
    System,

    /// Real-time metrics gauges (CPU, memory, disk usage)
    Metrics,

    /// Sortable process list table
    Processes,

    /// Detailed view of a specific process with given PID
    SingleProcess(u32), // pid

    /// Network interface statistics and trends
    Network,
}

impl Page {
    /// Returns tab titles for the main navigation pages.
    ///
    /// Excludes `SingleProcess` as it's a sub-view, not a main tab.
    ///
    /// # Returns
    ///
    /// Vector of static strings: `["System", "Metrics", "Processes", "Network"]`
    pub fn titles() -> Vec<&'static str> {
        vec!["System", "Metrics", "Processes", "Network"]
    }

    /// Converts a tab index to its corresponding page.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based tab index
    ///
    /// # Returns
    ///
    /// The corresponding `Page` variant, defaults to `System` for invalid indices
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Page::System,
            1 => Page::Metrics,
            2 => Page::Processes,
            3 => Page::Network,
            _ => Page::System,
        }
    }
}

/// Trait for types that can be displayed as table rows.
///
/// Provides a consistent interface for converting data structures into
/// table cells with appropriate column widths for ratatui tables.
///
/// # Examples
///
/// ```ignore
/// impl TableRow for MyData {
///     fn to_cells(&self) -> Vec<Cell<'_>> {
///         vec![
///             Cell::from(self.id.to_string()),
///             Cell::from(self.name.clone()),
///         ]
///     }
///
///     fn column_widths() -> Vec<Constraint> {
///         vec![
///             Constraint::Length(10),
///             Constraint::Min(20),
///         ]
///     }
/// }
/// ```
pub trait TableRow {
    /// Converts the data into a vector of table cells.
    fn to_cells(&self) -> Vec<Cell<'_>>;

    /// Returns the column width constraints for the table.
    fn column_widths() -> Vec<Constraint>;
}

/// Comprehensive UI state management for all monitoring views.
///
/// Maintains state across different pages including table selections,
/// process data, disk I/O history, and network statistics.
#[derive(Debug)]
pub struct UIState {
    /// State for the process list table (selection, count)
    pub process_table: ProcessesUIState,

    /// Disk I/O history for the currently viewed process
    pub single_process_disk_usage: SingleProcessDiskUsage,

    /// Time-series data for all network interfaces
    pub networks_state: Option<HashMap<String, NetworkInterfaceData>>,
}

/// State management for the process list table.
///
/// Tracks table selection, total process count, and the PID of the
/// currently selected process for drilling down into details.
#[derive(Debug)]
pub struct ProcessesUIState {
    /// Ratatui table state for selection and scrolling
    pub process_list: TableState,

    /// Total number of processes in the list
    pub process_count: usize,

    /// PID of the selected process (if any)
    pub selected_pid: Option<u32>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            process_table: ProcessesUIState {
                process_list: TableState::default().with_selected(0),
                process_count: 0,
                selected_pid: None,
            },
            single_process_disk_usage: SingleProcessDiskUsage::default(),
            networks_state: None,
        }
    }
}

/// Wrapper for single process data display.
///
/// Used to pass process details to the detailed process view.
pub struct SingleProcessUI<'a> {
    /// Process data including metrics and metadata
    pub data: SingleProcessData<'a>,
}

/// Time-series storage for a single process's disk I/O activity.
///
/// Maintains historical read and write byte counts for visualizing
/// disk usage trends in sparkline charts.
#[derive(Debug)]
pub struct SingleProcessDiskUsage {
    /// PID of the process being tracked
    pub pid: u32,

    /// Historical disk read bytes (up to MAX_HISTORY_IN_MEMORY points)
    pub disk_read_usage: VecDeque<u64>,

    /// Historical disk write bytes (up to MAX_HISTORY_IN_MEMORY points)
    pub disk_write_usage: VecDeque<u64>,
}

impl Default for SingleProcessDiskUsage {
    fn default() -> Self {
        Self {
            pid: 0,
            disk_read_usage: VecDeque::<u64>::with_capacity(MAX_HISTORY_IN_MEMORY),
            disk_write_usage: VecDeque::<u64>::with_capacity(MAX_HISTORY_IN_MEMORY),
        }
    }
}

impl SingleProcessDiskUsage {
    /// Updates disk I/O history with new measurements.
    ///
    /// Maintains a sliding window of disk read/write data. When the tracked
    /// PID changes, clears the history to start fresh for the new process.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process ID of the current process
    /// * `disk_usage` - Current disk I/O statistics
    ///
    /// # Behavior
    ///
    /// - If PID changes: Clears all history and updates tracked PID
    /// - If history exceeds 60 points: Removes oldest entry (FIFO)
    /// - Appends new read/write byte counts to history
    pub fn update_disk_history(&mut self, pid: u32, disk_usage: &DiskUsage) {
        // reset the UI state data for disk write/read when changed at current displaying pid
        if pid != self.pid {
            self.disk_read_usage.clear();
            self.disk_write_usage.clear();
            self.pid = pid;
        }

        if self.disk_read_usage.len() > 60 {
            self.disk_read_usage.pop_front();
        }
        self.disk_read_usage.push_back(disk_usage.read_bytes);

        if self.disk_write_usage.len() > 60 {
            self.disk_write_usage.pop_front();
        }
        self.disk_write_usage.push_back(disk_usage.written_bytes);
    }
}

/// Time-series storage for a single network interface's statistics.
///
/// Maintains historical data for bytes, packets, and errors in both
/// transmit and receive directions using ring buffers for efficient
/// memory usage and visualization.
#[derive(Debug)]
pub struct NetworkInterfaceData {
    /// Bytes received over time
    pub received_bytes: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    /// Bytes transmitted over time
    pub transmitted_bytes: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    /// Packets received over time
    pub packets_received: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    /// Packets transmitted over time
    pub packets_transmitted: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    /// Receive errors over time
    pub errors_received: Ring<u64, MAX_NETWORK_IN_MEMORY>,
    /// Transmit errors over time
    pub errors_transmitted: Ring<u64, MAX_NETWORK_IN_MEMORY>,
}

impl Default for NetworkInterfaceData {
    fn default() -> Self {
        Self {
            received_bytes: Ring::new(),
            transmitted_bytes: Ring::new(),
            packets_received: Ring::new(),
            packets_transmitted: Ring::new(),
            errors_received: Ring::new(),
            errors_transmitted: Ring::new(),
        }
    }
}

impl NetworkInterfaceData {
    /// Updates all network statistics with new measurements.
    ///
    /// Pushes new values to all six ring buffers with clamping to prevent
    /// visualization distortion from transient spikes.
    ///
    /// # Arguments
    ///
    /// * `network_data` - Current network interface statistics
    ///
    /// # Notes
    ///
    /// Uses `push_clamped` to smooth out extreme spikes that could distort
    /// trend visualization while preserving genuine traffic patterns.
    pub fn update_network_history(&mut self, network_data: &NetworkInterfaces) {
        self.received_bytes
            .push_clamped(network_data.bytes_received);
        self.transmitted_bytes
            .push_clamped(network_data.bytes_transmitted);
        self.packets_received
            .push_clamped(network_data.packets_received);
        self.packets_transmitted
            .push_clamped(network_data.packets_transmitted);
        self.errors_received
            .push_clamped(network_data.errors_on_received);
        self.errors_transmitted
            .push_clamped(network_data.errors_on_transmitted);
    }
}

/// Fixed-size ring buffer for time-series data storage.
///
/// Efficiently stores a bounded history of measurements using a circular
/// buffer. When capacity is reached, oldest values are automatically
/// discarded (FIFO behavior).
///
/// # Type Parameters
///
/// * `T` - Element type
/// * `N` - Maximum capacity (compile-time constant)
///
/// # Examples
///
/// ```ignore
/// let mut ring: Ring<u64, 100> = Ring::new();
/// ring.push(42);
/// ring.push(100);
/// // After 100 pushes, oldest values automatically removed
/// ```
#[derive(Debug)]
pub struct Ring<T, const N: usize> {
    inner: VecDeque<T>,
}

impl<T, const N: usize> Ring<T, N> {
    /// Creates a new empty ring buffer with capacity `N`.
    pub fn new() -> Self {
        Self {
            inner: VecDeque::with_capacity(N),
        }
    }

    /// Pushes a value onto the ring buffer.
    ///
    /// If the buffer is at capacity, removes the oldest element before
    /// adding the new value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value to append
    pub fn push(&mut self, value: T) {
        if self.inner.len() == N {
            self.inner.pop_front();
        }
        self.inner.push_back(value);
    }

    /// Returns a mutable slice of the ring buffer's contents in contiguous memory.
    ///
    /// Rearranges elements if necessary to make them contiguous, enabling
    /// efficient iteration and visualization.
    pub fn make_contiguous(&mut self) -> &mut [T] {
        self.inner.make_contiguous()
    }
}

impl<T, const N: usize> Ring<T, N>
where
    T: Copy + Ord + From<u8>,
{
    /// Pushes a value with statistical clamping to reduce spike distortion.
    ///
    /// Instead of blindly accepting extreme values that could distort
    /// visualization, this method clamps incoming values to a percentile
    /// threshold based on historical data. This preserves genuine trends
    /// while smoothing transient spikes.
    ///
    /// # Arguments
    ///
    /// * `value` - Raw value to push
    ///
    /// # Algorithm
    ///
    /// 1. If buffer is empty, push value directly (no history to compare)
    /// 2. Collect all historical values plus the new value
    /// 3. Calculate the percentile threshold (defined by `CLAMP_TREND_VALUE`)
    /// 4. Find the percentile value using nth_element selection
    /// 5. Clamp the new value to the percentile if it exceeds it
    /// 6. Push the clamped value
    ///
    /// # Use Case
    ///
    /// Ideal for network throughput, disk I/O, or other metrics where brief
    /// spikes (e.g., from system updates, backups) shouldn't dominate the
    /// visual trend line.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut ring: Ring<u64, 100> = Ring::new();
    /// ring.push_clamped(100);  // Normal value
    /// ring.push_clamped(500);  // Spike, may be clamped
    /// ring.push_clamped(120);  // Normal value
    /// ```
    pub fn push_clamped(&mut self, value: T) {
        if self.inner.is_empty() {
            self.push(value);
            return;
        }

        // collect historical values
        let mut data: Vec<T> = self.inner.iter().copied().collect();
        data.push(value);

        // compute percentile index
        let p_index = ((data.len() - 1) as f64 * CLAMP_TREND_VALUE).round() as usize;

        // nth_element selection
        let (_, p_val, _) = data.select_nth_unstable(p_index);

        // clamp
        let clamped = if value > *p_val { *p_val } else { value };

        self.push(clamped);
    }
}

/////////////////////////
/// Input Widget Structs
/////////////////////////
#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Input Widget state
#[derive(Debug, Clone)]
pub struct InputWidgetState {
    /// Current value of input box
    pub input: String,
    /// Position of cursor in input box
    pub character_index: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// Recoded message history
    pub messages: String,
}
