mod app;
mod terminal;
mod event;
mod event_handler;
mod dispatcher;
mod tui;

use terminal::terminal;

use ratatui::style::Stylize;

use crate::app::App;
use crate::dispatcher::Dispatcher;
use crate::event::AppEvent;
use crate::event_handler::EventHandler;
use color_eyre::eyre::{Result, WrapErr};
use ratzilla::WebRenderer;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = terminal()?;
    let mut events = EventHandler::new();

    let key_event_sender = events.sender();
    terminal.on_key_event(move |e| {
        if !e.alt && !e.ctrl {
            key_event_sender.dispatch(AppEvent::KeyPress(e.into()));
        }
    });

    let mut app = App::new();;

    terminal.draw_web(move |f| {
        events.receive_events(|event| {
            app.apply_event(event);
        });

        app.render(f)
    });

    Ok(())
}
