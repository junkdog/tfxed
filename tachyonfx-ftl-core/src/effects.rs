use std::cmp::max;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::prelude::Color;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Clear, Padding};
use tachyonfx::{fx, Duration, Effect};
use crate::gruvbox::Gruvbox;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Default)]
pub enum EffectKind {
    DslErrorPopup,
    #[default]
    Editor
}

pub fn display_dsl_error(
    duration: Duration,
    error_message: String,
    referenced_code: String,
    position: (u32, u32),
) -> Effect {
    use ratatui::widgets::Widget;

    let message_lines = error_message.lines().count();
    let code_lines = referenced_code.lines().count();

    let h = 2           // padding top + bottom
        + message_lines // word-wrappig not handled
        + 1             // empty line
        + code_lines;   // compilation error

    let code_w = referenced_code.lines()
        .map(|line| line.len())
        .max()
        .unwrap_or(0) + 6; // ~space req by position

    let msg_w = error_message.lines()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    let popup_area = Rect {
        x: 2,
        y: 2,
        width: max(code_w, msg_w) as u16 + 2,
        height: h as u16,
    };

    #[derive(Clone)]
    struct State {
        line_no: String,
        error_message: String,
        referenced_code: String,
    }

    let state = State {
        line_no: format!("{}:{} ", position.0, position.1),
        error_message: error_message.to_string(),
        referenced_code: referenced_code.to_string(),
    };

    fx::effect_fn_buf(state, duration, move | state, context, buf| {
        let popup_area = popup_area.intersection(*buf.area());
        Clear.render(popup_area.clone(), buf);

        Block::new()
            .style(Style::new()
                .fg(Gruvbox::light1())
                .bg(Gruvbox::red_bright())
            )
            .padding(Padding::symmetric(1, 1))
            .render(popup_area.clone(), buf);

        let layout = Layout::vertical([
            Constraint::Length(message_lines as u16),
            Constraint::Length(1),
            Constraint::Length(code_lines as u16),
        ]).split(popup_area.inner(Margin::new(1, 1)));

        // message
        Span::from(state.error_message.as_str())
            .style(Style::new().fg(Gruvbox::light0()).add_modifier(Modifier::BOLD))
            .render(layout[0], buf);

        // code
        Text::from(state.referenced_code.as_str())
            .style(Style::new().fg(Gruvbox::light0_soft())
        ).render(layout[2], buf);
    })
}