#![feature(iter_intersperse)]
mod fetch;
mod news;
mod tui;

use fetch::get_news_categories;
use tui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let categories = get_news_categories()?;
    let mut app = App::new(categories);
    app.start()?;
    Ok(())
}
