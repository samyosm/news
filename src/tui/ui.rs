use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::app::App;

pub fn ui(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
        .split(f.size());

    header(app, f, chunks[0]);
    body(app, f, chunks[1]);
}

pub fn body(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    let items: Vec<ListItem> = app
        .categories
        .get(app.selected_index)
        .unwrap()
        .news
        .iter()
        .map(|news| ListItem::new(news.title.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    let mut state = ListState::default().with_selected(Some(app.selected_news));
    f.render_stateful_widget(list, area, &mut state);
}

pub fn header(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    let tab_count = app.categories.len();
    let tab_width = 100 / tab_count;
    let contraints = vec![Constraint::Percentage(tab_width as u16); tab_count];

    let header = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(contraints)
        .split(area);

    let general = Block::default().borders(Borders::BOTTOM);
    let highlight_style = Style::new().fg(Color::LightBlue);

    app.categories
        .iter()
        .enumerate()
        .for_each(|(index, category)| {
            let paragraph = Paragraph::new(category.name.clone())
                .alignment(Alignment::Center)
                .style(if app.selected_index == index {
                    highlight_style
                } else {
                    Style::new()
                })
                .block(general.clone());
            f.render_widget(paragraph, header[index]);
        });
}
