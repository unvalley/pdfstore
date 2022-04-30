use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::key_config::KeyConfig;
use crate::state::AppState;
use crate::{
    actions::{Action, Actions},
    components::{inbox::InboxComponent, DrawableComponent},
};
use crate::{
    components::{Component, EventState},
    inputs::key::Key,
};

use crate::components::pdf_import_popup::PdfImportPopup;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

enum Focus {
    Inbox,
    Search,
}

/// if you want to need feature or screen, add it Focus and App
pub struct App {
    /// Contextual actions
    actions: Actions,
    state: AppState,
    inbox: InboxComponent,
    pdf_import_popup: PdfImportPopup,
    focus: Focus,
    pub key_config: KeyConfig,
    do_quit: bool,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(key_config: KeyConfig) -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::initialized();

        Self {
            actions,
            state,
            inbox: InboxComponent::new(key_config.clone()),
            pdf_import_popup: PdfImportPopup::new(),
            focus: Focus::Inbox,
            key_config,
            do_quit: false,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> anyhow::Result<()> {
        let size = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        self.inbox
            .draw(f, chunks_main[0], matches!(self.focus, Focus::Inbox))?;
        Ok(())
    }

    fn check_quit(&mut self, key: Key) -> bool {
        if key == self.key_config.quit || key == self.key_config.exit {
            self.do_quit = true;
            return true;
        }
        false
    }

    pub fn is_quit(&self) -> bool {
        self.do_quit
    }

    pub async fn event(&mut self, key: Key) -> anyhow::Result<EventState> {
        log::trace!("event: {:?}", key.clone());
        if self.check_quit(key) {
            return Ok(EventState::NotConsumed);
        }

        if self.components_event(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        if self.focus_components(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        if self.focus_inbox(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        Ok(EventState::NotConsumed)
    }

    /// handling focus to each component
    pub fn focus_inbox(&mut self, key: Key) -> anyhow::Result<EventState> {
        self.focus = Focus::Inbox;
        Ok(EventState::Consumed)
    }

    /// handling focus in each component
    pub fn focus_components(&mut self, key: Key) -> anyhow::Result<EventState> {
        match self.focus {
            Focus::Inbox => {
                if self.inbox.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed);
                }
            }
            Focus::Search => return Ok(EventState::Consumed),
        }
        Ok(EventState::NotConsumed)
    }

    pub async fn update_inbox_list(&mut self) -> anyhow::Result<()> {
        self.inbox.update().await?;
        Ok(())
    }

    pub async fn components_event(&mut self, key: Key) -> anyhow::Result<EventState> {
        // match self.focus {
        //     Focus::Inbox => {
        //         self.update_inbox_list().await?;
        //         return Ok(EventState::Consumed);
        //     }
        //     Focus::Search => todo!(),
        // }
        Ok(EventState::NotConsumed)
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }
}
