use std::time::Instant;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::{Color, style::Stylize};
use ratatui::widgets::{Block, BorderType, Paragraph};
use tachyonfx::{ref_count, Duration, EffectManager, RefCount};
use crate::event::{AppEvent, KeyCode, KeyEvent};

pub struct App {
    effects: EffectManager<u32>,
    buf_base: RefCount<Buffer>,
    last_tick_instant: Instant,
    last_tick_duration: Duration,
    counter: usize,
    is_running: bool,
}

impl App {
    pub fn new() -> Self {
        let area = ratatui::layout::Rect::new(0, 0, 80, 40);
        Self {
            effects: Default::default(),
            buf_base: ref_count(Buffer::empty(area)),
            last_tick_instant: std::time::Instant::now(),
            last_tick_duration: Duration::default(),
            counter: 0,
            is_running: true,
        }
    }

    pub fn render_ui(&self, frame: &mut Frame) {
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

    pub fn render_effects(&mut self, frame: &mut Frame) {
        let d = self.last_tick_duration;
        let rect = frame.area();
        self.effects.process_effects(d, frame.buffer_mut(), rect);
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn update_time(&mut self) -> Duration {
        let now = Instant::now();
        let last_frame_duration: Duration = now.duration_since(self.last_tick_instant).into();
        self.last_tick_instant = now;
        self.last_tick_duration = last_frame_duration;
        last_frame_duration
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
                self.counter = self.counter + 1;
            }
            _ => {}
        }
    }
}
