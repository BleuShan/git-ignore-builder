pub mod widgets;
mod window;

pub use tui::{
    layout,
    style::{
        Color,
        Modifier,
        Style,
    },
    text::{
        Span,
        Spans,
        Text,
    },
};

pub use window::{
    Builder as WindowBuilder,
    Window,
};
