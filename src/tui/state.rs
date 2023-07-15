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
        KeyCode::Tab | KeyCode::Char('l') => {
            app.selected_news = 0;
            // TODO: Enhance this
            app.selected_index += 1;
            if app.selected_index > app.categories.len() - 1 {
                app.selected_index = 0
            }
        }
        KeyCode::BackTab | KeyCode::Char('h') => {
            app.selected_news = 0;
            // TODO: Enhance this
            if app.selected_index == 0 {
                app.selected_index = app.categories.len() - 1
            } else {
                app.selected_index -= 1
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.selected_news += 1;
            if app.selected_news > app.categories.get(app.selected_index).unwrap().news.len() - 1 {
                app.selected_news = 0;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_news == 0 {
                app.selected_news = app.categories.get(app.selected_index).unwrap().news.len() - 1
            } else {
                app.selected_news -= 1
            }
        }
        _ => {}
    }
}
