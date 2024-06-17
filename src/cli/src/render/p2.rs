use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};
use style::palette::material::BLACK;
use symbols::border;

use crate::state::{AppState, Multiplicity, P2Tabs};

pub fn render_manual_mapping_p2(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    // Main title at the top of p2
    Block::new()
        .title("  Manual Mapping  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    // Layout
    let [_title, page, bottom] =
        Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)]).areas(area);
    let [mut left_selector, mut right_missing_fields] =
        Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Min(0)]).areas(page);
    Block::new().borders(Borders::RIGHT).render(left_selector, buf);

    // Inner blocks for margins
    left_selector = left_selector.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });
    right_missing_fields = right_missing_fields.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });

    // Store areas in state
    state.selector_area_p2 = left_selector;
    state.missing_fields_area_p2 = right_missing_fields;

    // Highlight active area
    let mut inputfields_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let mut missingfields_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let mut mappingoptions_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    match state.p2_tabs {
        P2Tabs::InputFields => {
            inputfields_style = Style::default().fg(Color::Yellow);
        }
        P2Tabs::MissingFields => {
            missingfields_style = Style::default().fg(Color::Yellow);
        }
        P2Tabs::MappingOptions => {
            mappingoptions_style = Style::default().fg(Color::Yellow);
        }
    }

    // Render left selector list of input fields
    let mut table_state = TableState::default().with_selected(Some(state.selected_input_field));
    let rows: Vec<Row> = state
        .input_fields
        .iter()
        .map(|(key, value)| Row::new(vec![key.as_str(), value.as_str()]))
        .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::new())
            .header(Row::new(vec!["Field", "Value"]).style(Style::new().add_modifier(Modifier::BOLD)))
            .highlight_style(inputfields_style),
        left_selector,
        buf,
        &mut table_state,
    );

    // Render right tab containing missing fields
    state.missing_data_fields = Some(vec![
        "".to_string(),
        "field1".to_string(),
        "field1".to_string(),
        "field1".to_string(),
    ]); // todo: remove hard code testdata
    state.amount_missing_fields = state.missing_data_fields.as_ref().unwrap().len() - 1; // todo
    let mut table_state = TableState::default().with_selected(Some(state.selected_missing_field));
    let rows: Vec<Row> = state
        .missing_data_fields
        .as_ref()
        .unwrap() // todo: cleanup unwrap
        .iter()
        .map(|key| Row::new(vec![key.as_str(), ""]))
        .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::new())
            .header(Row::new(vec!["Missing Field", "Result Value"]).style(Style::new().add_modifier(Modifier::BOLD)))
            .highlight_style(missingfields_style),
        right_missing_fields,
        buf,
        &mut table_state,
    );

    // todo: render  output results

    // Render bottom mapping options bar
    if state.select_multiplicity {
        let multiplicities = vec![" OneToOne", "OneToMany", "ManyToOne"];
        let [multiplicity_tabs, selected, back, finish] = Layout::horizontal(vec![
            Constraint::Min(multiplicities.concat().len() as u16 + 10),
            Constraint::Percentage(100),
            Constraint::Length(6),
            Constraint::Length(9),
        ])
        .areas(bottom);

        Tabs::new(multiplicities)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(mappingoptions_style)
            .select(state.multiplicity as usize)
            .divider("")
            .render(multiplicity_tabs, buf);
        Block::new()
            .style(Style::default().fg(Color::Black).bg(Color::DarkGray))
            .render(selected, buf);

        render_mapping_bar_buttons(back, finish, state, buf);
    } else {
        match state.multiplicity {
            Multiplicity::OneToOne => render_onetoone(bottom, buf, state),
            Multiplicity::OneToMany => render_onetomany(bottom, buf, state),
            Multiplicity::ManyToOne => render_manytoone(bottom, buf, state),
        }
    }
}

pub fn render_onetoone(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let tabs = vec![" None", "LowerCase", "UpperCase", "Slice", "Regex"];

    let [transformations, selected, back, finish] = Layout::horizontal(vec![
        Constraint::Min(tabs.concat().len() as u16 + 10),
        Constraint::Percentage(100),
        Constraint::Length(6),
        Constraint::Length(9),
    ])
    .areas(area);

    // Highlight active area
    let mut active_style = Style::default().fg(Color::Yellow);
    if state.p2_tabs != P2Tabs::MappingOptions {
        active_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    }

    // Render options tab left and the selected options to the right
    if !state.selected_transformations_tab {
        Tabs::new(tabs)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(active_style)
            .select(state.transformations as usize)
            .divider("")
            .render(transformations, buf);
        let selected_transformations: Vec<String> =
            state.selected_transformations.iter().map(|x| x.to_string()).collect();
        Tabs::new(selected_transformations)
            .style(Style::default().fg(Color::Black).bg(Color::Gray))
            .highlight_style(Style::default().fg(BLACK))
            .select(state.selected_transformation as usize)
            .divider("")
            .render(selected, buf);
    } else {
        Tabs::new(tabs)
            .style(Style::default().fg(Color::Black).bg(Color::Gray))
            .highlight_style(Style::default().fg(BLACK))
            .select(state.transformations as usize)
            .divider("")
            .render(transformations, buf);
        let selected_transformations: Vec<String> =
            state.selected_transformations.iter().map(|x| x.to_string()).collect();
        Tabs::new(selected_transformations)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(active_style)
            .select(state.selected_transformation as usize)
            .divider("")
            .render(selected, buf);
    }

    render_mapping_bar_buttons(back, finish, state, buf);
}

fn render_onetomany(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let txt = "  Enter a divider, or select indices manually: ";
    let [txt_area, dividers, back, finish] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16),
        Constraint::Percentage(100),
        Constraint::Length(6),
        Constraint::Length(9),
    ])
    .areas(area);

    Paragraph::new(txt)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .render(txt_area, buf);

    // Display italic instructions to be overwritten by user input for dividers.
    if state.dividers.is_empty() {
        Paragraph::new(" Select the output fields in the right tab")
            .style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            )
            .render(dividers, buf);
    } else {
        Paragraph::new(" ".to_owned() + state.dividers.as_str())
            .style(Style::default().fg(Color::Black).bg(Color::Gray))
            .render(dividers, buf);
    }

    render_mapping_bar_buttons(back, finish, state, buf);
}

fn render_manytoone(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let txt = "  Select multiple fields in the left tab, the result is shown in the right tab. ";
    let [txt_area, back, finish] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16 + 2),
        Constraint::Length(6),
        Constraint::Length(9),
    ])
    .areas(area);

    Paragraph::new(txt)
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .render(txt_area, buf);

        render_mapping_bar_buttons(back, finish, state, buf);
}

fn render_mapping_bar_buttons (back: Rect, finish: Rect, state: &mut AppState, buf: &mut Buffer) {
    state.back_area_p2 = back;
    state.finish_button = finish;

    Paragraph::new(" Back ")
        .style(Style::default().fg(Color::Black).bg(Color::Red))
        .render(back, buf);
    Paragraph::new(" Confirm ")
        .style(Style::default().fg(Color::Black).bg(Color::Green))
        .render(finish, buf);
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
    let [top, _bottom] = vertical_sections.areas(area);
    let [left, middle, right] = horizontal_sections.areas(top);

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

    //state.candidate_data_value = Some("asssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssffgg".to_string());
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
}
