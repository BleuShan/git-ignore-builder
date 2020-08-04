use super::*;

use std::panic;

#[derive(Default)]
pub struct ApplicationBuilder {
    use_alternate_screen: bool,
    enable_raw_mode: bool,
}

impl ApplicationBuilder {
    pub fn use_alternate_screen(&mut self) -> &mut Self {
        self.use_alternate_screen = true;
        self
    }

    pub fn enable_raw_mode(&mut self) -> &mut Self {
        self.enable_raw_mode = true;
        self
    }

    pub fn build(&mut self) -> Result<Application> {
        let mut stdout = io::stdout();
        let mut exit_handler = ExitHandler::default();
        let ApplicationBuilder {
            use_alternate_screen,
            enable_raw_mode,
        } = *self;

        if enable_raw_mode {
            terminal::enable_raw_mode()?;
            exit_handler.enable_raw_mode();
        }

        if use_alternate_screen {
            stdout.execute(EnterAlternateScreen)?;
            exit_handler.use_alternate_screen();
        }

        let panic_handler = exit_handler.clone();
        panic::set_hook(Box::new(move |panic| {
            panic_handler.run();
            eprintln!("{}: {}", "Error".red(), panic);
        }));

        let renderer: Renderer = GenericRenderer::new(CrosstermBackend::new(stdout))?;

        Ok(Application::new(renderer, exit_handler))
    }
}
