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
        let help_style = Style::default().fg(Color::Gray);

        let mut rows = vec![];
        let row = Row::new(vec![Cell::from(Span::styled("AAA".to_string(), key_style))]);
        rows.push(row);
        // for action in self.actions.actions().iter() {
        //     let mut first = true;
        //     for key in action.keys() {
        //         let pdf_details = if first {
        //             first = false;
        //             action.to_string()
        //         } else {
        //             String::from("")
        //         };
        //         let row = Row::new(vec![
        //             Cell::from(Span::styled(key.to_string(), key_style)),
        //             Cell::from(Span::styled(pdf_details, help_style)),
        //         ]);
        //         rows.push(row);
        //     }
        // }

        let table = Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title("Details"),
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
