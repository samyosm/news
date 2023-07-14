use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use derivative::Derivative;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::news::NewsCategories;

use super::{state, ui};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct App {
    pub categories: Vec<NewsCategories>,
    pub selected_index: usize,
    pub selected_news: usize,
    pub online: bool,
}

impl App {
    pub fn new(categories: Vec<NewsCategories>) -> Self {
        Self {
            categories,
            selected_index: 0,
            selected_news: 0,
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
