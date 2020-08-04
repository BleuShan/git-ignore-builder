use super::*;

use super::terminal::{
    EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::event::EventStream;
use std::{
    io,
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    Terminal as GenericRenderer,
};
use InputEvent::*;

pub type Renderer = GenericRenderer<CrosstermBackend<io::Stdout>>;

static SHOULD_INIT_STDOUT: AtomicBool = AtomicBool::new(true);

pub enum Event {
    Init,
    Update,
    Input(InputEvent),
}

pub struct Application {
    renderer: Renderer,
    events: EventStream,
}

impl Application {
    pub fn new() -> Result<Self> {
        let mut stdout = io::stdout();

        if SHOULD_INIT_STDOUT.fetch_and(false, Ordering::SeqCst) {
            terminal::enable_raw_mode()?;
            stdout.execute(EnterAlternateScreen)?;
        }

        let renderer = Renderer::new(CrosstermBackend::new(stdout))?;
        Ok(Self {
            renderer,
            events: EventStream::default(),
        })
    }

    pub async fn run_loop<'state, F, S>(&mut self, f: F) -> Result<()>
    where
        F: Fn(&mut Renderer, Event, S) -> Pin<Box<dyn Future<Output = Result<S>> + Send + '_>>,
        S: Default + Sized,
    {
        let mut state = S::default();
        state = f(&mut self.renderer, Event::Init, state).await?;

        loop {
            state = match self.events.next().await {
                Some(result) => {
                    let event = result?;
                    match event {
                        Key(KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                        }) => break,
                        _ => f(&mut self.renderer, Event::Input(event), state).await?,
                    }
                }
                None => f(&mut self.renderer, Event::Update, state).await?,
            };

            task::sleep(Duration::from_millis(16)).await;
        }

        Ok(())
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        if !SHOULD_INIT_STDOUT.fetch_or(true, Ordering::SeqCst) {
            io::stdout()
                .execute(LeaveAlternateScreen)
                .expect("Failed to leave alternate screen.");
            terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        }
    }
}
