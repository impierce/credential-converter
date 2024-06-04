use std::collections::HashMap;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};
use serde_json::Value;

use crate::utils::{AppState, PathPrompts, Tabs};

pub fn render_page(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let vertical_sections = Layout::vertical(vec![Constraint::Min(0), Constraint::Length(1)]);
    let [top, bottom_area] = vertical_sections.areas(area);

    match state.tab {
        Tabs::InputPromptsP1 => render_description_input_p1(top, frame.buffer_mut(), state),
        Tabs::ManualMappingP2 => render_manual_mapping_p2(top, frame.buffer_mut(), state),
        Tabs::UnusedDataP3 => render_lost_data_p3(top, frame.buffer_mut(), state),
    }
    render_bottom_bar(bottom_area, frame.buffer_mut());
}

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

    let active_style = Style::default().fg(Color::LightYellow);

    match state.path_prompts {
        PathPrompts::Input => input_prompt = input_prompt.style(active_style),
        PathPrompts::Output => output_prompt = output_prompt.style(active_style),
        PathPrompts::UnusedData => lost_data_prompt = lost_data_prompt.style(active_style),
    };

    let rdr = std::fs::File::open(state.input_path.clone()).unwrap();
    ///
    Paragraph::new(state.input_path.as_str())
        .block(input_prompt)
        .render(input_path, buf);

    Paragraph::new(state.output_path.as_str())
        .block(output_prompt)
        .render(output_path, buf);

    Paragraph::new(state.unused_data_path.as_str())
        .block(lost_data_prompt)
        .render(lost_data_path, buf);
}

pub fn render_manual_mapping_p2(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Manual Mapping  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);

    let vertical_sections = Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Min(0)]);
    let [mut left_selector, mut right_missing_fields] = vertical_sections.areas(area);

    left_selector = left_selector.inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    Block::new().borders(Borders::RIGHT).render(left_selector, buf);

    right_missing_fields = right_missing_fields.inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    let rdr = std::fs::File::open(state.input_path.clone()).unwrap();
    let input_value: Value = serde_json::from_reader(rdr).unwrap();
    let leaf_nodes: HashMap<String, Value> = get_leaf_nodes(input_value);
    let mut input_fields = vec![(String::new(), String::new())];

    for (key, value) in leaf_nodes {
        input_fields.push((key, value.to_string()));
    }

    input_fields.sort();
    state.amount_input_fields = input_fields.len() - 2;

    // if let Some(input_value) = input_value.as_object() {
    //     for (key, value) in input_value {
    //         input_fields.push((key.to_string(), value.to_string()));
    //     }
    // }

    let mut table_state = TableState::default().with_selected(Some(state.selected_input_field));
    let rows: Vec<Row> = input_fields
        .iter()
        .map(|(key, value)| Row::new(vec![key.as_str(), value.as_str()]))
        .collect();

    StatefulWidget::render(
        Table::new(rows, [Constraint::Length(30), Constraint::Length(30)])
            .block(Block::new())
            .header(Row::new(vec!["Field", "Value"]).style(Style::new().add_modifier(Modifier::BOLD)))
            .highlight_style(Style::new().light_yellow()),
        left_selector,
        buf,
        &mut table_state,
    );

    // let mut bla = ListState::default().with_selected(Some(state.selected_input_field));
    // let rows: Vec<ListItem> = input_fields.iter().map(|(key, value)| {
    //     ListItem::new(Span::styled(
    //         format!("{}: {}", key, value),
    //         Style::default().fg(Color::White),
    //     ))    }).collect();

    // StatefulWidget::render(
    //     List::new(rows),
    //         //.block(Block::new())
    //         //.header(ListItem::new("Field - Value").style(Style::new()
    //         //.add_modifier(Modifier::BOLD)))
    //         //.highlight_style(Style::new().light_yellow()),
    //     left_selector,
    //     buf,
    //     &mut bla,
    // );

    Block::new()
        .title("  Missing field here  xxx")
        .render(right_missing_fields, buf);
}

pub fn render_lost_data_p3(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Block::new()
        .title("  Lost Data  ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(area, buf);
}

fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
    let vertical_sections = Layout::horizontal(vec![Constraint::Length(23), Constraint::Min(0)]);
    let [left, right] = vertical_sections.areas(area);

    Block::new()
        .title("  Impierce Technologies ")
        .title_alignment(Alignment::Left)
        .title_style(style::Color::Blue)
        .bg(Color::Black)
        .render(left, buf);

    let keys = [
        ("←↓↑→", "Navigate"),
        ("Tab", "Next Page"),
        ("F2", "Prev Page"),
        ("Enter", "Save"),
        ("Esc", "Quit"),
    ];
    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(
                format!(" {key} "),
                Style::default().on_black().add_modifier(Modifier::BOLD),
            );
            let desc = Span::styled(format!(" {desc} "), Style::default().on_dark_gray());
            [key, desc]
        })
        .collect();
    Line::from(spans)
        .centered()
        .style((Color::Black, Color::Black))
        .render(right, buf);
}

fn extract_leaf_nodes(json_object: &Value, path: String, result: &mut HashMap<String, Value>) {
    match json_object {
        Value::Object(map) => {
            for (key, value) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}/{}", path, key)
                };
                extract_leaf_nodes(value, new_path, result);
            }
        }
        _ => {
            result.insert(path, json_object.clone());
        }
    }
}

fn get_leaf_nodes(json_object: Value) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    extract_leaf_nodes(&json_object, String::new(), &mut result);
    result
        .into_iter()
        .map(|(key, value)| (format!("/{}", key), value))
        .collect()
}

#[test]
fn test_function() {
    // Example usage
    let json_data = r#"
    {
        "a": 1,
        "b": {
            "c": 2,
            "d": {
                "e": 3
            }
        },
        "f": [4, 5, 6]
    }"#;

    let parsed_json: Value = serde_json::from_str(json_data).expect("Invalid JSON");
    let leaf_nodes: HashMap<String, Value> = get_leaf_nodes(parsed_json);
    for (key, value) in &leaf_nodes {
        println!("{}: {}", key, value);
    }
}
