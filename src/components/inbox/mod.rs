pub mod existing_directory_list;
pub mod paper_directory_list;
pub mod pdf_detail;

pub use existing_directory_list::ExistingDirectoryListComponent;
pub use paper_directory_list::PaperDirectoryListComponent;
pub use pdf_detail::PdfDetailComponent;

use crate::components::{Component, DrawableComponent, EventState};
use crate::inputs::key::Key;
use crate::key_config::KeyConfig;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

enum Focus {
    /// ~/paper
    PaperDirectoryList,
    /// e.g. Downloads/, Documents/
    ExistingDirectoryList,
    PdfDetail,
}

pub struct InboxComponent {
    paper_directory_list: PaperDirectoryListComponent,
    existing_directory_list: ExistingDirectoryListComponent,
    pdf_detail: PdfDetailComponent,
    focus: Focus,
    key_config: KeyConfig,
}

impl InboxComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            paper_directory_list: PaperDirectoryListComponent::new(key_config.clone()),
            existing_directory_list: ExistingDirectoryListComponent::new(key_config.clone()),
            pdf_detail: PdfDetailComponent::new(key_config.clone()),
            focus: Focus::PaperDirectoryList,
            key_config,
        }
    }
}

impl DrawableComponent for InboxComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        focused: bool,
    ) -> anyhow::Result<()> {
        let inbox_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(area);

        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inbox_layout[0]);

        self.paper_directory_list.draw(
            f,
            list_layout[0],
            focused && matches!(self.focus, Focus::PaperDirectoryList),
        )?;
        self.existing_directory_list.draw(
            f,
            list_layout[1],
            focused && matches!(self.focus, Focus::ExistingDirectoryList),
        )?;

        self.pdf_detail.draw(
            f,
            inbox_layout[1],
            focused && matches!(self.focus, Focus::PdfDetail),
        )?;

        Ok(())
    }
}

impl Component for InboxComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        match key {
            Key::Up => {
                // focus to paper
                self.focus = Focus::PaperDirectoryList;
                return Ok(EventState::Consumed);
            }
            Key::Down => {
                // focus to existing
                self.focus = Focus::ExistingDirectoryList;
                return Ok(EventState::Consumed);
            }
            Key::Right => {
                // detailにfocus
                self.focus = Focus::PdfDetail;
                return Ok(EventState::Consumed);
            }
            Key::Left => {
                // detailからどちらかにfocus
                self.focus = Focus::PaperDirectoryList;
                return Ok(EventState::Consumed);
            }
            _ => Ok(EventState::NotConsumed),
        }
    }
}
