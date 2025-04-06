use bitflags::bitflags;

pub enum AppEvent {
    Tick,
    KeyPress(KeyEvent),
    Resize(u16, u16),
    Quit,
}

pub struct KeyEvent {
    pub key_code: KeyCode,
    pub modifier_keys: ModifierKeys
}

pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Esc,
}

bitflags! {
    pub struct ModifierKeys: u8 {
        const SHIFT   = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT     = 0b0000_0100;
        const META    = 0b0000_1000;
        const SUPER   = 0b0001_0000;
    }
}
