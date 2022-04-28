pub mod managed_pdf_list;
pub mod pdf_detail;
pub mod searchbar;
pub mod unmanaged_pdf_list;

pub use managed_pdf_list::ManagedPdfListComponent;
pub use pdf_detail::PdfDetailComponent;
pub use searchbar::SearchbarComponent;
pub use unmanaged_pdf_list::UnmanagedPdfListComponent;

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
    ManagedPdfList,
    /// e.g. Downloads/, Documents/
    UnmanagedPdfList,
    PdfDetail,
}

pub struct InboxComponent {
    searchbar: SearchbarComponent,
    managed_pdf_list: ManagedPdfListComponent,
    unmanaged_pdf_list: UnmanagedPdfListComponent,
    pdf_detail: PdfDetailComponent,
    focus: Focus,
    key_config: KeyConfig,
}

impl InboxComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            searchbar: SearchbarComponent::new(key_config.clone()),
            managed_pdf_list: ManagedPdfListComponent::new(key_config.clone()),
            unmanaged_pdf_list: UnmanagedPdfListComponent::new(key_config.clone()),
            pdf_detail: PdfDetailComponent::new(key_config.clone()),
            focus: Focus::ManagedPdfList,
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

        self.managed_pdf_list.draw(
            f,
            list_layout[0],
            focused && matches!(self.focus, Focus::ManagedPdfList),
        )?;
        self.unmanaged_pdf_list.draw(
            f,
            list_layout[1],
            focused && matches!(self.focus, Focus::UnmanagedPdfList,),
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
                self.focus = Focus::ManagedPdfList;
                return Ok(EventState::Consumed);
            }
            Key::Down => {
                // focus to existing
                self.focus = Focus::UnmanagedPdfList;
                return Ok(EventState::Consumed);
            }
            Key::Right => {
                // detailにfocus
                self.focus = Focus::PdfDetail;
                return Ok(EventState::Consumed);
            }
            Key::Left => {
                // detailからどちらかにfocus
                self.focus = Focus::ManagedPdfList;
                return Ok(EventState::Consumed);
            }
            _ => Ok(EventState::NotConsumed),
        }
    }
}
