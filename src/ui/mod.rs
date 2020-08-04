use crate::prelude::*;
pub use crossterm::{
    event::{
        Event as InputEvent,
        KeyCode,
        KeyEvent,
        KeyModifiers,
        MouseButton,
        MouseEvent,
    },
    style::{
        Colorize,
        Styler,
    },
    terminal,
    ExecutableCommand,
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

mod application;

pub use application::{
    Application,
    Event,
    Renderer,
};
