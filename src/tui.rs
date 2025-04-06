use std::io;


use crate::event_handler::EventHandler;
use crate::event::AppEvent;
use ratatui::layout::Size;
use ratatui::{Frame, Terminal};
use ratatui::prelude::Backend;

#[cfg(feature = "web-backend")]
use ratzilla::WebRenderer;

#[cfg(feature = "crossterm-backend")]
pub type DefaultTui = Tui<ratatui::backend::CrosstermBackend<io::Stdout>>;

#[cfg(feature = "web-backend")]
pub type DefaultTui = Tui<ratzilla::DomBackend>;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui<BACKEND : Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<BACKEND>,
    /// Terminal event handler.
    events: EventHandler,
}

impl<BACKEND: Backend + 'static> Tui<BACKEND> {
    pub fn new(terminal: Terminal<BACKEND>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    #[cfg(feature = "crossterm-backend")]
    pub fn draw(
        &mut self,
        render_ui: impl FnMut(&mut Frame),
    ) -> io::Result<()> {
        self.terminal.draw(render_ui)?;
        Ok(())
    }

    #[cfg(feature = "web-backend")]
    pub fn draw(
        self,
        render_ui: impl FnMut(&mut Frame) + 'static,
    ) {
        self.terminal.draw_web(render_ui);
    }

    pub fn size(&self) -> Size {
        self.terminal.size().unwrap()
    }

    /// iterates over all currently available events; waits
    /// until at least one event is available.
    pub fn receive_events<F>(&self, mut f: F)
        where F: FnMut(AppEvent)
    {
        f(self.events.next().unwrap());
        while let Some(event) = self.events.try_next() { f(event) }
    }
}