pub mod existing_directory_list;
pub mod paper_directory_list;
pub mod pdf_detail;
pub mod searchbar;

pub use existing_directory_list::ExistingDirectoryListComponent;
pub use paper_directory_list::PaperDirectoryListComponent;
pub use pdf_detail::PdfDetailComponent;
pub use searchbar::SearchbarComponent;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::components::{Component, DrawableComponent, EventState};
use crate::inputs::key::Key;
use crate::key_config::KeyConfig;

enum Focus {
    Searchbar,
    /// ~/paper
    PaperDirectoryList,
    /// e.g. Downloads/, Documents/
    ExistingDirectoryList,
    PdfDetail,
}

pub struct InboxComponent {
    searchbar: SearchbarComponent,
    paper_directory_list: PaperDirectoryListComponent,
    existing_directory_list: ExistingDirectoryListComponent,
    pdf_detail: PdfDetailComponent,
    focus: Focus,
    key_config: KeyConfig,
}

impl InboxComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            searchbar: SearchbarComponent::new(key_config.clone()),
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
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(5)])
            .split(area);

        let inbox_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_layout[1]);

        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inbox_layout[0]);

        self.searchbar.draw(
            f,
            main_layout[0],
            focused && matches!(self.focus, Focus::Searchbar),
        )?;

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
