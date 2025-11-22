use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};
use stomata_core::collectors::structs::{
    MetricsCategory, MetricsHistory, SystemCollector, SystemInfo, SystemMetrics,
};

use crate::{
    renders::{
        displays::{
            display_network::display_network_stats, display_single_process::SingleProcessDisplay,
        },
        render_widgets::{
            render_gauge::render_gauge, render_paragraph::paragraph_widget,
            render_table::render_table,
        },
    },
    structs::{Page, SingleProcessUI, UIState},
    utils::bytes_to_mb,
};

#[derive(Debug)]
pub struct App {
    pub render: bool,
    // pub metrics_history: MetricsStorage,
    pub system_info: SystemInfo,
    pub metrics_collector: SystemCollector,
    pub tab_index: usize,
    pub current_page: Page,
    pub store_data: bool,
    pub ui_state: UIState,
}

impl App {
    pub fn new(store_metrics: bool) -> Self {
        let collector = SystemCollector::new(store_metrics);
        let system_info = collector.system_info();
        Self {
            render: true,
            system_info,
            metrics_collector: collector,
            tab_index: 0,
            current_page: Page::System,
            store_data: store_metrics, // by default don't store history data
            ui_state: UIState::default(),
        }
    }

    pub fn update_metrics(&mut self, refresh_category: MetricsCategory) {
        if let Err(e) = self.metrics_collector.collect(refresh_category) {
            eprintln!("Error collecting metrics: {:?}", e);
        };
    }

    pub fn get_latest_metric(&self) -> Option<&SystemMetrics> {
        match &self.metrics_collector.system_metrics {
            MetricsHistory::History(history) => history.back(),
            MetricsHistory::Single(metric) => Some(metric),
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
                let _ = self.draw_chart(frame, chunks[1]);
            }
            Page::System => {
                let _ = self.display_system_info(frame, chunks[1]);
            }
            Page::Processes => {
                let _ = self.display_processes(frame, chunks[1]);
            }
            Page::SingleProcess(pd) => {
                let latest_metrics = self.get_latest_metric().cloned();
                if let Some(process) = self.metrics_collector.get_process_for_pid(pd.pid) {
                    self.ui_state
                        .single_process_disk_usage
                        .update_disk_history(process.basic_process_data.pid, &process.disk_usage);
                    let _ = SingleProcessUI { data: process }.display_process_metrics(
                        frame,
                        chunks[1],
                        latest_metrics,
                        &mut self.ui_state,
                    );
                }
            }
            Page::Network => {
                let _ = display_network_stats(frame, chunks[1]);
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

    fn display_system_info(&self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let mut system_info_str = format!(
            "\n\n\n\n\nOS name: {}\nOS version: {}\nKernel Version: {}\nHostname: {}",
            self.system_info.os_name,
            self.system_info.os_version,
            self.system_info.kernel_version,
            self.system_info.hostname
        );

        let helper_instructions = "\n\n\n\n\n\n\nSwitch Tabs: Use number keys OR Tab btn OR <-, -> arrow keys\nMove selector: Up. Down arrow keys\nSelect: Enter key";
        system_info_str.push_str(helper_instructions);
        let paragraph = paragraph_widget(&system_info_str, "System Info");
        frame.render_widget(
            paragraph.alignment(ratatui::layout::Alignment::Center),
            area,
        );
        Ok(())
    }

    fn draw_chart(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        self.update_metrics(MetricsCategory::Basic);

        let latest_metric = match self.get_latest_metric() {
            Some(metric) => metric,
            None => {
                eprintln!("No metrics available yet.");
                &SystemMetrics::default()
            }
        };

        let layout = Layout::vertical([
            Constraint::Percentage(23),
            Constraint::Percentage(23),
            Constraint::Percentage(24),
            Constraint::Percentage(30),
        ])
        .split(area);

        // render memory usage gauge
        frame.render_widget(
            render_gauge(
                bytes_to_mb(latest_metric.memory_used),
                bytes_to_mb(latest_metric.memory_total),
                "Memory Usage",
                "MB",
            ),
            layout[0],
        );

        // render swap usage gauge
        frame.render_widget(
            render_gauge(
                bytes_to_mb(latest_metric.swap_used),
                bytes_to_mb(latest_metric.swap_total),
                "Swap Usage",
                "MB",
            ),
            layout[1],
        );

        // render cpu usage gauge
        frame.render_widget(
            render_gauge(latest_metric.cpu_usage as f64, 100.0, "CPU Usage", "%"),
            layout[2],
        );

        // --- PARAGRAPH ---
        let memory_used =
            latest_metric.memory_used as f64 / latest_metric.memory_total as f64 * 100.0;

        let text = format!(
            "Memory Used: {:.2} Bytes\nTotal Memory: {:.2} Bytes\nUsage: {:.2}%",
            latest_metric.memory_used, latest_metric.memory_total, memory_used,
        );

        let swap_used = latest_metric.swap_used as f64 / latest_metric.swap_total as f64 * 100.0;
        let text_swap = format!(
            "Swap Used: {:.2} Bytes\nTotal Swap: {:.2} Bytes\nUsage: {:.2}%",
            latest_metric.swap_used, latest_metric.swap_total, swap_used,
        );

        let processes_count_text = format!(
            "CPU count: {}\nCurrent Processes count {}",
            latest_metric.cpu_count, latest_metric.processes_count
        );
        let process_paragraph = paragraph_widget(&processes_count_text, "Processes Count");

        let paragraph = paragraph_widget(&text, "Memory Info");
        let swap_paragraph = paragraph_widget(&text_swap, "Swap Info");

        let layout_paragraph = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(layout[3]);

        frame.render_widget(paragraph, layout_paragraph[0]);
        frame.render_widget(swap_paragraph, layout_paragraph[1]);
        frame.render_widget(process_paragraph, layout_paragraph[2]);

        Ok(())
    }

    // display the current running processes
    fn display_processes(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
        self.update_metrics(MetricsCategory::ProcessesWithoutTasks); // update processes only

        let processes = match self.get_latest_metric() {
            Some(metrics) => metrics.processes.clone(),
            None => Vec::new(),
        };
        let headers = vec!["PID", "Name", "CPU", "Memory", "Status"];

        let table_widget = render_table(headers, &processes, "Processes");
        frame.render_stateful_widget(table_widget, area, &mut self.ui_state.process_list);
        Ok(())
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
        match key.code {
            KeyCode::Down => {
                let max_processes = match self.get_latest_metric() {
                    Some(system_metrics) => system_metrics.processes_count,
                    None => 10 as usize,
                };
                if let Some(selected_row) = self.ui_state.process_list.selected() {
                    let next_row = (selected_row + 1).min(max_processes.saturating_sub(1));
                    self.ui_state.process_list.select(Some(next_row));
                }
            }
            KeyCode::Up => {
                if let Some(selected_row) = self.ui_state.process_list.selected() {
                    let next_row = selected_row.saturating_sub(1);
                    self.ui_state.process_list.select(Some(next_row));
                }
            }
            KeyCode::Enter => {
                if let Some(selected_process) = self.ui_state.process_list.selected() {
                    if let Some(selected_process_data) = self.get_latest_metric() {
                        let process_data = &selected_process_data.processes[selected_process];
                        // switch to a new page with path process/pid to show process_data
                        self.current_page = Page::SingleProcess(process_data.clone());
                    }
                }
            }
            _ => {}
        }
    }
}
