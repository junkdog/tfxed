use std::sync::mpsc;
use ratzilla::event::KeyEvent as RatzillaKeyEvent;
use tfxed_core::{AppEvent, KeyCode, KeyEvent, ModifierKeys};

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,
}


impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    pub fn sender(&self) -> mpsc::Sender<AppEvent> {
        self.sender.clone()
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_next(&self) -> Option<AppEvent> {
        match self.receiver.try_recv() {
            Ok(e)  => Some(e),
            Err(_) => None
        }
    }

    /// iterates over all currently available events
    pub fn receive_events<F>(&self, mut f: F)
        where F: FnMut(AppEvent)
    {
        // f(self.next().unwrap());
        while let Some(event) = self.try_next() { f(event) }
    }
}

pub fn convert_key_event(
    RatzillaKeyEvent { code, ctrl, alt, shift }: RatzillaKeyEvent
) -> KeyEvent {
    let mut modifier_keys = ModifierKeys::empty();
    if ctrl  { modifier_keys |= ModifierKeys::CONTROL; }
    if alt   { modifier_keys |= ModifierKeys::ALT; }
    if shift { modifier_keys |= ModifierKeys::SHIFT; }

    use ratzilla::event::KeyCode as RzKeyCode;
    KeyEvent {
        key_code: match code {
            RzKeyCode::Char(c) => KeyCode::Char(c),
            RzKeyCode::F(n) => KeyCode::F(n),
            RzKeyCode::Backspace => KeyCode::Backspace,
            RzKeyCode::Enter => KeyCode::Enter,
            RzKeyCode::Left => KeyCode::Left,
            RzKeyCode::Right => KeyCode::Right,
            RzKeyCode::Up => KeyCode::Up,
            RzKeyCode::Down => KeyCode::Down,
            RzKeyCode::Tab => KeyCode::Tab,
            RzKeyCode::Delete => KeyCode::Delete,
            RzKeyCode::Home => KeyCode::Home,
            RzKeyCode::End => KeyCode::End,
            RzKeyCode::PageUp => KeyCode::PageUp,
            RzKeyCode::PageDown => KeyCode::PageDown,
            RzKeyCode::Esc => KeyCode::Esc,
            RzKeyCode::Unidentified => KeyCode::Char(' '),
        },
        modifier_keys
    }
}