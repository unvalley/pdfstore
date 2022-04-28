use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::{
    components::{Component, DrawableComponent, EventState},
    inputs::key::Key,
    key_config::KeyConfig,
};

pub struct ManagedPdfListComponent {
    pub files: Vec<String>,
    key_config: KeyConfig,
}

impl ManagedPdfListComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            files: Vec::new(),
            key_config: key_config.clone(),
        }
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

        let body = Paragraph::new(vec![Spans::from(Span::raw("Test"))])
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(border_style)
                    .title("Paper"),
            );

        f.render_widget(body, area);

        Ok(())
    }
}

impl Component for ManagedPdfListComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        Ok(EventState::Consumed)
    }
}
