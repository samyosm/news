use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::news::News;

use super::app::{App, Page};

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
            if let Page::Home(home) = &mut app.page {
                home.selected_news = 0;
                // TODO: Enhance this
                home.selected_index += 1;
                if home.selected_index > app.categories.len() - 1 {
                    home.selected_index = 0
                }
            }
        }
        KeyCode::BackTab | KeyCode::Char('h') => {
            if let Page::Home(home) = &mut app.page {
                home.selected_news = 0;
                // TODO: Enhance this
                if home.selected_index == 0 {
                    home.selected_index = app.categories.len() - 1
                } else {
                    home.selected_index -= 1
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Page::Home(home) = &mut app.page {
                home.selected_news += 1;
                // TODO: not using unwrap
                if home.selected_news
                    > app.categories.get(home.selected_index).unwrap().news.len() - 1
                {
                    home.selected_news = 0;
                }
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Page::Home(home) = &mut app.page {
                if home.selected_news == 0 {
                    home.selected_news =
                        app.categories.get(home.selected_index).unwrap().news.len() - 1
                } else {
                    home.selected_news -= 1
                }
            }
        }
        KeyCode::Enter => {
            if let Page::Home(home) = &mut app.page {
                let news = app.categories[home.selected_index].news[home.selected_news].clone();
                app.page = Page::News(news)
            }
        }
        KeyCode::Esc => {
            if let Page::News(_) = &mut app.page {
                app.page = Page::default()
            }
        }

        _ => {}
    }
}
