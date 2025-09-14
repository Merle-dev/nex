use ratatui::crossterm::event::KeyCode;

pub fn parse(str: String) -> Vec<KeyCode> {
    divide_str(str)
        .iter()
        .map(|key| {
            if key.len() == 1 {
                KeyCode::Char(key.chars().nth(0).unwrap())
            } else {
                parse_special_key(key.to_string())
            }
        })
        .collect::<Vec<KeyCode>>()
}

fn parse_special_key(str: String) -> KeyCode {
    match str.as_str() {
        "Esc" => KeyCode::Esc,
        "Space" => KeyCode::Char(' '),
        "F1" => KeyCode::F(1),
        "F2" => KeyCode::F(2),
        "F3" => KeyCode::F(3),
        "F4" => KeyCode::F(4),
        "F5" => KeyCode::F(5),
        "F6" => KeyCode::F(6),
        "F7" => KeyCode::F(7),
        "F8" => KeyCode::F(8),
        "F9" => KeyCode::F(9),
        "F10" => KeyCode::F(10),
        "F11" => KeyCode::F(11),
        "F12" => KeyCode::F(12),
        "F13" => KeyCode::F(13),
        "F14" => KeyCode::F(14),
        "F15" => KeyCode::F(15),
        "F16" => KeyCode::F(16),
        "F17" => KeyCode::F(17),
        "F18" => KeyCode::F(18),
        "F19" => KeyCode::F(19),
        "F20" => KeyCode::F(20),
        "Print" => KeyCode::PrintScreen,
        "Del" => KeyCode::Delete,
        "Tab" => KeyCode::Tab,
        "Enter" => KeyCode::Enter,
        "BackTab" => KeyCode::BackTab,
        "Backspace" => KeyCode::Backspace,
        "Insert" => KeyCode::Insert,
        "CapsLock" => KeyCode::CapsLock,
        "NumLock" => KeyCode::NumLock,
        "ScrollLock" => KeyCode::ScrollLock,
        "Pause" => KeyCode::Pause,
        "PageDown" => KeyCode::PageDown,
        "PageUp" => KeyCode::PageUp,
        "Menu" => KeyCode::Menu,
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "Begin" => KeyCode::KeypadBegin,
        "Up" => KeyCode::Up,
        "Left" => KeyCode::Left,
        "Down" => KeyCode::Down,
        "Right" => KeyCode::Right,
        _ => KeyCode::Null,
    }
}

fn divide_str(str: String) -> Vec<String> {
    if str.is_empty() {
        return vec![];
    }
    if let Some((start, special, rest)) = str.split_once('<').and_then(|(start, end)| {
        end.split_once('>')
            .and_then(|(special, rest)| Some((start, special, rest)))
    }) {
        let start = start.chars().map(String::from).collect::<Vec<String>>();
        [
            start,
            vec![special.to_string()],
            divide_str(rest.to_string()),
        ]
        .concat()
    } else {
        str.chars().map(String::from).collect::<Vec<String>>()
    }
}
