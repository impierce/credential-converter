use crate::state::AppState;
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

use super::is_mouse_over_area;

pub fn p3_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
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
        }
    }
    if let event::Event::Mouse(mouse_event) = event {
        match mouse_event.kind {
            event::MouseEventKind::Up(_) => {
                if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
                    state.tab.prev();
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
