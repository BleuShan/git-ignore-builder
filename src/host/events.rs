pub mod input;

use crate::prelude::*;
use crossterm::event::Event as CrosstermEvent;
use input::InputEvent;

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Event {
    Draw { column: u16, row: u16 },
    Input(InputEvent),
}

impl From<CrosstermEvent> for Event {
    fn from(source: CrosstermEvent) -> Self {
        match source {
            CrosstermEvent::Resize(column, row) => Self::Draw { column, row },
            CrosstermEvent::Key(event) => Self::Input(event.into()),
            CrosstermEvent::Mouse(event) => Self::Input(event.into()),
        }
    }
}
