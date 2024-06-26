use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::*,
};

use crate::{
    state::AppState, trace_dbg,
};

pub fn render_end_p4(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new().style(Style::default().fg(Color::White))
        .render(area, buf);

    let vertical_margin;
    if area.height >= 3 {
        vertical_margin = (area.height - 3) / 2;
    }
    else {
        vertical_margin = 0;
    }

    let txt = format!("The mapping has been completed.
    The output file has been saved to: {}", state.output_path);
    let txt2 = format!("\nThe unused data has been saved to: {}", state.unused_data_path);
    // add custom mapping file path for saving.

    if !state.unused_data_path.is_empty() {
        Paragraph::new(txt + &txt2)
            .centered()
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .render(area.inner(&Margin {
                vertical: vertical_margin,
                horizontal: 4,
            }), buf);
    }
    else {
        Paragraph::new(txt)
            .centered()
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .render(area.inner(&Margin {
                vertical: vertical_margin,
                horizontal: 4,
            }), buf);
    }
}
