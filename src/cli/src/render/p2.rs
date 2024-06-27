use crate::{
    backend::selector::selector,
    mapping_bars::{
        render_manytoone_bar, render_mapping_bar_buttons, render_onetomany_bar, render_transformations_bar,
    },
    p2_handler::update_repository,
    popups::{render_popup_field_value, render_popup_mapping, render_popup_uncompleted_warning_p2},
    state::{AppState, MappingOptions, P2P3Tabs},
    trace_dbg,
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
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
    state.selector_area_p2_p3 = left_selector;
    state.output_fields_area_p2_p3 = right_missing_fields;

    // Highlight active area
    let mut inputfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    let mut missingfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    let mut mappingoptions_style = Style::default().fg(Color::White);
    match state.p2_p3_tabs {
        P2P3Tabs::InputFields => {
            inputfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2P3Tabs::OutputFields => {
            missingfields_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2P3Tabs::MappingOptions => {
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
            .header(Row::new(vec!["Field", "Value"]).style(Style::new()))
            .highlight_style(inputfields_style),
        left_selector,
        buf,
        &mut table_state,
    );

    // Render right tab containing missing fields
    state.amount_missing_fields = state.missing_data_fields.len() - 2; // todo
    let mut table_state = TableState::default().with_selected(Some(state.selected_missing_field));
    let rows: Vec<Row> = state
        .missing_data_fields
        .iter()
        .enumerate()
        .map(|(index, (key, value))| {
            let mut row = Row::new(vec![key.as_str(), value.as_str()]);
            if state.completed_missing_fields.contains(&index) {
                row = row.style(Style::default().fg(Color::Green));
            }
            row
        })
        .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::new())
            .header(Row::new(vec!["Missing Field", "Result Value"]).style(Style::new()))
            .highlight_style(missingfields_style),
        right_missing_fields,
        buf,
        &mut table_state,
    );
    // todo: render  output results

    // Render bottom mapping options bar
    if state.select_mapping_option {
        let multiplicities = vec![" DirectCopy", "Transformations", "OneToMany", "ManyToOne"];
        let [multiplicity_tabs, abort, review] = Layout::horizontal(vec![
            Constraint::Percentage(100),
            Constraint::Length(7),
            Constraint::Length(8),
        ])
        .areas(bottom);

        Tabs::new(multiplicities)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(mappingoptions_style)
            .select(state.mapping_option as usize)
            .divider("")
            .render(multiplicity_tabs, buf);

        render_mapping_bar_buttons(abort, review, state, buf);
    } else {
        match state.mapping_option {
            MappingOptions::Transformations => render_transformations_bar(bottom, buf, state),
            MappingOptions::OneToMany => render_onetomany_bar(bottom, buf, state),
            MappingOptions::ManyToOne => render_manytoone_bar(bottom, buf, state),
            _ => {
                // this is actually event handling and should be moved
                selector(state);
                state.selected_transformations_tab = false;
                state.select_mapping_option = true;
                state.selected_transformations.clear();
                state.popup_offset_path = 0;
                state.popup_offset_value = 0;
                state.p2_p3_tabs = P2P3Tabs::InputFields;

                if !state.completed_input_fields.contains(&state.selected_input_field) {
                    state.completed_input_fields.push(state.selected_input_field);
                }
                if !state.completed_missing_fields.contains(&state.selected_missing_field) {
                    state.completed_missing_fields.push(state.selected_missing_field);
                }
                state.missing_data_fields[state.selected_missing_field].1 = state.candidate_data_value.clone().unwrap();
                trace_dbg!(state.candidate_data_value.as_ref().unwrap());
                trace_dbg!(&state.missing_data_fields[state.selected_missing_field]);

                if state.selected_input_field == state.input_fields.len() - 1 {
                    state.selected_input_field = 1;
                } else {
                    state.selected_input_field += 1;
                }

                if state.selected_missing_field == state.missing_data_fields.len() - 1 {
                    state.selected_missing_field = 1;
                } else {
                    state.selected_missing_field += 1;
                }

                update_repository(state);
            } // DirectCopy
        }
    }

    if state.popup_uncompleted_warning {
        render_popup_uncompleted_warning_p2(
            area.inner(&Margin {
                vertical: 4,
                horizontal: 20,
            }),
            buf,
        );
    } else if state.popup_mapping_p2_p3 {
        if state.select_mapping_option {
            match state.p2_p3_tabs {
                P2P3Tabs::InputFields => render_popup_field_value(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                    P2P3Tabs::InputFields,
                ),
                P2P3Tabs::OutputFields => render_popup_field_value(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                    P2P3Tabs::OutputFields,
                ),
                P2P3Tabs::MappingOptions => {
                    state.popup_mapping_p2_p3 = false;
                }
            }
        } else {
            match state.mapping_option {
                MappingOptions::Transformations => render_popup_mapping(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
                MappingOptions::OneToMany => render_popup_mapping(
                    //
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
                MappingOptions::ManyToOne => render_manytoone_bar(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
                _ => {} // DirectCopy
            }
        }
    }
}
