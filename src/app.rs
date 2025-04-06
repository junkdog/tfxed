use std::sync::mpsc::Sender;
use std::time::Instant;
use ansi_to_tui::IntoText;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Alignment, Offset, Rect};
use ratatui::prelude::{Color, style::Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use tachyonfx::{ref_count, BufferRenderer, CenteredShrink, Duration, EffectManager, RefCount};
use crate::event::{AppEvent, KeyCode, KeyEvent};

pub struct App {
    sender: std::sync::mpsc::Sender<AppEvent>,
    effects: EffectManager<u32>,
    canvas_buf: RefCount<Buffer>,
    last_tick_instant: Instant,
    last_tick_duration: Duration,
    counter: usize,
    is_running: bool,
}

impl App {
    pub fn new(sender: Sender<AppEvent>) -> Self {
        let area = ratatui::layout::Rect::new(0, 0, 20, 10);
        let canvas_buf = ref_count(Buffer::empty(area));

        Self {
            sender,
            effects: Default::default(),
            canvas_buf,
            last_tick_instant: std::time::Instant::now(),
            last_tick_duration: Duration::default(),
            counter: 0,
            is_running: true,
        }
    }

    pub fn sender(&self) -> Sender<AppEvent> {
        self.sender.clone()
    }

    pub fn render_ui(&self, frame: &mut Frame) {
        let canvas_area = self.canvas_buf.borrow().area;
        let frame_area = frame.area();

        let Rect { x, y, .. } = frame_area
            .inner_centered(canvas_area.width, canvas_area.height);

        self.canvas_buf.borrow()
            .render_buffer(Offset { x: x as _, y: y as _ }, &mut frame.buffer_mut());
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
            AppEvent::UpdateCanvas(s) => self.update_canvas(s),
            _ => {}
        }
    }

    fn update_canvas(&mut self, source: String) {
        let input = source.into_text().unwrap_or_else(|_| {
            eprintln!("Failed to parse input file");
            std::process::exit(1);
        });

        let w = input.lines.iter().map(|line| line.width()).max().unwrap_or(0);
        let h = input.lines.len();

        let area = ratatui::layout::Rect::new(0, 0, w as _, h as _);
        let canvas_buf = ref_count(Buffer::empty(area));

        input.render(area, &mut canvas_buf.borrow_mut());

        // replace the old buffer with the new one
        self.canvas_buf = canvas_buf;
    }
}
