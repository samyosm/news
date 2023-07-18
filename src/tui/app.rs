use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, widgets::ScrollbarState, Terminal};

use crate::{
    fetch::get_news_categories,
    news::{News, NewsCategories},
};

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
    News(NewsPage),
    Message(String),
}

impl Default for Page {
    fn default() -> Self {
        Page::Home(HomePage::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct HomePage {
    pub selected_index: usize,
    pub selected_news: usize,
}

#[derive(Debug, Default)]
pub struct NewsPage {
    pub news: News,
    pub scroll_position: u16,
    pub scroll_state: ScrollbarState,
    pub home_page: HomePage,
}

impl App {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            page: Page::Message("Please wait while we fetch your tasks.\nIt should be noted that this might take a while.".to_string()),
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

        let categories = get_news_categories().expect("couldn't fetch news categories");
        self.categories = categories;
        self.page = Page::Home(HomePage::default());

        while self.online {
            terminal.draw(|f| ui(self, f))?;

            // TODO: Make const
            if crossterm::event::poll(Duration::from_millis(250))? {
                let event = event::read()?;
                state(self, event);
            }
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
