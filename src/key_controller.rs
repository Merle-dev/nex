use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::{
    key_table_parser::parse,
    tree::{self, Tree, TreeValue},
};

pub struct KeyController {
    pub mode: char,
    pub event_buffer: Vec<Event>,
    pub key_map: Tree<KeyCode, (String, String)>,
}

impl KeyController {
    pub fn new() -> Self {
        let mut key_map = Tree::new();
        key_map.add(parse("<Esc>".into()), ("Quit".into(), "*".into()));
        key_map.add(parse("vi".into()), ("Select".into(), "n".into()));
        key_map.add(parse("<Enter>".into()), ("Enter".into(), "*".into()));
        key_map.add(parse("<Up>".into()), ("Up".into(), "*".into()));
        key_map.add(parse("<Down>".into()), ("Down".into(), "*".into()));
        key_map.add(parse("<Left>".into()), ("Left".into(), "*".into()));
        key_map.add(parse("<Right>".into()), ("Right".into(), "*".into()));
        key_map.add(parse("<PageUp>".into()), ("PageUp".into(), "*".into()));
        key_map.add(parse("<PageDown>".into()), ("PageDown".into(), "*".into()));
        key_map.add(
            parse("<Backspace>".into()),
            ("Backspace".into(), "*".into()),
        );
        Self {
            mode: 'n',
            event_buffer: vec![],
            key_map,
        }
    }
    pub fn get(&self) -> Option<&TreeValue<KeyCode, (String, String)>> {
        self.key_map.get(
            self.event_buffer
                .iter()
                .fold(Vec::new(), |acc, event| match *event {
                    Event::Key(key) => [acc, vec![key.code]].concat(),
                    _ => acc,
                }),
        )
    }
}

pub fn key_from_event(ev: event::Event) -> Option<char> {
    match ev {
        Event::Key(ke) => match ke.code {
            KeyCode::Char(c) => Some(c),
            _ => None,
        },
        _ => None,
    }
}
