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
        KeyCode::Tab => {
            // TODO: Enhance this
            app.selected_index += 1;
            if app.selected_index > 3 {
                app.selected_index = 0
            }
        }
        KeyCode::BackTab => {
            // TODO: Enhance this
            if app.selected_index == 0 {
                app.selected_index = 3
            } else {
                app.selected_index -= 1
            }
        }
        _ => {}
    }
}
