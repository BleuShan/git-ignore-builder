use crate::prelude::*;
pub use crossterm::{
    event::{
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
        MouseButton,
        MouseEvent,
    },
    terminal,
    ExecutableCommand,
};
use std::io::{
    self,
    prelude::*,
};

pub use tui::{
    layout,
    style::{
        Color,
        Style,
    },
    text::{
        self,
        Span,
        Spans,
    },
    widgets::{
        self,
        StatefulWidget,
        Widget,
    },
};

mod event_loop;
mod window;

pub use event_loop::EventLoop;
pub use window::Window;
