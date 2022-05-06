pub mod command;
pub mod inbox;
pub mod pdf_import_popup;
pub mod utils;

use tui::{backend::Backend, layout::Rect, Frame};

use crate::inputs::key::Key;

pub trait DrawableComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect,
        focused: bool,
    ) -> anyhow::Result<()>;
}

pub trait Component {
    fn commands(&self);
    fn event(&mut self, key: Key) -> anyhow::Result<EventState>;
    fn focused(&self) -> bool {
        false
    }
    fn focus(&mut self, _focus: bool) {}
}

#[derive(Copy, Clone)]
pub enum ScrollType {
    Up,
    Down,
    // PageUp,
    // PageDown
}

#[derive(PartialEq)]
pub enum CommandBlocking {
    Blocking,
    PassingOn,
}

#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    NotConsumed,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}

impl From<bool> for EventState {
    fn from(consumed: bool) -> Self {
        if consumed {
            Self::Consumed
        } else {
            Self::NotConsumed
        }
    }
}
