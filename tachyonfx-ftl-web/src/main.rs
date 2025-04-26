mod event_handler;
mod interop;

use std::collections::HashMap;
use std::error::Error;
use ratatui::style::Stylize;

use crate::event_handler::{convert_key_event, EventHandler};
use crate::interop::init_global_state;
use console_error_panic_hook::set_once as set_panic_hook;
use eyre::{eyre, Result, WrapErr};
use ratatui::Terminal as RatTerminal;
use ratzilla::{CanvasBackend, DomBackend, WebRenderer};
use ratzilla::backend::canvas::CanvasBackendOptions;
use tfxed_core::{App, AppEvent, Dispatcher};


fn main() -> Result<()> {
    set_panic_hook();
    let events = EventHandler::new();
    let sender = events.sender();

    // globally set the sender for the JS interop functions
    init_global_state(sender.clone());

    let mut terminal = terminal()?;
    terminal.on_key_event(move |e| {
        if !e.alt && !e.ctrl {
            sender.dispatch(AppEvent::KeyPress(convert_key_event(e)));
        }
    });

    let mut app = App::new(events.sender());

    terminal.draw_web(move |f| {
        events.receive_events(|event| {
            app.apply_event(event);
        });


        app.update_time();
        app.render_ui(f);
    });

    Ok(())
}


fn terminal() -> Result<RatTerminal<CanvasBackend>> {
    let backend = CanvasBackend::new_with_options(CanvasBackendOptions::new().grid_id("content"))
        .map_err(|e| eyre!("{e}"))?;

    RatTerminal::new(backend)
        .wrap_err("failed to initialize terminal")
}