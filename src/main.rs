mod fetch;
mod news;
mod tui;

use tui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.start()?;
    Ok(())
}
