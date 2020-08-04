use super::*;
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

static DID_RUN: AtomicBool = AtomicBool::new(false);

#[derive(Default, Clone)]
pub struct ExitHandler {
    use_alternate_screen: bool,
    enable_raw_mode: bool,
}

impl ExitHandler {
    pub fn use_alternate_screen(&mut self) -> &mut Self {
        self.use_alternate_screen = true;
        self
    }

    pub fn enable_raw_mode(&mut self) -> &mut Self {
        self.enable_raw_mode = true;
        self
    }

    pub fn run(&self) {
        if !DID_RUN.fetch_or(true, Ordering::SeqCst) {
            if self.use_alternate_screen {
                let _ = io::stdout().execute(LeaveAlternateScreen);
            }

            if self.enable_raw_mode {
                let _ = terminal::disable_raw_mode();
            }
        }
    }
}
