use std::path::Path;

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

pub fn render_popup_unused_data(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Clear.render(area, buf);
    Block::new().style(Style::default().fg(Color::White).bg(Color::Black))
        .borders(Borders::ALL)
        .render(area, buf);

    let [txt, path, _margin] = Layout::vertical(vec![
        Constraint::Percentage(100),
        Constraint::Length(3),
        Constraint::Min(1),
    ]).areas(area);

    let vertical_margin;
    if area.height >= 3 {
        vertical_margin = (area.height - 4) / 2;
    }
    else {
        vertical_margin = 0;
    }
    Paragraph::new("\n There is still some unused data from the input file.\nIf you want to save this, please enter a file path, leave empty to discard: ")
        .centered()
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
        .render(txt.inner(&Margin {
            vertical: vertical_margin,
            horizontal: 1,
        }), buf);

    let unused_data_prompt = Block::new()
        .title("  Unused Data Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);    
    let unused_data_path = Path::new(&state.unused_data_path);
    if state.unused_data_path.is_empty() {
        Paragraph::new(state.unused_data_path.as_str())
            .block(unused_data_prompt)
            .fg(Color::White)
            .render(path, buf);
    }        
    else if !unused_data_path.is_file() {
        Paragraph::new(state.unused_data_path.as_str())
            .block(unused_data_prompt)
            .fg(Color::Green)
            .render(path, buf);
    } else {
        Paragraph::new(state.unused_data_path.as_str())
            .block(unused_data_prompt)
            .fg(Color::Rgb(240, 160, 100))
            .render(path, buf);
    }

}
