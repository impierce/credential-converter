use crate::utils::AppState;
use crossterm::event::{self, KeyCode::*};

pub fn p2_handler(key: event::KeyEvent, state: &mut AppState) -> Result<bool, std::io::Error> {
    match key.code {
        Esc => return Ok(true),
        Tab => {
            state.tab.next();
        }
        F(2) => {
            state.tab.prev();
        }
        Left => {state.transformation.prev();}
        Right => {state.transformation.next();}
        Up => {
            if state.selected_input_field > 1 {
                state.selected_input_field -= 1;
            }
        }
        Down => {
            if state.selected_input_field <= state.amount_input_fields {
                state.selected_input_field += 1;
            }
        }
        Enter => {
            state.map_input_field = !state.map_input_field;
        }
        _ => {}
    }

    Ok(false)
}
