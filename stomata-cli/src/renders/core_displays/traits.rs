use crate::structs::UIState;
use ratatui::{Frame, layout::Rect};

pub trait Display {
    fn display(
        &self,
        frame: &mut Frame,
        area: Rect,
        ui_state: Option<&mut UIState>,
    ) -> anyhow::Result<()>;
}

pub trait SingleProcessDisplay {
    fn display_process_metrics(
        &self,
        frame: &mut Frame,
        area: Rect,
        total_memory: f64,
        ui_state: &mut UIState,
    ) -> anyhow::Result<()>;
}
