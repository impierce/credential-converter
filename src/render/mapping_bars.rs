use crate::state::{translate, AppState, MappingOptions, P2P3Tabs, Transformations};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};

pub fn render_mapping_bar(bottom: Rect, buf: &mut Buffer, state: &mut AppState, style: Style) {
    // Render bottom mapping options bar
    if state.select_mapping_option {
        // Get the translation first to calculate how much space the texts need
        let multiplicities = [
            format!(" {}", translate("direct_copy")),
            translate("transformations").to_string(),
            translate("one_to_many").to_string(),
            translate("many_to_one").to_string(),
        ];
        let clear_len = format!(" {} ", translate("clear")).chars().count() as u16;
        let view_len = format!(" {} ", translate("view")).chars().count() as u16;

        let [multiplicity_tabs, clear, view] = Layout::horizontal([
            Constraint::Percentage(100),
            Constraint::Length(clear_len),
            Constraint::Length(view_len),
        ])
        .areas(bottom);

        Tabs::new(multiplicities)
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .highlight_style(style)
            .select(state.mapping_option as usize)
            .divider("")
            .render(multiplicity_tabs, buf);

        render_mapping_bar_buttons(clear, view, state, buf);
    } else {
        match state.mapping_option {
            MappingOptions::Transformations => render_transformations_bar(bottom, buf, state),
            MappingOptions::OneToMany => render_onetomany_bar(bottom, buf, state),
            MappingOptions::ManyToOne => render_manytoone_bar(bottom, buf, state),
            MappingOptions::DirectCopy => {}
        }
    }
}

pub fn render_transformations_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    // Get the translation first to calculate how much space the texts need
    let tabs = [
        format!(" {}", translate("lowercase")),
        translate("uppercase").to_string(),
        translate("takeindex").to_string(),
        translate("slice").to_string(),
        "Regex".to_string(),
    ];
    let clear_len = format!(" {} ", translate("clear")).chars().count() as u16;
    let view_len = format!(" {} ", translate("view")).chars().count() as u16;

    let [transformations, selected, clear, view] = Layout::horizontal(vec![
        Constraint::Min(tabs.concat().chars().count() as u16 + 11),
        Constraint::Percentage(100),
        Constraint::Length(clear_len),
        Constraint::Length(view_len),
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
        } else {
            active_style_selected_tab = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
    }

    // Render options tab on the left
    Tabs::new(tabs)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .highlight_style(active_style)
        .select(state.transformations as usize - 1) // todo: this ' - 1 ' is due to the "DirectCopy" enum variant which is not shown in the tabber.
        .divider("")
        .render(transformations, buf);

    // Render the selected options to the right // todo: remove index setter when this tab is active.
    if state.transformations == Transformations::TakeIndex || state.transformations == Transformations::Slice {
        let txt = format!(" {} ", translate("enter_index"));
        if let Some(index) = state.transformation_index {
            Paragraph::new(txt + &index.to_string())
                .style(Style::new().fg(Color::Green).bg(Color::DarkGray))
                .render(selected, buf);
        } else {
            Paragraph::new(txt)
                .style(Style::new().fg(Color::Red).bg(Color::DarkGray))
                .render(selected, buf);
        }
    } else {
        let selected_transformations: Vec<String> = state
            .selected_transformations
            .iter()
            .map(|x| translate(&x.to_string().to_lowercase()).to_string())
            .collect();
        // if no transformations selected to show and tab is active, show a yellow cursor
        if selected_transformations.is_empty() && state.selected_transformations_tab {
            Tabs::new(vec![" __"])
                .style(Style::default().fg(Color::White).bg(Color::DarkGray))
                .highlight_style(active_style_selected_tab)
                .divider("")
                .render(selected, buf);
        } else {
            Tabs::new(selected_transformations)
                .style(Style::default().fg(Color::White).bg(Color::DarkGray))
                .highlight_style(active_style_selected_tab)
                .select(state.selected_transformation)
                .divider("")
                .render(selected, buf);
        }
    }

    render_mapping_bar_buttons(clear, view, state, buf);
}

pub fn render_onetomany_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    // Get the translation first to calculate how much space the texts need
    let txt = format!("  {} ", translate("enter_divider"));
    let clear_len = format!(" {} ", translate("clear")).chars().count() as u16;
    let view_len = format!(" {} ", translate("view")).chars().count() as u16;

    let [txt_area, dividers, clear, view] = Layout::horizontal(vec![
        Constraint::Min(txt.chars().count() as u16),
        Constraint::Percentage(100),
        Constraint::Length(clear_len),
        Constraint::Length(view_len),
    ])
    .areas(area);

    Paragraph::new(txt)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .render(txt_area, buf);

    // Display italic instructions to be overwritten by user input for dividers.
    if state.dividers.is_empty() {
        let txt = format!("{} ", translate("select_output"));
        Paragraph::new(txt)
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

    render_mapping_bar_buttons(clear, view, state, buf);
}

pub fn render_manytoone_bar(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    // Get the translation first to calculate how much space the texts need
    let txt = format!("  {} ", translate("select_fields"));
    let clear_len = format!(" {} ", translate("clear")).chars().count() as u16;
    let view_len = format!(" {} ", translate("view")).chars().count() as u16;

    let [txt_area, clear, view] = Layout::horizontal(vec![
        Constraint::Min(txt.len() as u16 + 2),
        Constraint::Length(clear_len),
        Constraint::Length(view_len),
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

    render_mapping_bar_buttons(clear, view, state, buf);
}

pub fn render_mapping_bar_buttons(clear: Rect, view: Rect, state: &mut AppState, buf: &mut Buffer) {
    let clear_txt = format!(" {} ", translate("clear"));
    let view_txt = format!(" {} ", translate("view"));

    state.clear_button = clear;
    state.view_button = view;

    let mut clear_style = Style::default().fg(Color::Black).bg(Color::Red);
    let mut view_style = Style::default().fg(Color::Black).bg(Color::Green);
    match state.p2_p3_tabs {
        P2P3Tabs::Clear => {
            clear_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        P2P3Tabs::View => {
            view_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        _ => {}
    }

    Paragraph::new(clear_txt).style(clear_style).render(clear, buf);
    Paragraph::new(view_txt).style(view_style).render(view, buf);
}
