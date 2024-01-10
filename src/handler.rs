use crate::{app::{App, AppResult}, key_sequencer::KeySequencer};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

// create macro for press key event
// accept code only
macro_rules! key_press {
    ($code:pat) => {
        KeyEvent {
            code: $code,
            kind: KeyEventKind::Press,
            ..
        }
    };
    ($code:pat, $modifiers:pat) => {
        KeyEvent {
            code: $code,
            kind: KeyEventKind::Press,
            modifiers: $modifiers,
            ..
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeySequenceEvent {
    Delete,
    NavigateTop,
    Change,
}

pub fn setup_key_sequences(key_sequencer: &mut KeySequencer<KeySequenceEvent>) {
    key_sequencer.register(
        KeyEvent::from(KeyCode::Char('g')),
        KeyEvent::from(KeyCode::Char('g')),
        KeySequenceEvent::NavigateTop,
    );
}

pub fn handle_key_sequence_events(key_sequence_event: KeySequenceEvent, app: &mut App) -> AppResult<()> {
    match key_sequence_event {
        KeySequenceEvent::Delete => {
            app.delete_task();
        }
        KeySequenceEvent::NavigateTop => {
            app.navigate_top();
        }
        KeySequenceEvent::Change => {
            app.change_task();
        }
    }
    Ok(())
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event {
        key_press!(KeyCode::Esc | KeyCode::Char('q')) => {
            app.quit();
        }
        key_press!(KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
            app.quit();
        }
        key_press!(KeyCode::Char('j')) => {
            app.navigate_down();
        }
        key_press!(KeyCode::Char('k')) => {
            app.navigate_up();
        }
        key_press!(KeyCode::Char('h')) => {
            app.toggle_help();
        }
        key_press!(KeyCode::Char('o')) => {
            app.add_task_below();
        }
        key_press!(KeyCode::Char('O')) => {
            app.add_task_above();
        }
        key_press!(KeyCode::Char('x')) => {
            app.toggle_task();
        }
        key_press!(KeyCode::Char('G')) => {
            app.navigate_bottom();
        }
        key_press!(KeyCode::Char('d')) => {
            app.delete_task();
        }
        key_press!(KeyCode::Char('c')) => {
            app.change_task();
        }
        _ => {}
    }
    Ok(())
}
