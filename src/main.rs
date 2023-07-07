mod news;
mod tui;

use std::io;
use tui::App;

fn main() -> Result<(), io::Error> {
    let mut app = App::new(vec![]);
    app.start()?;
    Ok(())
}
