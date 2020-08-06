use crate::prelude::*;
pub use crossterm::event::{
    KeyCode,
    KeyModifiers,
    MouseButton,
};
use crossterm::event::{
    KeyEvent,
    MouseEvent,
};

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MouseEventKind {
    Drag,
    ButtonUp,
    ButtonDown,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InputEvent {
    MouseEvent {
        button: Option<MouseButton>,
        modifiers: KeyModifiers,
        column: u16,
        row: u16,
        kind: MouseEventKind,
    },
    Key {
        code: KeyCode,
        modifiers: KeyModifiers,
    },
}

impl From<KeyEvent> for InputEvent {
    fn from(source: KeyEvent) -> Self {
        let KeyEvent { code, modifiers } = source;
        Self::Key { code, modifiers }
    }
}

impl From<MouseEvent> for InputEvent {
    fn from(source: MouseEvent) -> Self {
        match source {
            MouseEvent::Down(button, column, row, modifiers) => Self::MouseEvent {
                button: Some(button),
                column,
                row,
                modifiers,
                kind: MouseEventKind::ButtonDown,
            },
            MouseEvent::Up(button, column, row, modifiers) => Self::MouseEvent {
                button: Some(button),
                column,
                row,
                modifiers,
                kind: MouseEventKind::ButtonUp,
            },
            MouseEvent::Drag(button, column, row, modifiers) => Self::MouseEvent {
                button: Some(button),
                column,
                row,
                modifiers,
                kind: MouseEventKind::Drag,
            },
            MouseEvent::ScrollUp(column, row, modifiers) => Self::MouseEvent {
                button: None,
                column,
                row,
                modifiers,
                kind: MouseEventKind::ScrollUp,
            },
            MouseEvent::ScrollDown(column, row, modifiers) => Self::MouseEvent {
                button: None,
                column,
                row,
                modifiers,
                kind: MouseEventKind::ScrollDown,
            },
        }
    }
}
