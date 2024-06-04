use crate::utils::{AppState};
use crossterm::event::{self, KeyCode::*};

pub fn p3_handler(key: event::KeyEvent, state: &mut AppState) -> Result<bool, std::io::Error> {
    match key.code {
        Esc => return Ok(true),
        Tab => {
            state.tab.next();
        }
        F(2) => {
            state.tab.prev();
        }
        _ => {}
    }

    Ok(false)
}
