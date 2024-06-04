use std::path::Path;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};

use crate::utils::AppState;

pub fn render_lost_data_p3(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Lost Data  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);
}
