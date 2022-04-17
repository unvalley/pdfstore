use inputs::events::Events;
use inputs::InputEvent;
use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub mod inputs;
pub mod app;
pub mod version;
pub mod state;
pub mod actions;
pub mod ui;

use crate::app::{App, AppReturn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: How to use Rc & RefCell
    let app = Rc::new(RefCell::new(App::new()));

    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();
        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;
        // Handle inputs
        let result = match events.next()? {
            // lets process that event
            InputEvent::Input(key) => app.do_action(key),
            // handle no user input
            InputEvent::Tick => app.update_on_tick(),
        };
        // Check if we should exit
        if result == AppReturn::Exit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
