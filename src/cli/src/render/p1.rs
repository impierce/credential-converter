use std::path::Path;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};

use crate::{trace_dbg, utils::{AppState, P1Prompts}};

pub fn render_description_input_p1(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  OB-ELM Mapper  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    let vertical_sections = Layout::horizontal(vec![Constraint::Length(50), Constraint::Min(0)]);
    let [left_description, right_prompts] = vertical_sections.areas(area);

    let text = format!("\n
    This tool is made for converting credentials between the OpenBadges and the ELM format.
    \nIt takes a json file as input and outputs the new, mapped json file. Unmapped fields can be mapped manually. Unused data will be stored in a seperate file. Manual mappings can be saved to a custom mapping file for future use.
    \nFor absolute paths start with '/', for relative paths the current directory is: {}",
    std::env::current_dir().unwrap().display());

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .render(left_description, buf);

    let prompts_area = right_prompts.inner(&Margin {
        vertical: 2,
        horizontal: 1,
    });
    let input_prompts = Layout::vertical(vec![
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
    ]);
    let [input_path, output_path, mapping] = input_prompts.areas(prompts_area);

    let mut input_prompt = Block::new()
        .title("  Input Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut output_prompt = Block::new()
        .title("  Output Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut mapping_prompt = Block::new()
        .title("  Choose Mapping  ") // todo: automatically infer unused data path from output_path
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let active_style = Style::default().fg(Color::LightYellow);

    match state.p1_prompts {
        P1Prompts::Input => input_prompt = input_prompt.style(active_style),
        P1Prompts::Output => output_prompt = output_prompt.style(active_style),
        P1Prompts::Mapping => mapping_prompt = mapping_prompt.style(active_style),
    };

    let path = Path::new(&state.input_path);
    if !path.exists() || !path.is_file() {
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

    Paragraph::new(state.output_path.as_str())
        .block(output_prompt)
        .render(output_path, buf);

    // Paragraph::new(state.unused_data_path.as_str())
    //     .block(mapping_prompt)
    //     .render(lost_data_path, buf);
    // Need to add this option again at the end

    mapping_prompt.render(mapping, buf);
    let mapping_prompt_inner = mapping.inner(&Margin {
        vertical: 1,
        horizontal: 0,
    });

    let string_ = " OBv3 -> ELM ".to_owned() + " ELM -> OBv3 ";
    let [_left, tabs_center, _right] = Layout::horizontal(vec![
        Constraint::Min(1),
        Constraint::Max(string_.len() as u16 + 2),
        Constraint::Min(1),
    ]).areas(mapping_prompt_inner);

    Tabs::new(vec![" OBv3 -> ELM ", " ELM -> OBv3 "])
        .style(Style::default().fg(Color::White))
        .highlight_style(Color::Yellow)
        .select(state.mapping as usize)
        .divider("")
        .render(tabs_center, buf,
    );
}
