use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Block,
};
use ratatui::{prelude::*, widgets::*};

use crate::state::AppState;

pub fn render_lost_data_p3(area: Rect, buf: &mut Buffer, _state: &mut AppState) {
    Block::new()
        .title("  Lost Data  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);
}

    // Paragraph::new(state.unused_data_path.as_str())
    //     .block(mapping_prompt)
    //     .render(lost_data_path, buf);
    // todo: Need to add this option again at the end
