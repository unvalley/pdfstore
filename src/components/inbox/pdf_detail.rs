use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

use crate::{
    components::{Component, DrawableComponent, EventState},
    inputs::key::Key,
    key_config::KeyConfig,
};

pub struct PdfDetailComponent {
    key_config: KeyConfig,
}

impl PdfDetailComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            key_config: key_config.clone(),
        }
    }
}

impl DrawableComponent for PdfDetailComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        focused: bool,
    ) -> anyhow::Result<()> {
        let key_style = Style::default().fg(Color::LightCyan);

        let mut rows = vec![];
        let row = Row::new(vec![Cell::from(Span::styled("".to_string(), key_style))]);
        rows.push(row);

        let border_style = if focused {
            Style::default().fg(Color::LightGreen)
        } else {
            Style::default().fg(Color::Gray)
        };

        let table = Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(border_style)
                    .title("Detail"),
            )
            .widths(&[Constraint::Length(11), Constraint::Min(20)])
            .column_spacing(1);

        f.render_widget(table, area);
        Ok(())
    }
}

impl Component for PdfDetailComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        Ok(EventState::Consumed)
    }
}
