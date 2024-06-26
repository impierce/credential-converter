use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    prelude::*,
    widgets::*,
};
use std::path::Path;

use crate::{
    popups::render_popup_overwrite_warning, state::{AppState, P1Prompts}, trace_dbg
};

pub fn render_description_input_p1(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    // Main title at the top of p1
    Block::new()
        .title("  OB-ELM Mapper  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    let vertical_sections = Layout::horizontal(vec![Constraint::Length(50), Constraint::Min(0)]);
    let [left_description, right_prompts] = vertical_sections.areas(area);

    // Left description area
    let description = format!("\n
    This tool is made for converting credentials between the OpenBadges and the ELM format.
    \nIt takes a json file as input and outputs the new, mapped json file. Unmapped fields can be mapped manually. Unused data will be stored in a seperate file. Manual mappings can be saved to a custom mapping file for future use.
    \nFor absolute paths start with '/', for relative paths the current directory is: {}",
    std::env::current_dir().unwrap().display());
    Paragraph::new(description)
        .wrap(Wrap { trim: true })
        .render(left_description, buf);

    // Right prompts area
    let prompts_area = right_prompts.inner(&Margin {
        vertical: 2,
        horizontal: 1,
    });
    let input_prompts = Layout::vertical(vec![
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
    ]);
    let [input_path, output_path, mapping_file, mapping] = input_prompts.areas(prompts_area);

    let mut input_prompt = Block::new()
        .title("  Input Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut output_prompt = Block::new()
        .title("  Output Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut mapping_prompt = Block::new()
        .title("  Choose Mapping  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut mapping_file_prompt = Block::new()
        .title("  Choose Mapping File  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let active_style = Style::default().fg(Color::LightYellow);
    match state.p1_prompts {
        P1Prompts::Input => input_prompt = input_prompt.style(active_style),
        P1Prompts::Output => output_prompt = output_prompt.style(active_style),
        P1Prompts::Mapping => mapping_prompt = mapping_prompt.style(active_style),
        P1Prompts::MappingFile => mapping_file_prompt = mapping_file_prompt.style(active_style),
    };

    // Checking paths for validity/overwriting.
    let path = Path::new(&state.input_path);
    if !path.is_file() {
        Paragraph::new(state.input_path.as_str())
            .block(input_prompt)
            .fg(Color::Red)
            .render(input_path, buf);
    } else {
        Paragraph::new(state.input_path.as_str())
            .block(input_prompt)
            .fg(Color::Green)
            .render(input_path, buf);
    }

    let path = Path::new(&state.output_path);
    if state.output_path.is_empty() {
        Paragraph::new(state.output_path.as_str())
            .block(output_prompt)
            .fg(Color::Red)
            .render(output_path, buf);
    }        
    else if !path.is_file() {
        trace_dbg!(&state.output_path);
        Paragraph::new(state.output_path.as_str())
            .block(output_prompt)
            .fg(Color::Green)
            .render(output_path, buf);
    } else {
        Paragraph::new(state.output_path.as_str())
            .block(output_prompt)
            .fg(Color::Rgb(240, 160, 100))
            .render(output_path, buf);
    }

    let path = Path::new(&state.mapping_path);
    if !path.exists() || !path.is_file() {
        Paragraph::new(state.mapping_path.as_str())
            .block(mapping_file_prompt)
            .fg(Color::Red)
            .render(mapping_file, buf);
    } else {
        Paragraph::new(state.mapping_path.as_str())
            .block(mapping_file_prompt)
            .fg(Color::Green)
            .render(mapping_file, buf);
    }

    // Mapping tabber prompt
    mapping_prompt.render(mapping, buf);
    let mapping_prompt_inner = mapping.inner(&Margin {
        vertical: 1,
        horizontal: 0,
    });

    let tabs = vec![" OBv3 -> ELM ", " ELM -> OBv3 "];
    let [_left, tabs_center, _right] = Layout::horizontal(vec![
        Constraint::Min(1),
        Constraint::Max(tabs.concat().len() as u16 + 2),
        Constraint::Min(1),
    ])
    .areas(mapping_prompt_inner);

    Tabs::new(tabs)
        .style(Style::default().fg(Color::White))
        .highlight_style(Color::Yellow)
        .select(state.mapping as usize)
        .divider("")
        .render(tabs_center, buf);

    // Render warning popup at the end so it doesnt get overwritten by previous renders.
    if state.output_warning {
        render_popup_overwrite_warning(area.inner(&Margin {
            vertical: 4,
            horizontal: 30,
        }), buf);
    }
}
