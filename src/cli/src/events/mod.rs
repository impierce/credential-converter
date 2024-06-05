use crate::utils::{AppState, Tabs};
use crossterm::event::{self, KeyEventKind};

pub mod p1_handler;
pub mod p2_handler;
pub mod p3_handler;

pub use p1_handler::p1_handler;
pub use p2_handler::p2_handler;
pub use p3_handler::p3_handler;
use ratatui::layout::Rect;

pub fn events_handler(state: &mut AppState) -> Result<bool, std::io::Error> {
    let mut quit = false;

    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match state.tab {
                    Tabs::InputPromptsP1 => {
                        quit = p1_handler(key, state)?;
                    }
                    Tabs::ManualMappingP2 => {
                        quit = p2_handler(key, state)?;
                    }
                    Tabs::UnusedDataP3 => {
                        quit = p3_handler(key, state)?;
                    }
                }
            }
        }
        if let event::Event::Mouse(mouse_event) = event::read()? {
            match mouse_event.kind {
                event::MouseEventKind::Moved => {
                    state.hover_scroll = is_mouse_over_area(mouse_event.column, mouse_event.row, state.area);
                }
                _ => {}
            }
        }
    }

    Ok(quit)
}

fn is_mouse_over_area(mouse_x: u16, mouse_y: u16, area: ratatui::layout::Rect) -> bool {
    mouse_x >= area.x && mouse_x < area.x + area.width &&
    mouse_y >= area.y && mouse_y < area.y + area.height
}
