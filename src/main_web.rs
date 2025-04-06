mod app;
mod terminal;
mod event;
mod event_handler;
mod dispatcher;
mod tui;

use std::collections::HashMap;
use base64::{alphabet, Engine};
use base64::engine::{general_purpose, GeneralPurpose};
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

    let events = EventHandler::new();
    let key_event_sender = events.sender();

    let mut terminal = terminal()?;
    terminal.on_key_event(move |e| {
        if !e.alt && !e.ctrl {
            key_event_sender.dispatch(AppEvent::KeyPress(e.into()));
        }
    });

    let mut app = App::new(events.sender());

    let mut last_search_url = String::new();
    let mut last_buffer = String::new();

    terminal.draw_web(move |f| {
        events.receive_events(|event| {
            app.apply_event(event);
        });

        let search_url = web_sys::window()
            .unwrap()
            .location()
            .search()
            .unwrap();

        if last_search_url != search_url {
            last_search_url = search_url.clone();
            let query_map = parse_query_params(&search_url);

            let b64decoder = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

            if let Some(code) = query_map.get("code") {
                if let Ok(decoded) = b64decoder.decode(code) {
                    let code_str = String::from_utf8_lossy(&decoded);
                    web_sys::console::log_1(&format!("Decoded code:\n{}", code_str).into());
                    app.sender().dispatch(AppEvent::CompileDsl(code_str.to_string()));
                }
            }

            if let Some(buffer) = query_map.get("buffer") {
                if let Ok(decoded) = b64decoder.decode(buffer) {
                    let buffer_str = String::from_utf8_lossy(&decoded);
                    web_sys::console::log_1(&format!("Decoded buffer:\n{}", buffer_str).into());
                    if buffer_str != last_buffer {
                        last_buffer = buffer_str.to_string();
                        app.sender().dispatch(AppEvent::UpdateCanvas(buffer_str.to_string()));
                    }
                }
            }

            if let Some(last_update) = query_map.get("last_update") {
                web_sys::console::log_1(&format!("Last update: {}", last_update).into());
            }
        }

        app.update_time();
        app.render_ui(f);
        app.render_effects(f);
    });

    Ok(())
}

fn parse_query_params(query: &str) -> HashMap<String, String> {
    query
        .trim_start_matches('?')
        .split('&')
        .filter_map(|pair| {
            let mut split = pair.splitn(2, '=');
            let key = split.next()?;
            let val = split.next().unwrap_or("");
            Some((
                percent_decode(key).unwrap_or_default(),
                percent_decode(val).unwrap_or_default(),
            ))
        })
        .collect()
}

fn percent_decode(s: &str) -> Option<String> {
    percent_encoding::percent_decode_str(s)
        .decode_utf8()
        .ok()
        .map(|s| s.to_string())
}