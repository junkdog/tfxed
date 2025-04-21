mod event_handler;
mod interop;

use std::collections::HashMap;
use std::error::Error;
use ratatui::style::Stylize;

use crate::event_handler::{convert_key_event, EventHandler};
use crate::interop::init_global_state;
use console_error_panic_hook::set_once as set_panic_hook;
use eyre::{eyre, Result, WrapErr};
use miniz_oxide::inflate::decompress_to_vec;
use ratatui::Terminal as RatTerminal;
use ratzilla::{DomBackend, WebRenderer};
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

fn parse_query_params(query: &str) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for pair in query.trim_start_matches('?').split('&') {
        let mut split = pair.splitn(2, '=');
        let key_enc = split.next().unwrap_or("");
        let val_enc = split.next().unwrap_or("");

        let key_bytes = percent_decode(key_enc)?;
        let val_bytes = percent_decode(val_enc)?;

        let key = String::from_utf8(key_bytes)?;
        let value = String::from_utf8(val_bytes)?;
        map.insert(key, value);
    }

    Ok(map)
}

/// decompresses raw DEFLATE-compressed bytes into a UTF-8 string
pub fn decompress(compressed: Vec<u8>) -> Result<String> {
    let bytes = decompress_to_vec(&compressed)
        .map_err(|e| eyre!("Failed to decompress buffer: {}", e))?;

    String::from_utf8(bytes)
        .map_err(|_| eyre!("Invalid UTF-8 in decompressed data"))
}

fn percent_decode(input: &str) -> Result<Vec<u8>> {
    Ok(percent_encoding::percent_decode(input.as_bytes()).collect())
}

fn terminal() -> Result<RatTerminal<DomBackend>> {
    let backend = DomBackend::new_by_id("content")
        .map_err(|e| eyre!("{e}"))?;

    RatTerminal::new(backend)
        .wrap_err("failed to initialize terminal")
}