use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};
use stomata_core::collectors::structs::{Metrics, MetricsToFetch, StomataSystemMetrics};

use crate::{
    renders::core_displays::traits::{Display, SingleProcessDisplay},
    structs::{Page, SingleProcessUI, UIState},
    utils::bytes_to_mb,
};

#[derive(Debug)]
pub struct App {
    pub render: bool,
    pub metrics: StomataSystemMetrics,
    pub tab_index: usize,
    pub current_page: Page,
    pub store_data: bool,
    pub ui_state: UIState,
}

impl App {
    pub fn new(store_metrics: bool) -> Self {
        Self {
            render: true,
            metrics: StomataSystemMetrics::new(),
            tab_index: 0,
            current_page: Page::System,
            store_data: store_metrics, // by default don't store history data
            ui_state: UIState::default(),
        }
    }

    // go to the next tab
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % Page::titles().len();
        self.current_page = Page::from_index(self.tab_index);
    }

    // go to the previous tab
    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = Page::titles().len() - 1;
        }
        self.current_page = Page::from_index(self.tab_index);
    }

    // render according to the tab selected
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

        // render tabs
        self.render_tabs(frame, chunks[0]);

        match &self.current_page {
            Page::Metrics => {
                if let Metrics::SystemResource(system_collector) =
                    self.metrics.fetch(MetricsToFetch::SystemResource)
                {
                    let _ = system_collector.display(frame, chunks[1], None);
                };
            }
            Page::System => {
                if let Metrics::SystemInfo(system_info) =
                    self.metrics.fetch(MetricsToFetch::SystemInfo)
                {
                    let _ = system_info.display(frame, chunks[1], None);
                };
            }
            Page::Processes => {
                if let Metrics::Processes(processes) = self.metrics.fetch(MetricsToFetch::Process) {
                    self.ui_state.process_table.process_count = processes.len();
                    let _ = processes.display(frame, chunks[1], Some(&mut self.ui_state));
                }
            }
            Page::SingleProcess(pid) => {
                let total_memory = bytes_to_mb(self.metrics.system.total_memory());
                if let Metrics::SingleProcessPid(Some(process)) =
                    self.metrics.fetch(MetricsToFetch::SingleProcessPid(*pid))
                {
                    self.ui_state
                        .single_process_disk_usage
                        .update_disk_history(process.basic_process_data.pid, &process.disk_usage);

                    let _ = SingleProcessUI { data: process }.display_process_metrics(
                        frame,
                        chunks[1],
                        total_memory,
                        &mut self.ui_state,
                    );
                }
            }
            Page::Network => {
                if let Metrics::Networks(network_metrics) =
                    self.metrics.fetch(MetricsToFetch::Networks)
                {
                    let _ = network_metrics.display(frame, chunks[1], Some(&mut self.ui_state));
                }
            }
        }
    }

    // render tabs
    pub fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = Page::titles().iter().map(|t| Line::from(*t)).collect();
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

    // handle quit events to close the new terminal
    pub fn handle_events(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if key.kind == KeyEventKind::Press {
            self.process_global_events(key);
            match self.current_page {
                Page::Processes => {
                    self.process_page_events(key);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn process_global_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.render = false;
            }
            KeyCode::Right | KeyCode::Tab => {
                self.next_tab();
            }
            KeyCode::Left => {
                self.previous_tab();
            }
            KeyCode::Char('1') => {
                self.tab_index = 0;
                self.current_page = Page::System;
            }
            KeyCode::Char('2') => {
                self.tab_index = 1;
                self.current_page = Page::Metrics;
            }
            KeyCode::Char('3') => {
                self.tab_index = 2;
                self.current_page = Page::Processes;
            }
            KeyCode::Char('4') => {
                self.tab_index = 3;
                self.current_page = Page::Network;
            }
            _ => {}
        }
    }

    fn process_page_events(&mut self, key: KeyEvent) {
        let max_processes = self.ui_state.process_table.process_count;
        match key.code {
            KeyCode::Down => {
                if let Some(selected_row) = self.ui_state.process_table.process_list.selected() {
                    let next_row = (selected_row + 1).min(max_processes.saturating_sub(1));
                    self.ui_state
                        .process_table
                        .process_list
                        .select(Some(next_row));
                }
            }
            KeyCode::Up => {
                if let Some(selected_row) = self.ui_state.process_table.process_list.selected() {
                    let next_row = selected_row.saturating_sub(1);
                    self.ui_state
                        .process_table
                        .process_list
                        .select(Some(next_row));
                }
            }
            KeyCode::Enter => {
                if let Some(selected_process_pid) = self.ui_state.process_table.selected_pid {
                    self.current_page = Page::SingleProcess(selected_process_pid);
                }
            }
            _ => {}
        }
    }
}
