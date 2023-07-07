use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::App;

pub fn ui(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
        .split(f.size());

    let header = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[0]);

    let general = Block::default().borders(Borders::BOTTOM);
    let highlight_style = Style::new().fg(Color::LightBlue);

    let general_paragraph = Paragraph::new("general")
        .alignment(Alignment::Center)
        .style(if app.selected_index == 0 {
            highlight_style
        } else {
            Style::new()
        })
        .block(general.clone());
    f.render_widget(general_paragraph, header[0]);

    let technology_paragraph = Paragraph::new("technology")
        .alignment(Alignment::Center)
        .style(if app.selected_index == 1 {
            highlight_style
        } else {
            Style::new()
        })
        .block(general.clone());
    f.render_widget(technology_paragraph, header[1]);

    let busines_paragraph = Paragraph::new("business")
        .alignment(Alignment::Center)
        .style(if app.selected_index == 2 {
            highlight_style
        } else {
            Style::new()
        })
        .block(general.clone());
    f.render_widget(busines_paragraph, header[2]);

    let science_paragraph = Paragraph::new("science")
        .alignment(Alignment::Center)
        .style(if app.selected_index == 3 {
            highlight_style
        } else {
            Style::new()
        })
        .block(general.clone());
    f.render_widget(science_paragraph, header[3]);

    let block = Block::default();
    f.render_widget(block, chunks[1]);
}
