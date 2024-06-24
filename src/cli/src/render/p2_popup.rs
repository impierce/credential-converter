use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};
use ratatui::{prelude::*, widgets::*};

use crate::{backend::selector::selector, state::{self, AppState, P2Tabs}};

pub fn render_popup_mapping(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    Clear.render(area, buf);
    Block::new().style(Style::default().bg(Color::Black)).render(area, buf);

    let [left, right] = Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
    let [left_top, left_bottom] = Layout::vertical(vec![Constraint::Percentage(50), Constraint::Percentage(50)]).areas(left);
    let [right_top, right_bottom] = Layout::vertical(vec![Constraint::Percentage(50), Constraint::Percentage(50)]).areas(right);
    
    state.popup_input_path_p2 = left_top;
    state.popup_output_path_p2 = right_top;
    state.popup_input_value_p2 = left_bottom;
    state.popup_output_result_p2 = right_bottom;
    
    selector(state);

    Block::new()
        .title("  Input Path  ")
        .add_modifier(Modifier::BOLD)
        .title_alignment(Alignment::Center)
        .borders(Borders::BOTTOM)
        .render(left_top, buf);
    Block::new()
        .borders(Borders::RIGHT)
        .border_type(BorderType::Thick)
        .render(left_top, buf);

    Block::new()
        .title("  Output Path  ")
        .add_modifier(Modifier::BOLD)
        .title_alignment(Alignment::Center)
        .borders(Borders::BOTTOM)
        .render(right_top, buf);

    Block::new()
        .title("  Input Value  ")
        .add_modifier(Modifier::BOLD)
        .title_alignment(Alignment::Center)
        .borders(Borders::RIGHT)
        .border_type(BorderType::Thick)
        .render(left_bottom, buf);
    Block::new()
        .title("  Output Result  ")
        .add_modifier(Modifier::BOLD)
        .title_alignment(Alignment::Center)
        .render(right_bottom, buf);

    // Calculate maximum lines used, this sets the maximum scroll offset
    if right.width > 2 {
        state.popup_amount_lines_path = state.input_fields[state.selected_input_field].0.len() / (right.width as usize - 2);
        state.popup_amount_lines_value = state.input_fields[state.selected_input_field].1.len() / (right.width as usize - 2);
        state.popup_amount_lines_output_path = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].0.len() / (right.width as usize - 2);
        state.popup_amount_lines_result = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].1.len() / (right.width as usize - 2);
    }
 
    Paragraph::new(state.input_fields[state.selected_input_field].0.as_str())
        .wrap(Wrap { trim: false })
        .remove_modifier(Modifier::BOLD)
        .scroll((state.popup_offset_path, 0))
        .render(
            left_top.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );

    Paragraph::new(state.input_fields[state.selected_input_field].1.as_str())
        .wrap(Wrap { trim: false })
        .remove_modifier(Modifier::BOLD)
        .scroll((state.popup_offset_value, 0))
        .render(
            left_bottom.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );

    Paragraph::new(state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].0.as_str())
        .wrap(Wrap { trim: false })
        .remove_modifier(Modifier::BOLD)
        .scroll((state.popup_offset_path, 0))
        .render(
            right_top.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );

    Paragraph::new(state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].1.as_str())
        .wrap(Wrap { trim: false })
        .remove_modifier(Modifier::BOLD)
        .scroll((state.popup_offset_value, 0))
        .render(
            right_bottom.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );
    
    let [_top, confirm_area] = Layout::vertical(vec![Constraint::Percentage(100), Constraint::Length(1)]).areas(right_bottom);
    let [_left, confirm_area] = Layout::horizontal(vec![Constraint::Percentage(100), Constraint::Length(9)]).areas(confirm_area);
    state.confirm_button = confirm_area;
    Paragraph::new(" Confirm ")
        .style(Style::new().fg(Color::Black).bg(Color::Green))
        .render(
            confirm_area,
            buf,
        );
}

pub fn render_popup_field_value(area: Rect, buf: &mut Buffer, state: &mut AppState, tab: P2Tabs) {
    Clear.render(area, buf);
    Block::new().style(Style::default().bg(Color::Black)).render(area, buf);
    let [left, right] = Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
    state.popup_path_area_p2 = left;
    state.popup_value_area_p2 = right;

    // (if tab == P2Tabs::InputFields)
    let mut field = state.input_fields[state.selected_input_field].0.as_str();
    let mut value = state.input_fields[state.selected_input_field].1.as_str();
    let mut title_left = "  Field  ";
    let mut title_right = "  Value  ";
    // Calculate maximum lines used, this sets the maximum scroll offset
    if right.width > 2 {
        state.popup_amount_lines_path = state.input_fields[state.selected_input_field].0.len() / (right.width as usize - 2);
        state.popup_amount_lines_value = state.input_fields[state.selected_input_field].1.len() / (right.width as usize - 2);
    }
    
    if tab == P2Tabs::MissingFields {
        title_left = "  Missing Field  ";
        title_right = "  Result Value  ";
        field = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].0.as_str();
        value = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].1.as_str();
        // Calculate maximum lines used, this sets the maximum scroll offset
        if right.width > 2 {
            state.popup_amount_lines_path = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].0.len() / (right.width as usize - 2);
            state.popup_amount_lines_value = state.missing_data_fields.as_ref().unwrap()[state.selected_missing_field].1.len() / (right.width as usize - 2);
        }
    }

    Block::new()
        .title(title_left)
        .add_modifier(Modifier::BOLD)
        .title_alignment(Alignment::Center)
        .borders(Borders::RIGHT)
        .border_type(BorderType::Thick)
        .render(left, buf);

    Block::new()
        .title(title_right)
        .add_modifier(Modifier::BOLD)
        .title_alignment(Alignment::Center)
        .render(right, buf);

    Paragraph::new(field)
        .wrap(Wrap { trim: false })
        .remove_modifier(Modifier::BOLD)
        .scroll((state.popup_offset_path, 0))
        .render(
            left.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );

    Paragraph::new(value)
        .wrap(Wrap { trim: false })
        .remove_modifier(Modifier::BOLD)
        .scroll((state.popup_offset_value, 0))
        .render(
            right.inner(&Margin {
                horizontal: 1,
                vertical: 1,
            }),
            buf,
        );
}

pub fn render_popup_uncompleted_warning(area: Rect, buf: &mut Buffer) {
    Clear.render(area, buf);
    Block::new().style(Style::default().fg(Color::Red).bg(Color::Black))
        .borders(Borders::ALL)
        .render(area, buf);

    let vertical_margin;
    if area.height >= 3 {
        vertical_margin = (area.height - 4) / 2;
    }
    else {
        vertical_margin = 0;
    }
    Paragraph::new("\n Not all missing fields are completed.\nContinuing now will render an invalid output file.\nPress 'Enter' to continue, 'Esc' to go back.")
        .centered()
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
        .render(area.inner(&Margin {
            vertical: vertical_margin,
            horizontal: 1,
        }), buf);
}
