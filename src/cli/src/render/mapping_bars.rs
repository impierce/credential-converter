use crate::state::{AppState, P2P3Tabs};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};

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
    if state.p2_p3_tabs != P2P3Tabs::MappingOptions {
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

pub fn render_onetomany_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
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

pub fn render_manytoone_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
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

pub fn render_mapping_bar_buttons(abort: Rect, review: Rect, state: &mut AppState, buf: &mut Buffer) {
    state.abort_button = abort;
    state.review_button = review;

    Paragraph::new(" Abort ")
        .style(Style::default().fg(Color::Black).bg(Color::Red))
        .render(abort, buf);
    Paragraph::new(" Review ")
        .style(Style::default().fg(Color::Black).bg(Color::Green))
        .render(review, buf);
}
