use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use stomata_core::collectors::system::metrics::SystemCollector;

use crate::{
    renders::{
        core_displays::traits::Display,
        render_widgets::{render_gauge::render_gauge, render_paragraph::paragraph_widget},
    },
    structs::UIState,
    utils::bytes_to_mb,
};

impl Display for SystemCollector {
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        _ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()> {
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
                bytes_to_mb(self.system_metrics.memory_used),
                bytes_to_mb(self.system_metrics.memory_total),
                "Memory Usage",
                "MB",
            ),
            layout[0],
        );

        // render swap usage gauge
        frame.render_widget(
            render_gauge(
                bytes_to_mb(self.system_metrics.swap_used),
                bytes_to_mb(self.system_metrics.swap_total),
                "Swap Usage",
                "MB",
            ),
            layout[1],
        );

        // render cpu usage gauge
        frame.render_widget(
            render_gauge(
                self.system_metrics.cpu_usage as f64,
                100.0,
                "CPU Usage",
                "%",
            ),
            layout[2],
        );

        // --- PARAGRAPH ---
        let memory_used = self.system_metrics.memory_used as f64
            / self.system_metrics.memory_total as f64
            * 100.0;

        let text = format!(
            "Memory Used: {:.2} Bytes\nTotal Memory: {:.2} Bytes\nUsage: {:.2}%",
            self.system_metrics.memory_used, self.system_metrics.memory_total, memory_used,
        );

        let swap_used =
            self.system_metrics.swap_used as f64 / self.system_metrics.swap_total as f64 * 100.0;
        let text_swap = format!(
            "Swap Used: {:.2} Bytes\nTotal Swap: {:.2} Bytes\nUsage: {:.2}%",
            self.system_metrics.swap_used, self.system_metrics.swap_total, swap_used,
        );

        let processes_count_text = format!("CPU count: {}", self.system_metrics.cpu_count);
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
}
