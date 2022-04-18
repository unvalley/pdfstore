pub mod tabs;

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Rect},
    Frame,
};

use crate::inputs::{Key};

pub trait DrawableComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<()>;
}

pub trait Component {
    fn commands(&self);
    fn event(&mut self, key: Key) -> anyhow::Result<EventState>;
    fn focused(&self) -> bool { false }
    fn focus(&mut self, _focus: bool) {}
}