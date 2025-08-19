mod status;
mod throbbing;

pub use status::Status;
pub use throbbing::Throbbing;

use std::sync::{LazyLock, Mutex};

use ratatui::DefaultTerminal;

pub static TERMINAL: LazyLock<Mutex<DefaultTerminal>> = LazyLock::new(|| ratatui::init().into());
