pub mod managed_pdf_list;
pub mod pdf_detail;
pub mod pdf_file_loader;
pub mod pdf_import_popup;
pub mod searchbar;
pub mod unmanaged_pdf_list;

pub use managed_pdf_list::ManagedPdfListComponent;
pub use pdf_detail::PdfDetailComponent;
pub use pdf_file_loader::PdfFileLoader;
pub use pdf_import_popup::PdfImportPopup;
pub use searchbar::SearchbarComponent;
pub use unmanaged_pdf_list::UnmanagedPdfListComponent;

use std::path::Path;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::components::{Component, DrawableComponent, EventState};
use crate::inputs::key::Key;
use crate::key_config::KeyConfig;

pub enum InboxFocus {
    Searchbar,
    /// ~/paper
    ManagedPdfList,
    /// e.g. Downloads/, Documents/
    UnmanagedPdfList,
    PdfDetail,
}

pub struct InboxComponent {
    pub searchbar: SearchbarComponent,
    pub managed_pdf_list: ManagedPdfListComponent,
    pub unmanaged_pdf_list: UnmanagedPdfListComponent,
    pub pdf_detail: PdfDetailComponent,
    pub focus: InboxFocus,
    key_config: KeyConfig,
}

impl InboxComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            searchbar: SearchbarComponent::new(key_config.clone()),
            managed_pdf_list: ManagedPdfListComponent::new(key_config.clone()),
            unmanaged_pdf_list: UnmanagedPdfListComponent::new(key_config.clone()),
            pdf_detail: PdfDetailComponent::new(key_config.clone()),
            focus: InboxFocus::ManagedPdfList,
            key_config,
        }
    }

    pub async fn update(&mut self) -> anyhow::Result<()> {
        // TODO: refactor
        let managed_file_path = Path::new("/Users/unvalley/papers");
        let managed_pdf_files = self.managed_pdf_list.load_files(managed_file_path);
        match managed_pdf_files {
            Ok(pdf_files) => self.managed_pdf_list.update(pdf_files),
            Err(_) => todo!(),
        }

        let unmanaged_file_path = Path::new("/Users/unvalley/Downloads");
        let unmanaged_pdf_files = self.unmanaged_pdf_list.load_files(unmanaged_file_path);
        match unmanaged_pdf_files {
            Ok(pdf_files) => self.unmanaged_pdf_list.update(pdf_files),
            Err(_) => todo!(),
        }

        Ok(())
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
            focused && matches!(self.focus, InboxFocus::Searchbar),
        )?;

        self.managed_pdf_list.draw(
            f,
            list_layout[0],
            focused && matches!(self.focus, InboxFocus::ManagedPdfList),
        )?;
        self.unmanaged_pdf_list.draw(
            f,
            list_layout[1],
            focused && matches!(self.focus, InboxFocus::UnmanagedPdfList,),
        )?;

        self.pdf_detail.draw(
            f,
            inbox_layout[1],
            focused && matches!(self.focus, InboxFocus::PdfDetail),
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
                self.focus = InboxFocus::ManagedPdfList;
                return Ok(EventState::Consumed);
            }
            Key::Down => {
                // focus to existing
                self.focus = InboxFocus::UnmanagedPdfList;
                return Ok(EventState::Consumed);
            }
            Key::Right => {
                // detailにfocus
                self.focus = InboxFocus::PdfDetail;
                return Ok(EventState::Consumed);
            }
            Key::Left => {
                // detailからどちらかにfocus
                self.focus = InboxFocus::ManagedPdfList;
                return Ok(EventState::Consumed);
            }
            _ => Ok(EventState::NotConsumed),
        }
    }
}
