use crate::{
    p2_p3_common::{
        handle_backspace, handle_char, handle_down, handle_enter, handle_esc, handle_f2, handle_left, handle_mouse_up,
        handle_right, handle_scroll_down, handle_scroll_up, handle_tab, handle_up,
    },
    state::AppState,
};

use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    handle_esc(state);
                }
                Backspace => {
                    handle_backspace(state);
                }
                Tab => {
                    handle_tab(state);
                }
                F(2) => {
                    handle_f2(state);
                }
                Left => {
                    handle_left(state);
                }
                Right => {
                    handle_right(state);
                }
                Up => {
                    handle_up(state);
                }
                Down => {
                    handle_down(state);
                }
                Enter => {
                    if handle_enter(state) {
                        return Ok(true);
                    }
                }
                Char(char) => {
                    handle_char(state, char);
                }
                _ => {}
            }
        }
    }
    if let event::Event::Mouse(mouse_event) = event {
        match mouse_event.kind {
            event::MouseEventKind::ScrollDown => {
                handle_scroll_down(state, mouse_event);
            }
            event::MouseEventKind::ScrollUp => {
                handle_scroll_up(state, mouse_event);
            }
            event::MouseEventKind::Up(_) => {
                handle_mouse_up(state, mouse_event);
            }
            _ => {}
        }
    }

    Ok(false)
}

/// When user returns to P1 to select the input, output paths etc. we don't have any caching or whatever,
/// so this simply restarts the logic even if the same paths etc. are kept.
/// Therefore we need to clean the rest of the state as well when this happens.
pub fn clear_progress(state: &mut AppState) {
    state.selected_input_field = 1; 
    state.selected_missing_field = 1;
    state.selected_optional_field = 1;
    state.select_mapping_option = true;
    state.completed_missing_fields.clear();
    state.completed_optional_fields.clear();
    state.input_fields.clear();
    state.input_display_section.clear();
    state.input_field_pointer.clear();
    state.resolved_subsets.clear();
    state.missing_display_subset.clear();
    state.missing_field_pointer.clear();
    state.optional_display_subset.clear();
    state.optional_field_pointer.clear();    
}
