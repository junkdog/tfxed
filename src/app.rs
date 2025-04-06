use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Color, style::Stylize};
use ratatui::widgets::{Block, BorderType, Paragraph};
use crate::event::{AppEvent, KeyCode, KeyEvent};

pub struct App {
    counter: usize,
    is_running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            counter: 0,
            is_running: true,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title("tfxed")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let c = self.counter();
        let text = format!(
            "This is a Ratzilla template.\n\
             Press left and right to increment and decrement the counter respectively.\n\
             Counter: {c}",
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, frame.area());
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Tick => {
                // Update the state based on the tick event
            }
            AppEvent::KeyPress(KeyEvent { key_code: KeyCode::Esc, .. }) => {
                self.is_running = false;
            }
            AppEvent::KeyPress(KeyEvent { key_code, .. }) => {
                self.increment_counter();
            }
            _ => {}
        }
    }
}
