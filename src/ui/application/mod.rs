mod builder;
mod exit_handler;

use super::*;
pub use builder::ApplicationBuilder;
pub use exit_handler::ExitHandler;

use crossterm::event::EventStream;
use std::{
    io,
    time::Duration,
};
use terminal::{
    self,
    EnterAlternateScreen,
    LeaveAlternateScreen,
};
use tui::{
    backend::CrosstermBackend,
    Terminal as GenericRenderer,
};
use InputEvent::*;

pub type Renderer = GenericRenderer<CrosstermBackend<io::Stdout>>;

pub enum Event {
    Init,
    Update,
    Input(InputEvent),
    Exit,
}

pub struct Application {
    events: EventStream,
    exit_handler: ExitHandler,
    renderer: Renderer,
}

impl Application {
    fn new(renderer: Renderer, exit_handler: ExitHandler) -> Self {
        Application {
            events: EventStream::default(),
            exit_handler,
            renderer,
        }
    }
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub async fn run_loop<F, S>(&mut self, f: F) -> Result<()>
    where
        F: Fn(&mut Renderer, Event, S) -> Pin<Box<dyn Future<Output = Result<S>> + '_>>,
        S: Default + Sized,
    {
        let mut state = S::default();
        state = f(&mut self.renderer, Event::Init, state).await?;
        let mut should_exit = false;

        loop {
            state = match self.events.next().await {
                Some(result) => {
                    let event = result?;
                    match event {
                        Key(KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                        }) => {
                            should_exit = true;
                            f(&mut self.renderer, Event::Exit, state).await?
                        }
                        _ => f(&mut self.renderer, Event::Input(event), state).await?,
                    }
                }
                None => f(&mut self.renderer, Event::Update, state).await?,
            };

            task::sleep(Duration::from_millis(16)).await;
            if should_exit {
                break;
            }
        }

        Ok(())
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        self.exit_handler.run()
    }
}
