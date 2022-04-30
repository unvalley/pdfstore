use std::path::Path;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    components::{Component, DrawableComponent, EventState},
    domain::pdf_file::PdfFile,
    inputs::key::Key,
    key_config::KeyConfig,
};

use super::pdf_file_loader::PdfFileLoader;

pub struct ManagedPdfListComponent {
    pub pdf_files: Vec<PdfFile>,
    pdf_file_loader: PdfFileLoader,
    key_config: KeyConfig,
}

impl ManagedPdfListComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            pdf_files: Vec::new(),
            pdf_file_loader: PdfFileLoader::new(),
            key_config: key_config.clone(),
        }
    }

    pub fn load_files(&mut self, p: &Path) -> anyhow::Result<Vec<PdfFile>> {
        self.pdf_file_loader.load_files(p)
    }

    pub fn update(&mut self, pdf_files: Vec<PdfFile>) {
        self.pdf_files = pdf_files;
    }
}

impl DrawableComponent for ManagedPdfListComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        focused: bool,
    ) -> anyhow::Result<()> {
        let border_style = if focused {
            Style::default().fg(Color::LightGreen)
        } else {
            Style::default().fg(Color::Gray)
        };

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

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(border_style)
                    .title("Managed"),
            );

        let body = Paragraph::new(vec![Spans::from(Span::raw("Test"))])
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(border_style)
                    .title("Managed"),
            );

        // f.render_widget(body, area);
        f.render_widget(list, area);

        Ok(())
    }
}

impl Component for ManagedPdfListComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        if key == self.key_config.scroll_up {
            // TODO: handle scroll and change eventState
            return Ok(EventState::NotConsumed);
        } else if key == self.key_config.scroll_down {
            // TODO: handle scroll change eventState
            return Ok(EventState::Consumed);
        }

        Ok(EventState::Consumed)
    }
}
