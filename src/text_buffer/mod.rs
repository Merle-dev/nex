use std::{fs, io::Result, path::Path};

mod text_mutation;

use ratatui::{
    buffer::Cell,
    layout::Position,
    style::palette::material::GRAY,
    widgets::{List, ListItem, Widget},
};

pub struct TextBuffer {
    pub raw_lines: Vec<String>,
    lines: Vec<Vec<Cell>>,
    pub cursor: (usize, usize),
    pub window_cursor: Position,
    pub scroll: i64,
    pub aimed_scroll: i64,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            raw_lines: vec![],
            lines: vec![],
            cursor: (0, 0),
            window_cursor: Position::ORIGIN,
            scroll: 0,
            aimed_scroll: 0,
        }
    }
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(Self {
            raw_lines: fs::read_to_string(path)?
                .split("\n")
                .map(String::from)
                .collect(),
            lines: vec![],
            cursor: (0, 0),
            window_cursor: Position::ORIGIN,
            scroll: 0,
            aimed_scroll: 0,
        })
    }
    pub fn up(&mut self) {
        self.cursor.1 = (self.cursor.1 as i64 - 1).max(0) as usize;
    }
    pub fn down(&mut self) {
        self.cursor.1 = (self.cursor.1 + 1).min(self.raw_lines.len() - 2);
    }
    pub fn left(&mut self) {
        self.cursor.0 = (self.cursor.0 as i64 - 1).max(0) as usize;
    }
    pub fn right(&mut self) {
        if self.cursor.0 < self.raw_lines[self.cursor.1].len() {
            self.cursor.0 += 1;
        }
    }
    pub fn scroll_animate(&mut self) {
        if self.aimed_scroll == 0 {
            return;
        } else if self.aimed_scroll < 0 {
            self.up();
            self.aimed_scroll += 1;
        } else {
            self.down();
            self.aimed_scroll -= 1;
        }
    }
}

impl Widget for &mut TextBuffer {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.scroll_animate();
        self.window_cursor.x = self.cursor.0 as u16 + area.x;
        self.window_cursor.y =
            (self.cursor.1 as u16 - self.scroll as u16 + area.y).clamp(area.y, area.height);

        if self.scroll > self.cursor.1 as i64 {
            self.scroll = self.cursor.1 as i64;
        } else if (self.scroll as usize + area.height as usize - 1) < self.cursor.1 {
            self.scroll = self.cursor.1 as i64 - (area.height as i64 - 1);
        }

        let range = if self.raw_lines.len() < area.height as usize {
            0..self.raw_lines.len()
        } else {
            self.scroll as usize
                ..(self.scroll as usize + area.height as usize).min(self.raw_lines.len())
        };

        List::new(
            range
                .into_iter()
                .map(|index| ListItem::style(self.raw_lines[index].clone().into(), GRAY.c400)),
        )
        .render(area, buf);
    }
}
