#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

mod cli;
mod constants;
mod prelude;
mod repository;
mod ui;

use crate::prelude::*;
use cli::*;
use repository::Repository;
use std::time::Duration;
use ui::{
    widgets::*,
    EventLoop,
    Window,
};

#[async_std::main]
async fn main() -> Result<()> {
    let mut window = Window::new()?;
    window.clear()?;
    let mut event_loop = EventLoop::default();
    window.draw(|frame| {
        let size = frame.size();
        frame.render_widget(Paragraph::new("Loading configuration"), size);
    })?;
    let config = Configuration::load().await?;

    if let Some(input) = event_loop.run(move |event| Ok(Some(event))).await? {
        window.draw(|frame| {
            let size = frame.size();
            frame.render_widget(Paragraph::new(format!("{:?}", input).as_str()), size);
        })?;
        task::sleep(Duration::from_millis(200)).await
    }

    let mut _repo = Repository::new(&config);
    config.save_if_needed().await
}
