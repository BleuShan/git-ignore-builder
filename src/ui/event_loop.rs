use super::{
    Event::*,
    *,
};
use crossterm::event::EventStream;

#[derive(Default)]
pub struct EventLoop {
    events: EventStream,
}

impl EventLoop {
    pub async fn run<F, R>(&mut self, f: F) -> Result<Option<R>>
    where
        F: Fn(Event) -> Result<Option<R>> + 'static,
    {
        while let Some(result) = self.events.next().await {
            match result {
                Ok(event) => match event {
                    Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                    }) => return Ok(None),

                    arg => match f(arg) {
                        Ok(value) if value.is_some() => return Ok(value),
                        Err(error) => return Err(error),
                        _ => {}
                    },
                },
                Err(error) => return Err(anyhow!("{}", error)),
            };
        }

        Ok(None)
    }
}
