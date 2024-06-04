use std::path::Path;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};

use crate::utils::AppState;

pub fn render_manual_mapping_p2(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Manual Mapping  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    let horizontal_sections = Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0)]);
    let [_title, page] = horizontal_sections.areas(area);
    let vertical_sections = Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Min(0)]);
    let [mut left_selector, mut right_missing_fields] = vertical_sections.areas(page);

    Block::new().borders(Borders::RIGHT).render(left_selector, buf);

    left_selector = left_selector.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });
    right_missing_fields = right_missing_fields.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });


    let path = Path::new(&state.input_path);
    if path.exists() && path.is_file() {
        let mut table_state = TableState::default().with_selected(Some(state.selected_input_field));
        let rows: Vec<Row> = state
            .input_fields
            .iter()
            .map(|(key, value)| Row::new(vec![key.as_str(), value.as_str()]))
            .collect();

        StatefulWidget::render(
            Table::new(rows, [Constraint::Length(30), Constraint::Length(30)])
                .block(Block::new())
                .header(Row::new(vec!["Field", "Value"]).style(Style::new().add_modifier(Modifier::BOLD)))
                .highlight_style(Style::new().light_yellow()),
            left_selector,
            buf,
            &mut table_state,
        );

        // if state.map_input_field == true {
        //     let popup = Block::new()
        //         .title("  Selected Mapping  ")
        //         .title_alignment(Alignment::Center)
        //         .style(Style::default().bg(Color::Black));
        //     popup.render(area.inner(&Margin{ horizontal: 15, vertical: 4}), buf);
        // }
    } 
    else {
        Paragraph::new("Invalid input path")
            .block(Block::default())
            .fg(Color::Red)
            .render(left_selector, buf);
    }

    if state.map_input_field == true {
        let popup = Block::new()
            .title("  Selected Mapping  ")
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black));
        popup.render(area.inner(&Margin{ horizontal: 15, vertical: 4}), buf);
    }
    Block::new()
        .title("  Missing field here  xxx")
        .render(right_missing_fields, buf);
}

