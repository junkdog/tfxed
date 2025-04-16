use std::sync::mpsc;
use crossterm::event::KeyEvent as CrosstermKeyEvent;
use tfxed_core::{AppEvent, Dispatcher, KeyCode, KeyEvent, ModifierKeys};

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,
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
                sender.send(AppEvent::KeyPress(convert_key_event(e))),
            CrosstermEvent::Resize(w, h) =>
                sender.send(AppEvent::Resize(w, h)),

            _ => Ok(())
        }.expect("event should have been sent");
    }
}


fn convert_key_event(CrosstermKeyEvent { code, modifiers, .. }: CrosstermKeyEvent) -> KeyEvent {
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

    KeyEvent {
        key_code,
        modifier_keys
    }
}
