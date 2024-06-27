use crate::{
    state::{AppState, P2P3Tabs},
    trace_dbg,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};

pub fn render_transformations_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let tabs = vec![" LowerCase", "UpperCase", "Slice", "Regex"];

    let [transformations, selected, abort, view] = Layout::horizontal(vec![
        Constraint::Min(tabs.concat().len() as u16 + 10),
        Constraint::Percentage(100),
        Constraint::Length(7),
        Constraint::Length(6),
    ])
    .areas(area);

    Block::new()
        .border_type(BorderType::Thick)
        .borders(Borders::RIGHT)
        .render(transformations, buf);

    let mut active_style = Style::default().fg(Color::White).bg(Color::DarkGray);
    let mut active_style_selected_tab = Style::default().fg(Color::White).bg(Color::DarkGray);
    if state.p2_p3_tabs == P2P3Tabs::MappingOptions {
        if !state.selected_transformations_tab {
            active_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        else {
            active_style_selected_tab = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
    }

    // Render options tab left and the selected options to the right
    Tabs::new(tabs)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .highlight_style(active_style)
        .select(state.transformations as usize - 1) // todo: this ' - 1 ' is due to the fact i didn't want to dive into the backend just yet, which is depending on a "Copy" variant in the fn selector to complete a mapping. This means "Copy" is not shown in the tabber, but does exist in the enum Transformations
        .divider("")
        .render(transformations, buf);
    let selected_transformations: Vec<String> =
        state.selected_transformations.iter().map(|x| x.to_string()).collect();
    // if no transformations selected to show and tab is active, show a yellow cursor
    if selected_transformations.is_empty() && state.selected_transformations_tab {
        Tabs::new(vec![" __"])
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(active_style_selected_tab)
            .divider("")
            .render(selected, buf);        
    }
    else {
        Tabs::new(selected_transformations)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(active_style_selected_tab)
            .select(state.selected_transformation as usize)
            .divider("")
            .render(selected, buf);
    }

    render_mapping_bar_buttons(abort, view, state, buf);
}

pub fn render_onetomany_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let txt = "  Enter a divider, or select indices manually: ";
    let [txt_area, dividers, abort, view] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16),
        Constraint::Percentage(100),
        Constraint::Length(7),
        Constraint::Length(6),
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

    render_mapping_bar_buttons(abort, view, state, buf);
}

pub fn render_manytoone_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let txt = "  Select multiple fields in the left tab, the result is shown in the right tab. ";
    let [txt_area, abort, view] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16 + 2),
        Constraint::Length(7),
        Constraint::Length(6),
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

    render_mapping_bar_buttons(abort, view, state, buf);
}

pub fn render_mapping_bar_buttons(abort: Rect, view: Rect, state: &mut AppState, buf: &mut Buffer) {
    state.abort_button = abort;
    state.view_button = view;

    let mut clearresult_abort_style = Style::default().fg(Color::Black).bg(Color::Red);
    let mut view_style = Style::default().fg(Color::Black).bg(Color::Green);
    match state.p2_p3_tabs {
        P2P3Tabs::ClearResultAbort => {
            clearresult_abort_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2P3Tabs::View => {
            view_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        _ => {}
    }

    if state.select_mapping_option {
        Paragraph::new(" Clear ")
            .style(clearresult_abort_style)
            .render(abort, buf);
    }
    else {
        Paragraph::new(" Abort ")
            .style(clearresult_abort_style)
            .render(abort, buf);
    }
    Paragraph::new(" View ")
        .style(view_style)
        .render(view, buf);
}
