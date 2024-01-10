use std::collections::HashMap;

use crossterm::event::{KeyEvent, KeyEventKind, KeyEventState};

trait KeySequencerFilter {
    fn filter(&self, key_event: KeyEvent) -> Option<KeyEvent>;
}

struct DefaultKeySequencerFilter;

impl KeySequencerFilter for DefaultKeySequencerFilter {
    fn filter(&self, key_event: KeyEvent) -> Option<KeyEvent> {
        match key_event {
            KeyEvent {
                kind: KeyEventKind::Press,
                ..
            } => Some(key_event),
            _ => None,
        }
    }
}

pub struct KeySequencer<T: Clone> {
    sequences: HashMap<(KeyEvent, KeyEvent), T>,
    last_key: Option<KeyEvent>,
    tick_count: u32,
    reset_tick_count: u32,
    filter: Box<dyn KeySequencerFilter>,
}

impl<T: Clone> Default for KeySequencer<T> {
    fn default() -> Self {
        Self {
            sequences: HashMap::new(),
            last_key: None,
            tick_count: 0,
            reset_tick_count: 10,
            filter: Box::new(DefaultKeySequencerFilter),
        }
    }
}

impl<T: Clone> KeySequencer<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        if self.tick_count >= self.reset_tick_count {
            self.last_key = None;
            self.tick_count = 0;
        }
    }

    pub fn feed(&mut self, key_event: KeyEvent) -> Option<T> {
        if let None = self.filter.filter(key_event) {
            return None;
        }

        self.tick_count = 0;
        let last_key = self.last_key;
        self.last_key = Some(key_event);

        if let Some(last_key) = last_key {
            if let Some(event) = self.sequences.get(&(last_key, key_event)) {
                self.last_key = None;
                return Some(event.clone());
            }
        }

        None
    }

    pub fn register(&mut self, key1: KeyEvent, key2: KeyEvent, event: T) {
        self.sequences.insert((key1, key2), event);
    }
}
