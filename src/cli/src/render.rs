use ratatui::{
    buffer::Buffer, layout::{Rect, Constraint}, widgets::{Block, Paragraph}
};
use ratatui::{prelude::*, widgets::*};

use crate::utils::{AppState, PathPrompts, Tabs};

pub fn render_page(frame: &mut Frame, area: Rect, state: &mut AppState) {
    match state.tab {
        Tabs::InputPromptsP1 => render_description_input_p1(area, frame.buffer_mut(), state),
        Tabs::ManualMappingP2 => render_manual_mapping_p2(area, frame.buffer_mut(), state),
        Tabs::UnusedDataP3 => render_lost_data_p3(area, frame.buffer_mut(), state),
    }
}

pub fn render_description_input_p1(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    
    Block::new()
        .title("  OB-ELM Mapper  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    Block::new()
        .title_bottom("  Made by Impierce Technologies  ")
        .title_alignment(Alignment::Center)
        // .borders(Borders::BOTTOM) // Decisions on looks might be nice to run by Soula for a quick look before delivery.
        .render(area, buf);

    let vertical_sections = Layout::horizontal(vec![
        Constraint::Length(40),
        Constraint::Min(0),
    ]);
    let [left_description, right_prompts] = vertical_sections.areas(area);

    let text = "\n
    This tool is made for converting credentials between the OpenBadges and the ELM format.
    It takes a json file as input and after the mapping it returns a new json file as output.
    Unmapped fields can be mapped manually and unused data will be stored in a seperate file.
    All manual mappings can be saved to a custom mapping file for future use.";

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
    let [input_path, output_path, lost_data_path] = input_prompts.areas(prompts_area);
    
    let mut input_prompt = Block::new()
        .title("  Input Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut output_prompt = Block::new()
        .title("  Output Path  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let mut lost_data_prompt = Block::new()
        .title("  Unused Data Path  ") // todo rename lost data
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let active_style =
        Style::default().fg(Color::LightYellow);

    match state.path_prompts {
        PathPrompts::Input => input_prompt = input_prompt.style(active_style),
        PathPrompts::Output => output_prompt = output_prompt.style(active_style),
        PathPrompts::UnusedData => lost_data_prompt =lost_data_prompt.style(active_style),

    };

    Paragraph::new(state.input_path.as_str())
        .block(input_prompt)
        .render(input_path, buf);

    Paragraph::new(state.output_path.as_str())
        .block(output_prompt)
        .render(output_path, buf);

    Paragraph::new(state.unused_data_path.as_str())
        .block(lost_data_prompt)
        .render(lost_data_path, buf);

    // input_prompt.render(input_path, buf);
    // output_prompt.render(output_path, buf);
    // lost_data_prompt.render(lost_data_path, buf);

}

pub fn render_manual_mapping_p2(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Manual Mapping  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .render(area, buf);
}

//
pub fn render_lost_data_p3(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Lost Data  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .render(area, buf);
}
