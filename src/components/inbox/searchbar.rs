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

pub struct SearchbarComponent {
    key_config: KeyConfig,
}

impl SearchbarComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            key_config: key_config.clone(),
        }
    }
}

impl DrawableComponent for SearchbarComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        focused: bool,
    ) -> anyhow::Result<()> {

        let body = Paragraph::new(vec![
            Spans::from(Span::raw("Search")),
        ])
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain)
        );

        f.render_widget(body, area);

        Ok(())
    }
}

impl Component for SearchbarComponent {
    fn commands(&self) {}

    fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        Ok(EventState::Consumed)
    }
}
