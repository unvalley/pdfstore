use std::{cmp, path::Path};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    components::{
        inbox::PdfImportPopupComponent, utils::vertical_scroll::VerticalScroll, Component,
        DrawableComponent, EventState, ScrollType,
    },
    domain::pdf_file::PdfFile,
    inputs::key::Key,
    key_config::KeyConfig,
};

use super::pdf_file_loader::PdfFileLoader;

pub struct ManagedPdfListComponent {
    pub pdf_files: Vec<PdfFile>,
    pdf_file_loader: PdfFileLoader,
    list_state: ListState,
    selection: usize,
    scroll: VerticalScroll,
    key_config: KeyConfig,
}

impl ManagedPdfListComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            pdf_files: Vec::new(),
            pdf_file_loader: PdfFileLoader::new(),
            list_state: ListState::default(),
            selection: 0,
            scroll: VerticalScroll::new(),
            key_config: key_config.clone(),
        }
    }

    pub fn find_selected_file(&self) -> &PdfFile {
        let a = self.pdf_files.get(self.selection);
        let b = a.unwrap();
        return b;
    }

    pub fn load_files(&mut self, p: &Path) -> anyhow::Result<Vec<PdfFile>> {
        self.pdf_file_loader.load_files(p)
    }

    pub fn update(&mut self, pdf_files: Vec<PdfFile>) {
        self.pdf_files = pdf_files;
    }

    fn move_selection(&mut self, scroll: ScrollType) -> anyhow::Result<bool> {
        let speed_int = 1;
        let new_selection = match scroll {
            ScrollType::Up => self.selection.saturating_sub(speed_int),
            ScrollType::Down => self.selection.saturating_add(speed_int),
        };
        let selection_max = self.pdf_files.len().saturating_sub(1);
        if selection_max < new_selection {
            return Ok(false);
        }
        let new_selection = cmp::min(new_selection, selection_max);
        let needs_update = new_selection != self.selection;
        self.selection = new_selection;
        Ok(needs_update)
    }
}

impl DrawableComponent for ManagedPdfListComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        focused: bool,
    ) -> anyhow::Result<()> {
        let items: Vec<_> = self
            .pdf_files
            .iter()
            .map(|file| {
                ListItem::new(Spans::from(vec![Span::styled(
                    file.file_name.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        // TODO: unmanaged directories should be multiple
        let title = format!("{} {}", "Managed", "[~/papers]");

        let list_state_idx = Some(self.selection);
        self.list_state.select(list_state_idx);

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(if focused {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default().fg(Color::Gray)
                    })
                    .title(title),
            );

        f.render_stateful_widget(list, area, &mut self.list_state);
        self.scroll.draw(f, area);

        Ok(())
    }
}

impl Component for ManagedPdfListComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        let selection_changed = if key == self.key_config.scroll_down {
            self.move_selection(ScrollType::Down)?
        } else if key == self.key_config.scroll_up {
            self.move_selection(ScrollType::Up)?
        } else {
            false
        };

        match selection_changed {
            true => return Ok(EventState::Consumed),
            false => return Ok(EventState::NotConsumed),
        }
    }
}
