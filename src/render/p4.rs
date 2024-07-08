use ratatui::{buffer::Buffer, layout::Rect, prelude::*, widgets::*};

use crate::state::AppState;

pub fn render_end_p4(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new().style(Style::default().fg(Color::White)).render(area, buf);

    let vertical_margin = if area.height >= 3 { (area.height - 3) / 2 } else { 0 };

    let txt = format!(
        "The mapping has been completed.
    The output file has been saved to: {}
    The custom mapping file has been saved to: {}
    ",
        state.output_path, state.custom_mapping_path
    );

    Paragraph::new(txt)
        .centered()
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .render(
            area.inner(&Margin {
                vertical: vertical_margin,
                horizontal: 4,
            }),
            buf,
        );
}
