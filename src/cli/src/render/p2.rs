use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};

use crate::{
    p2_popup::{render_popup_field_value, render_popup_mapping},
    state::{AppState, Multiplicity, P2Tabs},
};

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
    let mut inputfields_style = Style::default().add_modifier(Modifier::UNDERLINED);
    let mut missingfields_style = Style::default().add_modifier(Modifier::UNDERLINED);
    let mut mappingoptions_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    match state.p2_tabs {
        P2Tabs::InputFields => {
            inputfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2Tabs::MissingFields => {
            missingfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2Tabs::MappingOptions => {
            mappingoptions_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
    }

    // Render left selector list of input fields
    let mut table_state = TableState::default().with_selected(Some(state.selected_input_field));
    let rows: Vec<Row> = state
        .input_fields
        .iter()
        .enumerate()
        .map(|(index, (key, value))| {
            let mut row = Row::new(vec![key.as_str(), value.as_str()]);
            if state.completed_input_fields.contains(&index) {
                row = row.style(Style::default().fg(Color::Green));
            }
            row
        })        
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
        ("".to_string(), "".to_string()),
        ("field1".to_string(), "".to_string()),
        ("field2".to_string(), "".to_string()),
        ("field3".to_string(), "".to_string()),
    ]); // todo: remove hard code testdata
    state.amount_missing_fields = state.missing_data_fields.as_ref().unwrap().len() - 2; // todo
    let mut table_state = TableState::default().with_selected(Some(state.selected_missing_field));
    let rows: Vec<Row> = state
        .missing_data_fields
        .as_ref()
        .unwrap() // todo: cleanup unwrap
        .iter()
        .enumerate()
        .map(|(index, (key, value))| {
            let mut row = Row::new(vec![key.as_str(), value.as_str()]);
            if state.completed_input_fields.contains(&index) {
                row = row.style(Style::default().fg(Color::Green));
            }
            row
        })
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
        let [multiplicity_tabs, abort, review] = Layout::horizontal(vec![
            Constraint::Percentage(100),
            Constraint::Length(7),
            Constraint::Length(8),
        ])
        .areas(bottom);

        Tabs::new(multiplicities)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(mappingoptions_style)
            .select(state.multiplicity as usize)
            .divider("")
            .render(multiplicity_tabs, buf);

        render_mapping_bar_buttons(abort, review, state, buf);
    } else {
        match state.multiplicity {
            Multiplicity::OneToOne => render_onetoone_bar(bottom, buf, state),
            Multiplicity::OneToMany => render_onetomany_bar(bottom, buf, state),
            Multiplicity::ManyToOne => render_manytoone_bar(bottom, buf, state),
        }
    }

    if state.popup_mapping_p2 {
        if state.select_multiplicity {
            match state.p2_tabs {
                P2Tabs::InputFields => render_popup_field_value(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                    P2Tabs::InputFields,
                ),
                P2Tabs::MissingFields => render_popup_field_value(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                    P2Tabs::MissingFields,
                ),
                P2Tabs::MappingOptions => { state.popup_mapping_p2 = false; }
            }
        }
        else {
            match state.multiplicity {
                Multiplicity::OneToOne => render_popup_mapping(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
                Multiplicity::OneToMany => render_popup_mapping( //
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
                Multiplicity::ManyToOne => render_manytoone_bar(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
            }
        }
    }
}

pub fn render_onetoone_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let tabs = vec![" None", "LowerCase", "UpperCase", "Slice", "Regex"];

    let [transformations, selected, abort, review] = Layout::horizontal(vec![
        Constraint::Min(tabs.concat().len() as u16 + 10),
        Constraint::Percentage(100),
        Constraint::Length(7),
        Constraint::Length(8),
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
            .highlight_style(Style::default().fg(Color::Black))
            .select(state.selected_transformation as usize)
            .divider("")
            .render(selected, buf);
    } else {
        Tabs::new(tabs)
            .style(Style::default().fg(Color::Black).bg(Color::Gray))
            .highlight_style(Style::default().fg(Color::Black))
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

    render_mapping_bar_buttons(abort, review, state, buf);
}

fn render_onetomany_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let txt = "  Enter a divider, or select indices manually: ";
    let [txt_area, dividers, abort, review] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16),
        Constraint::Percentage(100),
        Constraint::Length(7),
        Constraint::Length(8),
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

    render_mapping_bar_buttons(abort, review, state, buf);
}

fn render_manytoone_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let txt = "  Select multiple fields in the left tab, the result is shown in the right tab. ";
    let [txt_area, abort, review] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16 + 2),
        Constraint::Length(7),
        Constraint::Length(8),
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

    render_mapping_bar_buttons(abort, review, state, buf);
}

fn render_mapping_bar_buttons(abort: Rect, review: Rect, state: &mut AppState, buf: &mut Buffer) {
    state.abort_button = abort;
    state.review_button = review;

    Paragraph::new(" Abort ")
        .style(Style::default().fg(Color::Black).bg(Color::Red))
        .render(abort, buf);
    Paragraph::new(" Review ")
        .style(Style::default().fg(Color::Black).bg(Color::Green))
        .render(review, buf);
}
