use std::ops::Deref;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};

use crate::{
    backend::resolve::value_to_str,
    mapping_bars::{render_manytoone_bar, render_mapping_bar},
    popups::{render_popup_exit_warning, render_popup_mapping},
    render_breadcrumbs,
    state::{translate, AppState, MappingOptions, P2P3Tabs},
};

pub fn render_optional_data_p3(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title(format!("  {}  ", translate("optional_data")))
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    // Layout
    let [_title, breadcrumbs, page, bottom] = Layout::vertical(vec![
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);
    let [mut left_selector, mut right_optional_fields] =
        Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Min(0)]).areas(page);
    Block::new().borders(Borders::RIGHT).render(left_selector, buf);

    // Inner blocks for margins
    left_selector = left_selector.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });
    right_optional_fields = right_optional_fields.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });

    // Store areas in state
    state.selector_area_p2_p3 = left_selector;
    state.output_fields_area_p2_p3 = right_optional_fields;

    // Highlight active area
    let mut inputfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    let mut optionalfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    let mut mappingoptions_style = Style::default().fg(Color::White);
    match state.p2_p3_tabs {
        P2P3Tabs::InputFields => {
            inputfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2P3Tabs::OutputFields => {
            optionalfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2P3Tabs::MappingOptions => {
            mappingoptions_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        _ => {}
    }

    // Render left selector list of input fields
    let mut table_state = TableState::default().with_selected(Some(state.selected_input_field));
    let rows: Vec<Row> = state
        .input_fields
        .iter()
        .map(|(key, value)| {
            let mut row = Row::new(vec![key.as_str(), value.as_str()]);
            if state.completed_fields.iter().any(|(_, second)| second == key) {
                row = row.style(Style::default().fg(Color::Green));
            }
            row
        })
        .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::new())
            .header(Row::new([translate("field"), translate("value")]).style(Style::new().add_modifier(Modifier::BOLD)))
            .highlight_style(inputfields_style),
        left_selector,
        buf,
        &mut table_state,
    );

    // Render right tab containing optional fields

    ////
    let mut table_state = TableState::default().with_selected(Some(state.selected_output_field));

    state.output_display_subset = state
        .resolved_subsets
        .get(&state.optional_field_pointer)
        .and_then(|v| v.as_object())
        .expect("error: couldn't retrieve optional fields from Json Schema.")
        .iter()
        .map(|(key, value)| (key.to_string(), value_to_str(value)))
        .collect();

    state.output_display_subset.sort_by(|a, b| a.0.cmp(&b.0));
    if let Some(i) = state
        .output_display_subset
        .iter()
        .position(|(key, _)| key == "Your input >>")
    {
        let your_input_field = state.output_display_subset.remove(i);
        state.output_display_subset.insert(0, your_input_field);
    }
    state.output_display_subset.insert(0, ("".to_string(), "".to_string()));
    state.amount_output_fields = state.output_display_subset.len() - 2;

    let rows: Vec<Row> = state
        .output_display_subset
        .iter()
        .map(|(key, value)| {
            let mut row = Row::new(vec![key.deref(), value.deref()]);
            if state.completed_fields.iter().any(|(first, _)| {
                *first == state.optional_field_pointer
                    || *first == state.optional_field_pointer.as_str().to_owned() + "/" + key.as_str()
            })
            // this checks wether to pointer (equal to the breadcrumb in the CLI) has already been entered as completed, or that any in the fields under the current pointer have been completed
            {
                row = row.style(Style::default().fg(Color::Green));
            }
            row
        })
        .collect();

    ////

    // let mut table_state = TableState::default().with_selected(Some(state.selected_optional_field));
    // let rows: Vec<Row> = state
    //     .optional_fields
    //     .iter()
    //     .enumerate()
    //     .map(|(index, (key, value))| {
    //         let mut row = Row::new(vec![key.as_str(), value.as_str()]);
    //         if state.completed_optional_fields.iter().any(|&(first, _)| first == index) {
    //             row = row.style(Style::default().fg(Color::Green));
    //         }
    //         row
    //     })
    //     .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::new())
            .header(
                Row::new([translate("optional_field"), translate("result_value")])
                    .style(Style::new().add_modifier(Modifier::BOLD)),
            )
            .highlight_style(optionalfields_style),
        right_optional_fields,
        buf,
        &mut table_state,
    );

    render_breadcrumbs(state, breadcrumbs, buf);

    render_mapping_bar(bottom, buf, state, mappingoptions_style);

    if state.popup_mapping_p2_p3 {
        if state.select_mapping_option {
            render_popup_mapping(area, buf, state);
        } else {
            match state.mapping_option {
                MappingOptions::Transformations => render_popup_mapping(area, buf, state),
                MappingOptions::OneToMany => render_popup_mapping(area, buf, state), //todo
                MappingOptions::ManyToOne => render_manytoone_bar(area, buf, state), //todo
                MappingOptions::DirectCopy => {}                                     // DirectCopy
            }
        }
    }
    // Render warning if user wants to exit.
    if state.exit_warning {
        render_popup_exit_warning(area, buf);
    }
}
