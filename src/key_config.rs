use crate::inputs::key::Key;

#[derive(Debug, Clone)]
pub struct KeyConfig {
    // focus
    pub focus_left: Key,
    pub focus_right: Key,
    pub focus_up: Key,
    pub focus_down: Key,
    // scroll
    pub scroll_up: Key,
    pub scroll_down: Key,
    //
    pub enter: Key,
    pub exit: Key,
    pub quit: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            focus_left: Key::Left,
            focus_right: Key::Right,
            focus_up: Key::Up,
            focus_down: Key::Down,
            scroll_up: Key::Char('k'),
            scroll_down: Key::Char('j'),
            enter: Key::Enter,
            exit: Key::Ctrl('c'),
            quit: Key::Char('q'),
        }
    }
}
