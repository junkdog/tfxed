use std::sync::mpsc::Sender;
use ansi_to_tui::IntoText;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Alignment, Offset, Rect};
use ratatui::prelude::{Color, style::Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use tachyonfx::{ref_count, BufferRenderer, CenteredShrink, Duration, Effect, EffectManager, RefCount};
use tachyonfx::dsl::EffectDsl;
use tachyonfx::fx::consume_tick;
use crate::effects::display_dsl_error;
use crate::event::{AppEvent, KeyCode, KeyEvent};

pub struct App {
    sender: std::sync::mpsc::Sender<AppEvent>,
    effects: EffectManager<u32>,
    canvas_buf: RefCount<Buffer>,
    #[cfg(feature = "crossterm-backend")]
    last_tick_instant: std::time::Instant,
    #[cfg(feature = "web-backend")]
    last_tick_instant: web_time::Instant,
    last_tick_duration: Duration,
    counter: usize,
    is_running: bool,
}

impl App {
    pub fn new(sender: Sender<AppEvent>) -> Self {
        let area = ratatui::layout::Rect::new(0, 0, 20, 10);
        let canvas_buf = ref_count(Buffer::empty(area));

        #[cfg(feature = "crossterm-backend")]
        let last_tick_instant = std::time::Instant::now();

        #[cfg(feature = "web-backend")]
        let last_tick_instant = web_time::Instant::now();

        Self {
            sender,
            effects: Default::default(),
            canvas_buf,
            last_tick_instant,
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

        // let Rect { x, y, .. } = frame_area
        //     .inner_centered(canvas_area.width, canvas_area.height);

        self.canvas_buf.borrow()
            .render_buffer(Offset { x: 2, y: 2 }, &mut frame.buffer_mut());
            // .render_buffer(Offset { x: x as _, y: y as _ }, &mut frame.buffer_mut());
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

    #[cfg(feature = "crossterm-backend")]
    pub fn update_time(&mut self) -> Duration {
        let now = std::time::Instant::now();
        let last_frame_duration: Duration = now.duration_since(self.last_tick_instant).into();
        self.last_tick_instant = now;
        self.last_tick_duration = last_frame_duration;
        last_frame_duration
    }

    #[cfg(feature = "web-backend")]
    pub fn update_time(&mut self) -> Duration {
        let now = web_time::Instant::now();
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
            AppEvent::CompileDsl(dsl) => {
                // Compile the DSL and update the canvas
                let effect = EffectDsl::new()
                    .compiler()
                    .compile(dsl.as_str());

                match effect {
                    Ok(effect) => {
                        self.effects.add_unique_effect(0u32, effect);

                        // clear any old error popup
                        self.effects.add_unique_effect(0xf00u32, consume_tick());
                    }
                    Err(e)     => {
                        #[cfg(feature = "web-backend")]
                        web_sys::console::error_1(&e.to_string().into());

                        self.display_error_popup(
                            e.source.to_string(),
                            e.error_line().to_string(),
                            (e.line(), e.column()),
                        )
                    }
                }
            }
            _ => {}
        }
    }

    fn display_error_popup(
        &mut self,
        error_message: String,
        referenced_code: String,
        position: (u32, u32),
    ) {
        let duration = Duration::from_millis(15000);
        self.effects.add_unique_effect(0xf00u32,
            display_dsl_error(duration, error_message, referenced_code, position)
        );
    }

    fn update_canvas(&mut self, source: String) {
        let input = source.into_text().unwrap_or_else(|_| {
            eprintln!("Failed to parse input file");
            std::process::exit(1);
        });

        let w = input.lines.iter().map(|line| line.width()).max().unwrap_or(0);
        let h = input.lines.len();

        let area = Rect::new(0, 0, w as _, h as _);
        let canvas_buf = ref_count(Buffer::empty(area));

        input.render(area, &mut canvas_buf.borrow_mut());

        // replace the old buffer with the new one
        self.canvas_buf = canvas_buf;
    }
}
