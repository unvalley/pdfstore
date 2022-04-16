use std::cell::RefCell;
use std::rc::Rc;
use std::io::stdout;
use std::time::Duration;
use app::{App, AppReturn};
use inputs::events::Events;
use inputs::InputEvent;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::app::ui;

mod version;
pub mod app;
pub mod inputs;

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
