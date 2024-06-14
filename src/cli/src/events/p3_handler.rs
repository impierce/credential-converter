use crate::state::AppState;
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

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

    Ok(false)
}
