use std::{
    io::Result,
    path::Path,
    sync::mpsc::{self, Receiver},
    thread,
    time::Instant,
};

use ratatui::{
    crossterm::event::{self, Event},
    init,
    layout::Position,
    restore,
};

use crate::{
    DESIRED_FRAME_TIME,
    key_controller::{KeyController, key_from_event},
    text_buffer::TextBuffer,
    tree::TreeValue,
};

mod widget;

pub struct App {
    fps: f32,
    exit: bool,
    footer: Vec<String>,
    cursor: Position,
    pub text: TextBuffer,
    pub header_update: bool,
    pub footer_update: bool,
    pub main_area_update: bool,
    pub key_controller: KeyController,
    pub event_rx: Receiver<Result<Event>>,
}

impl App {
    pub fn new<T: AsRef<Path>>(file: Option<T>) -> Result<Self> {
        let (tx, event_rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                tx.send(event::read()).unwrap();
            }
        });
        Ok(Self {
            fps: 0.0,
            exit: false,
            footer: vec!["".to_string(); 3],
            cursor: Position::new(0, 1),
            text: file
                .and_then(|file| TextBuffer::from_file(file).ok())
                .unwrap_or_else(TextBuffer::new),
            header_update: true,
            footer_update: true,
            main_area_update: true,
            key_controller: KeyController::new(),
            event_rx,
        })
    }
    pub fn run(&mut self) -> Result<()> {
        let mut terminal = init();
        while !self.exit {
            let now = Instant::now();
            self.main_area_update = self.event_block()?;
            self.render(&mut terminal)?;
            terminal.show_cursor()?;
            terminal.set_cursor_position(self.cursor)?;
            self.fps = 1000.0 / now.elapsed().as_millis() as f32;
            self.footer_update = true;
        }
        restore();
        Ok(())
    }

    fn event_block(&mut self) -> Result<bool> {
        Ok(match self.event_rx.recv_timeout(DESIRED_FRAME_TIME).ok() {
            Some(event) => {
                let ev = event?;
                self.key_controller.event_buffer.push(ev.clone());
                match self.key_controller.get() {
                    Some(TreeValue::Leaf((leaf, mode))) => {
                        self.command(leaf.to_string(), mode.to_string())
                    }
                    None => self.key_controller.event_buffer = vec![],
                    _ => (),
                };
                if key_from_event(ev).is_some_and(|char| self.text.edit(char.to_string())) {
                    self.text.right();
                }

                true
            }
            None => false,
        })
    }
    fn command(&mut self, command: String, mode: String) {
        self.key_controller.event_buffer = vec![];
        let mode = mode.as_str();
        if mode == "*" || mode.contains(self.key_controller.mode) {
            match command.as_str() {
                "Quit" => self.exit = true,
                "Enter" => self.text.new_line(),
                "Up" => self.text.up(),
                "Down" => self.text.down(),
                "Left" => self.text.left(),
                "Right" => self.text.right(),
                "PageUp" => self.text.aimed_scroll = -12,
                "PageDown" => self.text.aimed_scroll = 12,
                "Backspace" => self.text.delete(),
                _ => (),
            }
        }
    }
}
