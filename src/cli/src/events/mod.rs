use crate::state::{AppState, Tabs};
use crossterm::event::{self};

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
        let event = event::read()?;
        match state.tab {
            Tabs::InputPromptsP1 => {
                quit = p1_handler(event, state)?;
            }
            Tabs::ManualMappingP2 => {
                quit = p2_handler(event, state)?;
            }
            Tabs::UnusedDataP3 => {
                quit = p3_handler(event, state)?;
            }
        }
    }

    Ok(quit)
}

//////////     HELPERS     //////////

fn is_mouse_over_area(area: Rect, mouse_x: u16, mouse_y: u16) -> bool {
    mouse_x >= area.x && mouse_x < area.x + area.width && mouse_y >= area.y && mouse_y < area.y + area.height
}
