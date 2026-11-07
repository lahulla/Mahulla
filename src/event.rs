
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Action {
    Up,
    Down,
    First,
    Last,
    Open,
    Back,
    PageUp,
    PageDown,
    Quit,
    None,
}

pub fn next_action(timeout: std::time::Duration) -> std::io::Result<Action> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                let action = match key.code {
                    KeyCode::Char('q') => Action::Quit,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        Action::Quit
                    }
                    KeyCode::Char('j') | KeyCode::Down => Action::Down,
                    KeyCode::Char('k') | KeyCode::Up => Action::Up,
                    KeyCode::Char('g') => Action::First,
                    KeyCode::Char('G') => Action::Last,
                    KeyCode::Enter | KeyCode::Char('l') => Action::Open,
                    KeyCode::Esc | KeyCode::Char('h') => Action::Back,
                    KeyCode::PageUp => Action::PageUp,
                    KeyCode::PageDown => Action::PageDown,
                    _ => Action::None,
                };
                return Ok(action);
            }
        }
    }
    Ok(Action::None)
}
