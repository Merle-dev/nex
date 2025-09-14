use std::io::Result;

use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect},
    widgets::{Paragraph, Widget},
};

use crate::app::App;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(100),
            Constraint::Length(1),
        ]);
        let [header, main, footer] = area.layout(&layout);
        self.render_header(header, buf);
        self.render_footer(footer, buf);
        self.text.render(main, buf);
        self.cursor = self.text.window_cursor;
        self.footer[1] = format!("cursor: {:?}", self.text.cursor);
        self.footer[2] = format!("scroll: {:?}", self.text.scroll);
    }
}

impl App {
    pub fn render(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }
    fn render_line(&mut self, area: Rect, buf: &mut Buffer, text: String, update: bool) {
        if update {
            Paragraph::new(text).centered().render(area, buf);
        }
    }
    fn render_header(&mut self, area: Rect, buf: &mut Buffer) {
        self.render_line(area, buf, format!("FPS: {}", self.fps), self.header_update);
    }
    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        self.render_line(area, buf, format!("{:?}", self.footer), self.footer_update);
    }
}
