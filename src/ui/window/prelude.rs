pub(super) use crate::prelude::*;

pub(super) use crossterm::{
    style::Colorize,
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};
pub(super) use std::{
    io::{
        self as stdio,
        Stderr,
        Stdout,
        Write,
    },
    panic,
};
pub(super) use tui::{
    backend::CrosstermBackend,
    Frame,
    Terminal,
};

pub(super) type StdoutBackend = CrosstermBackend<Stdout>;
pub(super) type StdoutTerminal = Terminal<StdoutBackend>;
pub(super) type StdoutFrame<'a> = Frame<'a, StdoutBackend>;
pub(super) type StderrBackend = CrosstermBackend<Stderr>;
pub(super) type StderrTerminal = Terminal<StderrBackend>;
pub(super) type StderrFrame<'a> = Frame<'a, StderrBackend>;

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub(super) enum WindowFlags {
    UsesAlternateScreen = 1 << 0,
    MouseCaptured = 1 << 1,
    RawModeEnabled = 1 << 2,
}

pub(super) fn exit_hook(flags: BitFlags<WindowFlags>) {
    let mut stdout = stdio::stdout();

    let result: Result<()> = try {
        if flags.contains(WindowFlags::RawModeEnabled) {
            terminal::disable_raw_mode()?;
        }

        if flags.contains(WindowFlags::UsesAlternateScreen) {
            stdout.queue(LeaveAlternateScreen)?;
        }

        stdout.flush()?;
    };

    if let Err(error) = result {
        eprintln!("{}: {}", "Error".red(), error)
    }
}
