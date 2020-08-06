mod builder;
pub(self) mod prelude;

use self::prelude::*;
pub use builder::Builder;

pub struct Window {
    terminal: StdoutTerminal,
    flags: BitFlags<WindowFlags>,
}

impl Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("flags", &self.flags)
            .finish_non_exhaustive()
    }
}

impl Window {
    fn new(terminal: StdoutTerminal, flags: BitFlags<WindowFlags>) -> Self {
        Self { terminal, flags }
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut StdoutFrame<'_>),
    {
        self.terminal.draw(f).map_err(|e| e.into())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        exit_hook(self.flags)
    }
}
