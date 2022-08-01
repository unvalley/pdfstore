pub mod managed_pdf_list;
pub mod pdf_detail;
pub mod pdf_file_loader;
pub mod pdf_import_popup;
pub mod searchbar;
pub mod unmanaged_pdf_list;

pub use managed_pdf_list::ManagedPdfListComponent;
pub use pdf_detail::PdfDetailComponent;
pub use pdf_file_loader::PdfFileLoader;
pub use pdf_import_popup::PdfImportPopupComponent;
pub use searchbar::SearchbarComponent;
pub use unmanaged_pdf_list::UnmanagedPdfListComponent;

use std::path::Path;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear},
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
    ImportPopup,
    // PdfDetail,
}

pub struct InboxComponent {
    pub searchbar: SearchbarComponent,
    pub managed_pdf_list: ManagedPdfListComponent,
    pub unmanaged_pdf_list: UnmanagedPdfListComponent,
    pub pdf_detail: PdfDetailComponent,
    pub pdf_import_popup: PdfImportPopupComponent,
    pub focus: InboxFocus,
    pub show_import_popup: bool,
    key_config: KeyConfig,
}

impl InboxComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            searchbar: SearchbarComponent::new(key_config.clone()),
            managed_pdf_list: ManagedPdfListComponent::new(key_config.clone()),
            unmanaged_pdf_list: UnmanagedPdfListComponent::new(key_config.clone()),
            pdf_detail: PdfDetailComponent::new(key_config.clone()),
            pdf_import_popup: PdfImportPopupComponent::new(key_config.clone()),
            // ui state
            focus: InboxFocus::ManagedPdfList,
            show_import_popup: false,
            key_config,
        }
    }

    pub async fn update(&mut self) -> anyhow::Result<()> {
        // TODO: refactor
        let managed_file_path = Path::new("/Users/unvalley/papers");
        let managed_pdf_files = self.managed_pdf_list.load_files(managed_file_path);
        match managed_pdf_files {
            Ok(pdf_files) => self.managed_pdf_list.update(pdf_files),
            Err(e) => panic!("Tried to load pdf files but an error occured: {:?}", e),
        }

        let unmanaged_file_path = Path::new("/Users/unvalley/Downloads");
        let unmanaged_pdf_files = self.unmanaged_pdf_list.load_files(unmanaged_file_path);
        match unmanaged_pdf_files {
            Ok(pdf_files) => self.unmanaged_pdf_list.update(pdf_files),
            Err(e) => panic!("Tried to load pdf files but an error occured: {:?}", e),
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
            // if we use PdfDetailComponent, use [Constraint::Percentage(70), Constraint::Percentage(30)]
            .constraints([Constraint::Percentage(100)].as_ref())
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

        // How to pass the pdf info data
        if self.show_import_popup {
            let selected_pdf = self.managed_pdf_list.find_selected_file();
            let area = centered_rect(80, 50, f.size());

            f.render_widget(Clear, area);
            f.render_widget(
                Block::default()
                    .title(&*selected_pdf.file_name)
                    .borders(Borders::ALL),
                area,
            );
        }

        Ok(())
    }
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

impl Component for InboxComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        let is_focusing_pdf_list = matches!(self.focus, InboxFocus::ManagedPdfList)
            || matches!(self.focus, InboxFocus::UnmanagedPdfList);

        // REFACTOR: Is correct here to handle keys?
        match key {
            Key::Enter => {
                if is_focusing_pdf_list {
                    self.show_import_popup = true;
                    self.focus = InboxFocus::ImportPopup;
                    return Ok(EventState::Consumed);
                }
                Ok(EventState::NotConsumed)
            }
            Key::Esc => {
                if matches!(self.focus, InboxFocus::ImportPopup) {
                    self.show_import_popup = false;
                    // TODO: wanna make to focus before open. (should I use stack?)
                    self.focus = InboxFocus::ManagedPdfList;
                    return Ok(EventState::Consumed);
                }
                Ok(EventState::NotConsumed)
            }
            Key::Char('/') => {
                self.focus = InboxFocus::Searchbar;
                Ok(EventState::Consumed)
            }
            Key::Tab => {
                self.focus = match self.focus {
                    InboxFocus::Searchbar => InboxFocus::ManagedPdfList,
                    InboxFocus::ManagedPdfList => InboxFocus::UnmanagedPdfList,
                    InboxFocus::UnmanagedPdfList => InboxFocus::Searchbar,
                    // TODO
                    InboxFocus::ImportPopup => InboxFocus::ImportPopup,
                };
                Ok(EventState::Consumed)
            }
            _ => Ok(EventState::NotConsumed),
        }
    }

    fn focus(&mut self, _focus: bool) {}
}
