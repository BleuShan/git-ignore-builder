use super::{
    prelude::*,
    Window,
    WindowFlags,
};

#[derive(Default, Debug)]
pub struct Builder {
    flags: BitFlags<WindowFlags>,
}

impl Builder {
    pub fn use_alternate_screen(mut self) -> Self {
        self.flags |= WindowFlags::UsesAlternateScreen;
        self
    }

    pub fn enable_raw_mode(mut self) -> Self {
        self.flags |= WindowFlags::RawModeEnabled;
        self
    }

    pub fn build(&mut self) -> Result<Window> {
        let flags = self.flags;
        let mut stdout = stdio::stdout();

        if flags.contains(WindowFlags::RawModeEnabled) {
            terminal::enable_raw_mode()?;
        }

        if flags.contains(WindowFlags::UsesAlternateScreen) {
            stdout.queue(EnterAlternateScreen)?;
        }

        stdout.queue(Clear(ClearType::All))?;

        panic::set_hook(Box::new(move |panic| {
            exit_hook(flags);
            eprintln!("{}: {}", "Error".red(), panic);
        }));

        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;

        Ok(Window::new(terminal, flags))
    }
}
