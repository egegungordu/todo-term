use crossterm::event::{KeyCode, KeyEvent};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use todo_term::app::{App, AppMode, AppResult};
use todo_term::event::{Event, EventHandler};
use todo_term::handler::{handle_key_events, handle_normal_mode_sequence_key_events, setup_key_sequences};
use todo_term::key_sequencer::KeySequencer;
use todo_term::tui::Tui;

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(30);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    let mut key_sequencer = KeySequencer::default();

    setup_key_sequences(&mut key_sequencer);

    while app.is_running() {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {
                app.tick();
                key_sequencer.tick();
            }
            Event::Key(key_event) => match (app.get_mode(), key_sequencer.feed(key_event)) {
                (AppMode::Normal, Some(key_sequence_event)) => {
                    handle_normal_mode_sequence_key_events(key_sequence_event, &mut app)?;
                }
                _ => {
                    handle_key_events(key_event, &mut app)?;
                }
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
