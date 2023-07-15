#![feature(iter_intersperse)]
mod news;
mod tui;

use news::NewsCategories;
use std::{
    fs::File,
    io::{self, BufReader},
};
use tui::App;

fn main() -> Result<(), io::Error> {
    // Load from cache or fetch
    let file = File::open("/home/samy/news/src/test.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let categories: Vec<NewsCategories> = serde_json::from_reader(reader)?;

    let mut app = App::new(categories);
    app.start()?;
    Ok(())
}
