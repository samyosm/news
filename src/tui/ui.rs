use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use super::app::App;

pub fn ui(app: &mut App) -> Result<(), io::Error> {
    app.terminal.draw(draw_home)?;

    Ok(())
}

fn draw_home(f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::BOTTOM);
    f.render_widget(block, chunks[0]);

    let block = Block::default();
    f.render_widget(block, chunks[1]);
}
