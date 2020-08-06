#![forbid(future_incompatible)]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(try_blocks, debug_non_exhaustive)]

mod cli;
mod host;
mod prelude;
mod ui;

use crate::prelude::*;
use async_std::stream;
use git_ignore_builder_core::config;
use std::time::Duration;
use ui::{
    widgets::{
        Block,
        BorderType,
        Borders,
        Paragraph,
        Wrap,
    },
    Color,
    Span,
    Spans,
    Style,
    Window,
};

const TARGET_FRAME_DURATION: Duration = Duration::from_millis(1000 / 60);

#[async_std::main]
async fn main() -> Result<()> {
    let config = config::Config::default();
    let mut window = Window::builder()
        .use_alternate_screen()
        .enable_raw_mode()
        .build()?;

    let mut tick = stream::interval(TARGET_FRAME_DURATION)
        .enumerate()
        .map(|(count, _)| count)
        .take_while(|count| *count != 3 * 50)
        .fuse();

    while let Some(count) = tick.next().await {
        window.draw(|frame| {
            let frame_rect = frame.size();
            let block = Block::default()
                .title(env!("CARGO_PKG_NAME"))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let text = vec![Spans::from(vec![
                Span::styled("Count: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}", count / 60)),
            ])];
            let p = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
            frame.render_widget(p, frame_rect)
        })?;
    }

    Ok(())
}
