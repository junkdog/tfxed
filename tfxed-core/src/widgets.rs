use ratatui::buffer::Buffer;
use ratatui::layout::{Offset, Rect, Size};
use ratatui::style::Style;
use ratatui::widgets::Widget;

pub struct Ruler {
    measured: Size,
    style: Style
}

impl Ruler {
    pub fn new(measured: Size) -> Self {
        Self {
            measured,
            style: Style::default()
        }
    }

    pub fn measure(&mut self, size: Size) {
        self.measured = size;
    }

    pub fn style(mut self, style: Style) -> Ruler {
        self.style = style;
        self
    }

    pub fn ruler_area(&self) -> Rect {
        Rect::new(0, 0, self.measured.width + 4, self.measured.height + 4)
    }
}

fn draw_rows(
    area: Rect,
    style: Style,
    buf: &mut Buffer,
) {
    let mut draw_row = |x, y, width, buf: &mut Buffer| {
        (x..(x + width))
            .enumerate()
            .map(|(i, x)| (i % 10, x))
            .for_each(|(i, x)| {
                buf[(x, y)]
                    .set_style(style.clone())
                    .set_char(char::from_digit(i as u32, 10).unwrap());
            });
    };

    let mut draw_row_10 = |x, y, width, buf: &mut Buffer| {
        (x..(x + width))
            .enumerate()
            .filter(|(i, _)| *i > 0)
            .filter(|(i, x)| i % 10 == 0)
            .map(|(i, x)| (i / 10, x))
            .map(|(i, x)| (i % 10, x))
            .for_each(|(i, x)| {
                buf[(x, y)]
                    .set_style(style.clone())
                    .set_char(char::from_digit(i as u32, 10).unwrap());
            });
    };

    draw_row_10(area.x + 2, area.y,            area.width - 4, buf);
    draw_row(   area.x + 2, area.y + 1,        area.width - 4, buf);
    draw_row(   area.x + 2, area.bottom() - 2, area.width - 4, buf);
    draw_row_10(area.x + 2, area.bottom() - 1, area.width - 4, buf);
}

fn draw_cols(
    area: Rect,
    style: Style,
    buf: &mut Buffer,
) {
    let mut draw_col = |x, y, height, buf: &mut Buffer| {
        (y..(y + height))
            .enumerate()
            .map(|(i, y)| (i % 10, y))
            .for_each(|(i, y)| {
                buf[(x, y)]
                    .set_style(style.clone())
                    .set_char(char::from_digit(i as u32, 10).unwrap());
            });
    };

    let mut draw_col_10 = |x, y, height, buf: &mut Buffer| {
        (y..(y + height))
            .enumerate()
            .filter(|(i, _)| *i > 0)
            .filter(|(i, x)| i % 10 == 0)
            .map(|(i, y)| (i / 10, y))
            .map(|(i, y)| (i % 10, y))
            .for_each(|(i, y)| {
                buf[(x, y)]
                    .set_style(style.clone())
                    .set_char(char::from_digit(i as u32, 10).unwrap());
            });
    };

    draw_col_10(area.x,           area.y + 2, area.height - 4, buf);
    draw_col(   area.x + 1,       area.y + 2, area.height - 4, buf);
    draw_col(   area.right() - 2, area.y + 2, area.height - 4, buf);
    draw_col_10(area.right() - 1, area.y + 2, area.height - 4, buf);
}

impl Widget for Ruler {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ruler_area = self.ruler_area()
            .offset(Offset { x: area.x as _, y: area.y as _ })
            .clamp(*buf.area());

        draw_rows(ruler_area, self.style, buf);
        draw_cols(ruler_area, self.style, buf);
    }
}