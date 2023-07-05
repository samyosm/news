use crossterm::event::{Event, KeyCode, KeyEvent};

use super::app::App;

pub fn state(app: &mut App, event: Event) {
    match event {
        Event::Key(event) => key_event(app, event),
        _ => {}
    }
}

fn key_event(app: &mut App, event: KeyEvent) {
    match event.code {
        KeyCode::Char('q') => app.online = false,
        _ => {}
    }
}
