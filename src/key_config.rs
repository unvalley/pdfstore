use crate::inputs::key::Key;

#[derive(Debug, Clone)]
pub struct KeyConfig {
    pub scroll_up: Key,
    pub scroll_down: Key,
    pub enter: Key,
    pub exit: Key,
    pub quit: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            scroll_up: Key::Char('k'),
            scroll_down: Key::Char('j'),
            enter: Key::Enter,
            exit: Key::Ctrl('c'),
            quit: Key::Char('q'),
        }
    }
}
