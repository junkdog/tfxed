mod event_handler;
mod interop;

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use base64::{alphabet, Engine};
use base64::engine::{general_purpose, GeneralPurpose};
use color_eyre::eyre;

use ratatui::style::Stylize;

use crate::event_handler::{convert_key_event, EventHandler};
use color_eyre::eyre::{eyre, Result, WrapErr};
use miniz_oxide::inflate::decompress_to_vec;
use ratatui::Terminal as RatTerminal;
use ratzilla::{DomBackend, WebRenderer};
use wasm_bindgen::prelude::wasm_bindgen;
use tfxed_core::{App, AppEvent, Dispatcher};
use crate::interop::set_js_sender;

fn main() -> Result<()> {
    color_eyre::install()?;

    let events = EventHandler::new();
    let sender = events.sender();

    // globally set the sender for the JS interop functions
    set_js_sender(sender.clone());

    let mut terminal = terminal()?;
    terminal.on_key_event(move |e| {
        if !e.alt && !e.ctrl {
            sender.dispatch(AppEvent::KeyPress(convert_key_event(e)));
        }
    });

    let mut app = App::new(events.sender());

    let mut last_search_url = String::new();
    let mut last_buffer = String::new();
    let mut last_update = String::new();
    let mut last_code = String::new();

    terminal.draw_web(move |f| {
        events.receive_events(|event| {
            app.apply_event(event);
        });

        // let search_url = web_sys::window()
        //     .unwrap()
        //     .location()
        //     .search()
        //     .unwrap();
        //
        // if last_search_url != search_url {
        //     last_search_url = search_url.clone();
        //     let query_map = parse_query_params(&search_url);
        //     let query_map = if let Ok(query_map) = query_map {
        //         web_sys::console::log_1(&format!("Parsed query params").into());
        //         query_map
        //     } else {
        //         web_sys::console::log_1(&format!("Failed to parse query params").into());
        //         HashMap::new()
        //     };
        //
        //     let b64decoder = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        //
        //     if let Some(code_b64) = query_map.get("code") {
        //         if let Ok(decoded) = b64decoder.decode(code_b64) {
        //             if let Ok(code_str) = decompress(decoded) {
        //                 web_sys::console::log_1(&format!("Decoded code:\n{}", code_str).into());
        //                 app.sender().dispatch(AppEvent::CompileDsl(code_str));
        //             }
        //         }
        //     }
        //
        //     if let Some(buffer_b64) = query_map.get("buffer") {
        //         if let Ok(decoded) = b64decoder.decode(buffer_b64) {
        //             if let Ok(buffer_str) = decompress(decoded) {
        //                 web_sys::console::log_1(&format!("Decoded buffer:\n{}", buffer_str).into());
        //                 if buffer_str != last_buffer {
        //                     last_buffer = buffer_str.clone();
        //                     app.sender().dispatch(AppEvent::UpdateCanvas(buffer_str));
        //                 }
        //             } else {
        //                 web_sys::console::log_1(&format!("Failed to decode buffer :(").into());
        //             }
        //         }
        //     }
        //
        //     if let Some(time) = query_map.get("last_update") {
        //         last_update = time.clone();
        //         web_sys::console::log_1(&format!("Last update: {}", last_update).into());
        //     }
        // }

        app.update_time();
        app.render_ui(f);
        app.render_effects(f);
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