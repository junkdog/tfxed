use std::sync::mpsc::Sender;
use wasm_bindgen::prelude::wasm_bindgen;
use tfxed_core::{AppEvent::UpdateCanvas, AppEvent::CompileDsl, Dispatcher, AppEvent};

#[wasm_bindgen]
pub fn compile_dsl(s: &str) {
    sender().dispatch(CompileDsl(s.into()));
}

#[wasm_bindgen]
pub fn update_canvas(s: &str) {
    sender().dispatch(UpdateCanvas(s.into()));
}

fn sender() -> Sender<AppEvent> {
    unsafe {
        match SENDER.as_ref() {
            None    => panic!("No sender in global state"),
            Some(s) => s.event_sender.clone(),
        }
    }
}

struct JsSender {
    event_sender: Sender<AppEvent>,
}

pub fn init_global_state(sender: Sender<AppEvent>) {
    unsafe {
        SENDER = Some(JsSender { event_sender: sender });
    }
}

static mut SENDER: Option<JsSender> = None;