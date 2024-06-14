pub mod p1;
pub mod p2;
pub mod p3;

use p1::render_description_input_p1;
use p2::render_manual_mapping_p2;
use p3::render_lost_data_p3;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::Block,
};
use ratatui::prelude::*;

use crate::state::AppState;

pub fn render_page(frame: &mut Frame, area: Rect, state: &mut AppState) {
    // let state.frame = frame;
    let vertical_sections = Layout::vertical(vec![Constraint::Min(0), Constraint::Length(1)]);
    let [top, bottom_area] = vertical_sections.areas(area);

    match state.tab {
        crate::state::Tabs::InputPromptsP1 => render_description_input_p1(top, frame.buffer_mut(), state),
        crate::state::Tabs::ManualMappingP2 => render_manual_mapping_p2(top, frame.buffer_mut(), state),
        crate::state::Tabs::UnusedDataP3 => render_lost_data_p3(top, frame.buffer_mut(), state),
    }
    render_bottom_bar(bottom_area, frame.buffer_mut());
}


fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
    let vertical_sections = Layout::horizontal(vec![Constraint::Length(23), Constraint::Min(0)]);
    let [left, right] = vertical_sections.areas(area);

    Block::new()
        .title("  Impierce Technologies ")
        .title_alignment(Alignment::Left)
        .title_style(style::Color::Blue)
        .bg(Color::Black)
        .render(left, buf);

    let keys = [
        ("←↓↑→", "Navigate"),
        ("Tab", "Next Page"),
        ("F2", "Prev Page"),
        ("Enter", "Save"),
        ("Esc", "Quit"),
    ];
    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(
                format!(" {key} "),
                Style::default().on_black().add_modifier(Modifier::BOLD),
            );
            let desc = Span::styled(format!(" {desc} "), Style::default().on_dark_gray());
            [key, desc]
        })
        .collect();
    Line::from(spans)
        .centered()
        .style((Color::Black, Color::Black))
        .render(right, buf);
}
