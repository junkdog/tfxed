mod terminal;
mod event;
mod event_handler;
mod dispatcher;
mod tui;
mod app;

use ratatui::style::Stylize;
use std::io::Stdout;
use terminal::terminal;

use crate::app::App;
use crate::event_handler::EventHandler;
use crate::tui::{DefaultTui, Tui};
use color_eyre::eyre::{Result, WrapErr};
use ratatui::backend::CrosstermBackend;

fn init_tui() -> Result<Tui<CrosstermBackend<Stdout>>> {
    let terminal = terminal()?;
    let events = EventHandler::new(std::time::Duration::from_millis(33));
    Ok(DefaultTui::new(terminal, events))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut tui = init_tui()?;
    let mut app = App::new();

    while app.is_running() {
        tui.receive_events(|event| {
            app.apply_event(event);
        });

        tui.draw(|f| {
            app.update_time();
            app.render_ui(f);
            app.render_effects(f);
        })?;
    }

    ratatui::restore();

    Ok(())
}


