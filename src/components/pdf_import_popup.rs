use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use super::{Component, DrawableComponent, EventState};
use crate::inputs::key::Key;

pub struct PdfImportPopup {}

impl PdfImportPopup {
    pub fn new() -> Self {
        Self {}
    }

    pub fn open(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl DrawableComponent for PdfImportPopup {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        focused: bool,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Component for PdfImportPopup {
    fn commands(&self) {}
    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}
