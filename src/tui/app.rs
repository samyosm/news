use std::io;

use crossterm::event;
use derivative::Derivative;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::news::NewsCategories;

use super::{state, ui};

type CrossBackend = Terminal<CrosstermBackend<io::Stdout>>;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct App<'a> {
    categories: Vec<NewsCategories<'a>>,
    #[derivative(Debug = "ignore")]
    pub terminal: &'a mut CrossBackend,
    pub selected_index: usize,
    pub online: bool,
}

impl<'a> App<'a> {
    pub fn new(categories: Vec<NewsCategories<'a>>, terminal: &'a mut CrossBackend) -> Self {
        Self {
            terminal,
            categories,
            selected_index: 0,
            online: true,
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        ui(self)?;

        while self.online {
            let event = event::read()?;
            state(self, event);
        }

        Ok(())
    }
}
