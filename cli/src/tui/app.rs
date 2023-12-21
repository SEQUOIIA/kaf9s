use std::convert::TryFrom;
use std::error;
use std::time::Duration;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use crate::tui::event::{Event, EventHandler};
use crate::tui::term::Term;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub term: Term,
    pub context : AppContext
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            term: Term::start().unwrap(),
            context: AppContext {
                tab_index: 0,
                row_index: 0
            }
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn handle_events(&mut self, event_handler: &EventHandler) -> AppResult<()> {
        match event_handler.next()? {
            Event::Key(key) => self.handle_key_event(key),
            Event::Resize(width, height) => {
                self.term.resize(Rect::new(0, 0, width, height)).unwrap();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> AppResult<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        let context = &mut self.context;
        const TAB_COUNT: usize = 3;

        if let Some(num) = key.code.to_number() {
            if num > 0 && num <= TAB_COUNT as i8 {
                context.tab_index = usize::try_from(num).unwrap() - 1;
                context.row_index = 0;
            }
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            KeyCode::Tab | KeyCode::BackTab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                let tab_index = context.tab_index + TAB_COUNT; // to wrap around properly
                context.tab_index = tab_index.saturating_sub(1) % TAB_COUNT;
                context.row_index = 0;
            }
            KeyCode::Tab | KeyCode::BackTab => {
                context.tab_index = context.tab_index.saturating_add(1) % TAB_COUNT;
                context.row_index = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                context.row_index = context.row_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                context.row_index = context.row_index.saturating_add(1);
            }
            _ => {}
        };
        Ok(())
    }
}

trait KeyCodeExtended {
    fn to_number(self) -> Option<i8>;
}

impl KeyCodeExtended for KeyCode {
    fn to_number(self) -> Option<i8> {
        match self {
            KeyCode::Char('1') => Some(1),
            KeyCode::Char('2') => Some(2),
            KeyCode::Char('3') => Some(3),
            KeyCode::Char('4') => Some(4),
            KeyCode::Char('5') => Some(5),
            KeyCode::Char('6') => Some(6),
            KeyCode::Char('7') => Some(7),
            KeyCode::Char('8') => Some(8),
            KeyCode::Char('9') => Some(9),
            KeyCode::Char('0') => Some(0),
            _ => None
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
    pub tab_index: usize,
    pub row_index: usize,
}