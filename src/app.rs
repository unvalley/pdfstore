use log::{debug, warn};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::actions::{Action, Actions};
use crate::inputs::key::Key;
use crate::state::AppState;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}
pub struct App {
    /// Contextual actions
    actions: Actions,
    state: AppState,
    tab: usize,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::initialized();
        Self {
            actions,
            state,
            tab: 0,
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> anyhow::Result<()> {
        let size = f.size();
        self.check_size(&size);

        let chunks_main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(size);

        let chunks_pdfs = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks_main[0]);

        let renamed_pdfs = self.draw_body(false, self.state());
        f.render_widget(renamed_pdfs, chunks_pdfs[0]);

        let original_pdfs = self.draw_body(false, self.state());
        f.render_widget(original_pdfs, chunks_pdfs[1]);

        let pdf_details = self.draw_pdf_details(self.actions());
        f.render_widget(pdf_details, chunks_main[1]);

        Ok(())
    }

    fn check_size(&self, rect: &Rect) {
        if rect.width < 52 {
            panic!("Require width >= 52, (got {})", rect.width);
        }
        if rect.height < 28 {
            panic!("Require height >= 28, (got {})", rect.height);
        }
    }

    fn draw_pdf_details(&self, actions: &Actions) -> Table {
        let key_style = Style::default().fg(Color::LightCyan);
        let help_style = Style::default().fg(Color::Gray);

        let mut rows = vec![];
        for action in actions.actions().iter() {
            let mut first = true;
            for key in action.keys() {
                let pdf_details = if first {
                    first = false;
                    action.to_string()
                } else {
                    String::from("")
                };
                let row = Row::new(vec![
                    Cell::from(Span::styled(key.to_string(), key_style)),
                    Cell::from(Span::styled(pdf_details, help_style)),
                ]);
                rows.push(row);
            }
        }

        Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title("Details"),
            )
            .widths(&[Constraint::Length(11), Constraint::Min(20)])
            .column_spacing(1)
    }

    pub fn draw_body<'a>(&self, loading: bool, state: &AppState) -> Paragraph<'a> {
        let loading_text = if loading { "Loading..." } else { "" };
        let tick_text = if let Some(ticks) = state.count_tick() {
            format!("Tick count: {}", ticks)
        } else {
            String::default()
        };

        Paragraph::new(vec![
            Spans::from(Span::raw(loading_text)),
            Spans::from(Span::raw(tick_text)),
        ])
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain)
                .title("PDF Files"),
        )
    }

    /// Handle a user action
    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    /// We could update the app or dispatch event on tick
    pub fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
        AppReturn::Continue
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }
}
