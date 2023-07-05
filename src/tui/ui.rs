use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());
    let block = Block::default().borders(Borders::BOTTOM);
    f.render_widget(block, chunks[0]);
    let block = Block::default();
    f.render_widget(block, chunks[1]);
}
