use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::news::{News, NewsCategories};

use super::{state, ui};

#[derive(Debug)]
pub struct App {
    pub categories: Vec<NewsCategories>,
    pub online: bool,
    pub page: Page,
}

#[derive(Debug)]
pub enum Page {
    Home(HomePage),
    News(News),
}

impl Default for Page {
    fn default() -> Self {
        Page::Home(HomePage::default())
    }
}

#[derive(Debug, Default)]
pub struct HomePage {
    pub selected_index: usize,
    pub selected_news: usize,
}

impl App {
    pub fn new(categories: Vec<NewsCategories>) -> Self {
        Self {
            categories,
            page: Page::default(),
            online: true,
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|f| ui(self, f))?;

        while self.online {
            let event = event::read()?;
            state(self, event);

            terminal.draw(|f| ui(self, f))?;
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}
