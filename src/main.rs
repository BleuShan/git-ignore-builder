#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

mod configuration;
mod constants;
mod prelude;
mod repository;
mod state;
mod ui;

use crate::prelude::*;
use async_std::future;
use configuration::Configuration;
use repository::Repository;
use state::State;
use ui::{
    widgets::*,
    Application,
    Event::*,
    Renderer,
};

async fn init(renderer: &mut Renderer, mut state: State) -> Result<State> {
    renderer.draw(|frame| {
        let size = frame.size();
        frame.render_widget(Paragraph::new("Loading Configuration"), size)
    })?;
    state.load_configuration().await?;
    renderer.draw(|frame| {
        let size = frame.size();
        frame.render_widget(Paragraph::new("Configuration Loaded"), size)
    })?;

    Ok(state)
}

async fn update(renderer: &mut Renderer, state: State) -> Result<State> {
    Ok(state)
}

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = Application::new()?;
    app.run_loop(|renderer, event, state| match event {
        Init => Box::pin(init(renderer, state)),
        Update => Box::pin(update(renderer, state)),
        _ => Box::pin(future::ready(Ok(state))),
    })
    .await
}
