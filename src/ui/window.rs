use super::*;

use super::terminal::{
    EnterAlternateScreen,
    LeaveAlternateScreen,
};

use std::sync::atomic::{
    AtomicBool,
    Ordering,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

type StdoutTerminal = Terminal<CrosstermBackend<io::Stdout>>;

static SHOULD_INIT_STDOUT: AtomicBool = AtomicBool::new(true);

#[derive(Deref, DerefMut)]
pub struct Window {
    #[deref]
    #[deref_mut]
    inner: StdoutTerminal,
}

impl Window {
    pub fn new() -> Result<Self> {
        let mut stdout = io::stdout();

        if SHOULD_INIT_STDOUT.fetch_and(false, Ordering::SeqCst) {
            terminal::enable_raw_mode()?;
            stdout.execute(EnterAlternateScreen)?;
        }

        let inner = Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(Self { inner })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if !SHOULD_INIT_STDOUT.fetch_or(true, Ordering::SeqCst) {
            io::stdout()
                .execute(LeaveAlternateScreen)
                .expect("Failed to leave alternate screen.");
            terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        }
    }
}
