use crate::{app::{App, AppResult, AppMode}, key_sequencer::KeySequencer};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

macro_rules! n_key_press {
    ($code:pat) => {
        (KeyEvent {
            code: $code,
            kind: KeyEventKind::Press,
            ..
        }, AppMode::Normal)
    };
    ($code:pat, $modifiers:pat) => {
        (KeyEvent {
            code: $code,
            kind: KeyEventKind::Press,
            modifiers: $modifiers,
            ..
        }, AppMode::Normal)
    };
}

macro_rules! i_key_press {
    ($code:pat) => {
        (KeyEvent {
            code: $code,
            kind: KeyEventKind::Press,
            ..
        }, AppMode::Insert)
    };
    ($code:pat, $modifiers:pat) => {
        (KeyEvent {
            code: $code,
            kind: KeyEventKind::Press,
            modifiers: $modifiers,
            ..
        }, AppMode::Insert)
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeySequenceEvent {
    NavigateTop,
}

pub fn setup_key_sequences(key_sequencer: &mut KeySequencer<KeySequenceEvent>) {
    key_sequencer.register(
        KeyEvent::from(KeyCode::Char('g')),
        KeyEvent::from(KeyCode::Char('g')),
        KeySequenceEvent::NavigateTop,
    );
}

pub fn handle_normal_mode_sequence_key_events(key_sequence_event: KeySequenceEvent, app: &mut App) -> AppResult<()> {
    match key_sequence_event{
        KeySequenceEvent::NavigateTop=> {
            app.navigate_top();
        }
    }
    Ok(())
}

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event, app.get_mode()) {
        n_key_press!(KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
            app.quit();
        }
        n_key_press!(KeyCode::Esc | KeyCode::Char('q')) => {
            app.quit();
        }
        n_key_press!(KeyCode::Char('j')) => {
            app.navigate_down();
        }
        n_key_press!(KeyCode::Char('k')) => {
            app.navigate_up();
        }
        n_key_press!(KeyCode::Char('h')) => {
            app.toggle_help();
        }
        n_key_press!(KeyCode::Char('o')) => {
            app.add_task_below();
            app.enter_insert_mode();
        }
        n_key_press!(KeyCode::Char('O')) => {
            app.add_task_above();
            app.enter_insert_mode();
        }
        n_key_press!(KeyCode::Char('x')) => {
            app.toggle_task();
        }
        n_key_press!(KeyCode::Char('G')) => {
            app.navigate_bottom();
        }
        n_key_press!(KeyCode::Char('d')) => {
            app.delete_task();
        }
        n_key_press!(KeyCode::Char('c')) => {
            app.reset_task();
            app.enter_insert_mode();
        }
        n_key_press!(KeyCode::Char('a')) => {
            app.enter_insert_mode();
        }
        n_key_press!(KeyCode::Char('y')) => {
            app.yank_task();
        }
        n_key_press!(KeyCode::Char('p')) => {
            app.paste_task_below();
        }
        n_key_press!(KeyCode::Char('P')) => {
            app.paste_task_above();
        }
        i_key_press!(KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
            app.exit_insert_mode();
        }
        i_key_press!(KeyCode::Esc) => {
            app.exit_insert_mode();
        }
        i_key_press!(KeyCode::Char(c)) => {
            app.append_to_task(c);
        }
        i_key_press!(KeyCode::Backspace) => {
            app.pop_from_task();
        }
        _ => {}
    }
    Ok(())
}
