use log::{debug, warn};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
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
    tab: usize,
    do_quit: bool,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(key_config: KeyConfig) -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::initialized();
        // let key_config = Rc::new(key_config);
        Self {
            actions,
            state,
            inbox: InboxComponent::new(key_config.clone()),
            pdf_import_popup: PdfImportPopup::new(),
            focus: Focus::Inbox,
            key_config,
            tab: 0,
            do_quit: false,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> anyhow::Result<()> {
        let size = f.size();
        self.check_size(&size);

        let chunks_main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        self.inbox
            .draw(f, chunks_main[0], matches!(self.focus, Focus::Inbox))?;
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

        if self.component_focus(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        if self.move_main_focus(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        Ok(EventState::NotConsumed)
    }

    /// handling focus to each component
    pub fn move_main_focus(&mut self, key: Key) -> anyhow::Result<EventState> {
        self.focus = Focus::Inbox;
        Ok(EventState::Consumed)
    }

    /// handling focus in each component
    pub fn component_focus(&mut self, key: Key) -> anyhow::Result<EventState> {
        match self.focus {
            Focus::Inbox => {
                if self.inbox.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed);
                }
                return Ok(EventState::Consumed);
            }
            Focus::Search => return Ok(EventState::Consumed),
        }
        Ok(EventState::NotConsumed)
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
