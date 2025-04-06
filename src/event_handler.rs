use std::sync::mpsc;
use crate::event::{AppEvent, KeyCode, ModifierKeys};
use crate::dispatcher::Dispatcher;

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,

    #[cfg(feature = "crossterm-backend")]
    _handler: std::thread::JoinHandle<()>
}


impl EventHandler {
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


#[cfg(feature = "crossterm-backend")]
impl EventHandler {

    pub fn new(tick_rate: std::time::Duration) -> Self {
        use ratatui::crossterm::event;

        let (sender, receiver) = mpsc::channel();

        let handler = {
            let sender = sender.clone();
            std::thread::spawn(move || {
                let mut last_tick = std::time::Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("successfully polled for events") {
                        Self::consume_event(&sender);
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.dispatch(AppEvent::Tick);
                        last_tick = std::time::Instant::now();
                    }
                }
            })
        };

        Self { sender, receiver, _handler: handler }
    }

    fn consume_event(sender: &mpsc::Sender<AppEvent>) {
        use ratatui::crossterm::event::KeyEventKind;
        use crossterm::{event::Event as CrosstermEvent};

        match crossterm::event::read().expect("event is read") {
            CrosstermEvent::Key(e) if e.kind == KeyEventKind::Press =>
                sender.send(AppEvent::KeyPress(e.into())),
            CrosstermEvent::Resize(w, h) =>
                sender.send(AppEvent::Resize(w, h)),

            _ => Ok(())
        }.expect("event should have been sent");
    }
}

#[cfg(feature = "crossterm-backend")]
impl From<crossterm::event::KeyEvent> for crate::event::KeyEvent {

    fn from(crossterm::event::KeyEvent { code, modifiers, .. }: crossterm::event::KeyEvent) -> Self {
        use crossterm::event::{KeyCode as CtKeyCode, KeyModifiers};
        let mut modifier_keys = ModifierKeys::empty();
        if modifiers.contains(KeyModifiers::SHIFT) {
            modifier_keys |= ModifierKeys::SHIFT;
        }
        if modifiers.contains(KeyModifiers::CONTROL) {
            modifier_keys |= ModifierKeys::CONTROL;
        }
        if modifiers.contains(KeyModifiers::ALT) {
            modifier_keys |= ModifierKeys::ALT;
        }
        if modifiers.contains(KeyModifiers::META) {
            modifier_keys |= ModifierKeys::META;
        }
        if modifiers.contains(KeyModifiers::SUPER) {
            modifier_keys |= ModifierKeys::SUPER;
        }

        let key_code = match code {
            CtKeyCode::Backspace => KeyCode::Backspace,
            CtKeyCode::Enter     =>  KeyCode::Enter,
            CtKeyCode::Left      => KeyCode::Left,
            CtKeyCode::Right     => KeyCode::Right,
            CtKeyCode::Up        => KeyCode::Up ,
            CtKeyCode::Down      => KeyCode::Down,
            CtKeyCode::Home      => KeyCode::Home,
            CtKeyCode::End       => KeyCode::End,
            CtKeyCode::PageUp    => KeyCode::PageUp,
            CtKeyCode::PageDown  => KeyCode::PageDown,
            CtKeyCode::Tab       => KeyCode::Tab,
            CtKeyCode::BackTab   => KeyCode::BackTab,
            CtKeyCode::Delete    => KeyCode::Delete,
            CtKeyCode::Insert    => KeyCode::Insert,
            CtKeyCode::F(n)      => KeyCode::F(n),
            CtKeyCode::Char(c)   => KeyCode::Char(c),
            CtKeyCode::Esc       => KeyCode::Esc,
            _                    => KeyCode::Char(' '),
        };

        Self {
            key_code,
            modifier_keys
        }
    }
}

#[cfg(feature = "web-backend")]
impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }
}

#[cfg(feature = "web-backend")]
impl From<ratzilla::event::KeyEvent> for crate::event::KeyEvent {
    fn from(ratzilla::event::KeyEvent { code, ctrl, alt, shift }: ratzilla::event::KeyEvent) -> Self {
        let mut modifier_keys = ModifierKeys::empty();
        if ctrl  { modifier_keys |= ModifierKeys::CONTROL; }
        if alt   { modifier_keys |= ModifierKeys::ALT; }
        if shift { modifier_keys |= ModifierKeys::SHIFT; }

        use ratzilla::event::KeyCode as RzKeyCode;
        Self {
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
}