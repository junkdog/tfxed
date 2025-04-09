// lib.rs
pub mod app;
pub mod dispatcher;
pub mod effects;
pub mod event;
pub mod gruvbox;

// Re-export common items for convenience
pub use app::App;
pub use dispatcher::Dispatcher;
pub use event::{AppEvent, KeyCode, KeyEvent, ModifierKeys};