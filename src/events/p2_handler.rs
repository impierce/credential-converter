use crate::{p2_p3_common::{handle_backspace, handle_char, handle_down, handle_enter, handle_esc, handle_f2, handle_left, handle_mouse_up, handle_right, handle_scroll_down, handle_scroll_up, handle_tab, handle_up}, state::AppState};

use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if handle_esc(state) {
                        return Ok(true);
                    }
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
                    handle_enter(state);
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
