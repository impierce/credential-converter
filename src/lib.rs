pub mod backend;
pub mod events;
pub mod render;
pub mod state;

// Load I18n macro, to allow use of the `t!` macro in anywhere.
use rust_i18n::i18n;
i18n!();
