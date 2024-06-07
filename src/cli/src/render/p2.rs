use std::path::Path;

use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};
use style::palette::material::BLACK;
use symbols::border;

use crate::{transformations, utils::AppState};

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
    state.selector_area_p2 = left_selector;

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
    } else {
        Paragraph::new("Invalid input path")
            .block(Block::default())
            .fg(Color::Red)
            .render(left_selector, buf);
    }

    Block::new()
        .title("Missing Field")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .render(right_missing_fields, buf);
    Paragraph::new("\n".to_owned() + state.missing_data_field.as_ref().unwrap().as_str())
        .wrap(Wrap { trim: false })
        .render(
            right_missing_fields.inner(&Margin {
                horizontal: 0,
                vertical: 1,
            }),
            buf,
        );

    if state.popup_mapper_p2 == true {
        render_popup_mapping(
            area.inner(&Margin {
                horizontal: 15,
                vertical: 4,
            }),
            buf,
            state,
        );
    }
}

pub fn render_popup_mapping(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Clear.render(area, buf);
    Block::new().style(Style::default().bg(Color::Black)).render(area, buf);

    let horizontal_sections = Layout::horizontal(vec![
        Constraint::Percentage(33),
        Constraint::Min(0),
        Constraint::Percentage(33),
    ]);
    let vertical_sections = Layout::vertical(vec![Constraint::Percentage(100), Constraint::Min(1)]);
    let [top, bottom] = vertical_sections.areas(area);
    let [left, middle, right] = horizontal_sections.areas(top);

    Block::default()
        .borders(Borders::RIGHT)
        .border_set(border::Set {
            top_left: ".",
            top_right: ".",
            bottom_left: ".",
            bottom_right: ".",
            vertical_left: ".",
            vertical_right: "|",
            horizontal_top: ".",
            horizontal_bottom: ".",
        })
        .render(
            left.inner(&Margin {
                horizontal: 0,
                vertical: 1,
            }),
            buf,
        );
    Block::default()
        .borders(Borders::RIGHT)
        .border_type(BorderType::Plain)
        .render(
            middle.inner(&Margin {
                horizontal: 0,
                vertical: 1,
            }),
            buf,
        );

    state.popup_area_p2 = area;
    state.popup_value_area_p2 = middle;
    Paragraph::new(state.input_fields[state.selected_input_field].0.as_str())
        .wrap(Wrap { trim: false })
        .render(
            left.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );

    Paragraph::new(state.input_fields[state.selected_input_field].1.as_str())
        .wrap(Wrap { trim: false })
        .scroll((state.offset_value, 0))
        .render(
            middle.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );

    let vertical_sections_result = Layout::vertical(vec![Constraint::Min(2), Constraint::Percentage(100)]);
    let [result_path, result_value] = vertical_sections_result.areas(right.inner(&Margin {
        horizontal: 1,
        vertical: 1,
    }));

    state.popup_result_path_p2 = result_path;
    state.popup_result_value_p2 = result_value;
    Paragraph::new(state.missing_data_field.as_ref().unwrap().as_str())
        .wrap(Wrap { trim: false })
        .scroll((state.offset_result_path, 0))
        .render(result_path, buf);

    state.candidate_data_value = Some("asssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssffgg".to_string());
    Paragraph::new(state.candidate_data_value.as_ref().unwrap().as_str())
        .block(Block::default().borders(Borders::TOP).border_set(border::Set {
            top_left: ".",
            top_right: ".",
            bottom_left: ".",
            bottom_right: ".",
            vertical_left: ".",
            vertical_right: ".",
            horizontal_top: "-",
            horizontal_bottom: ".",
        }))
        .wrap(Wrap { trim: false })
        .scroll((state.offset_result_value, 0))
        .render(result_value, buf);

    let tabs = vec![
        "Copy",
        "LowerCase",
        "UpperCase",
        "Split",
        "Concat",
        "OneToMany",
        "ManyToOne",
        "Regex",
    ];
    let [transformations, selected] = Layout::horizontal(vec![
        Constraint::Min(tabs.concat().len() as u16 + 16),
        Constraint::Percentage(100),
    ])
    .areas(bottom);

    Tabs::new(vec![
        "Copy",
        "LowerCase",
        "UpperCase",
        "Split",
        "Concat",
        "OneToMany",
        "ManyToOne",
        "Regex",
    ])
    .style(Style::default().fg(Color::White).bg(Color::DarkGray))
    .highlight_style(Color::Yellow)
    .select(state.transformations as usize)
    .divider("")
    .render(transformations, buf);

    let selected_transformations: Vec<String> = state.selected_transformations.iter().map(|x| x.to_string()).collect();
    Tabs::new(selected_transformations)
    .style(Style::default().fg(Color::Black).bg(Color::Gray))
    .highlight_style(Style::default().fg(BLACK).add_modifier(Modifier::BOLD))
    .select(state.selected_transformation as usize)
    .divider("")
    .render(selected, buf);
}
