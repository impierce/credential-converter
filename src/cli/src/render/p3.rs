use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};

use crate::{
    mapping_bars::{render_manytoone_bar, render_mapping_bar_buttons, render_onetomany_bar, render_onetoone_bar},
    popups::{render_popup_field_value, render_popup_mapping, render_popup_uncompleted_warning_p2},
    state::{AppState, Multiplicity, P2P3Tabs},
};

pub fn render_lost_data_p3(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Lost Data  ")
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
    let mut inputfields_style = Style::default().add_modifier(Modifier::UNDERLINED);
    let mut optionalfields_style = Style::default().add_modifier(Modifier::UNDERLINED);
    let mut mappingoptions_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
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
            .header(Row::new(vec!["optional Field", "Result Value"]).style(Style::new().add_modifier(Modifier::BOLD)))
            .highlight_style(optionalfields_style),
        right_optional_fields,
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

    if state.popup_uncompleted_warning {
        render_popup_uncompleted_warning_p2(
            area.inner(&Margin {
                vertical: 4,
                horizontal: 20,
            }),
            buf,
        );
    } else if state.popup_mapping_p2_p3 {
        if state.select_multiplicity {
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
            match state.multiplicity {
                Multiplicity::OneToOne => render_popup_mapping(
                    area.inner(&Margin {
                        vertical: 4,
                        horizontal: 20,
                    }),
                    buf,
                    state,
                ),
                Multiplicity::OneToMany => render_popup_mapping(
                    //
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
