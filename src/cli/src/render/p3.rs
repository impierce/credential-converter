use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};

use crate::{
    backend::selector::selector,
    mapping_bars::{
        render_manytoone_bar, render_mapping_bar_buttons, render_onetomany_bar, render_transformations_bar,
    },
    popups::{render_popup_field_value, render_popup_mapping},
    state::{AppState, MappingOptions, P2P3Tabs},
    trace_dbg,
};

pub fn render_lost_data_p3(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Unused Data  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    // Layout
    let [_title, page, bottom] =
        Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)]).areas(area);
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

    // Render right tab containing optional fields
    state.amount_optional_fields = state.optional_fields.len() - 2; // todo
    let mut table_state = TableState::default().with_selected(Some(state.selected_optional_field));
    let rows: Vec<Row> = state
        .optional_fields
        .iter()
        .enumerate()
        .map(|(index, (key, value))| {
            let mut row = Row::new(vec![key.as_str(), value.as_str()]);
            if state.completed_optional_fields.contains(&index) {
                row = row.style(Style::default().fg(Color::Green));
            }
            row
        })
        .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
            .block(Block::new())
            .header(Row::new(vec!["Optional Field", "Result Value"]).style(Style::new().add_modifier(Modifier::BOLD)))
            .highlight_style(optionalfields_style),
        right_optional_fields,
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

                state.completed_input_fields.push(state.selected_input_field);
                state.completed_optional_fields.push(state.selected_optional_field);
                state.optional_fields[state.selected_optional_field].1 = state.candidate_data_value.clone().unwrap();
                trace_dbg!(state.candidate_data_value.as_ref().unwrap());
                trace_dbg!(&state.optional_fields[state.selected_optional_field]);

                if state.selected_input_field == state.input_fields.len() - 1 {
                    state.selected_input_field = 1;
                } else {
                    state.selected_input_field += 1;
                }

                if state.selected_optional_field == state.optional_fields.len() - 1 {
                    state.selected_optional_field = 1;
                } else {
                    state.selected_optional_field += 1;
                }
            } // DirectCopy
        }
    }

    if state.popup_mapping_p2_p3 {
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
