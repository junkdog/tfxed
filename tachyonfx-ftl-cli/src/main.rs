mod event_handler;
mod tui;

use ratatui::style::Stylize;
use std::io::Stdout;
use ansi_to_tui::IntoText;

use crate::event_handler::EventHandler;
use crate::tui::Tui;
use color_eyre::eyre::{Result, WrapErr};
use ratatui::backend::CrosstermBackend;
use tfxed_core::{App, AppEvent, Dispatcher};

fn init_tui() -> Result<Tui<CrosstermBackend<Stdout>>> {
    let terminal = terminal()?;
    let events = EventHandler::new(std::time::Duration::from_millis(33));
    Ok(Tui::new(terminal, events))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input_file = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <input_file>", std::env::args().next().unwrap());
        std::process::exit(1);
    });

    let input = std::fs::read_to_string(input_file)?;

    let mut tui = init_tui()?;
    let mut app = App::new(tui.sender());
    app.sender().dispatch(AppEvent::UpdateCanvas(input));

    while app.is_running() {
        tui.receive_events(|event| {
            app.apply_event(event);
        });

        tui.draw(|f| {
            app.update_time();
            app.render_ui(f);
        })?;
    }

    ratatui::restore();

    Ok(())
}

fn terminal() -> Result<ratatui::DefaultTerminal> {
    Ok(ratatui::try_init()
        .wrap_err("failed to initialize terminal")?)
}

