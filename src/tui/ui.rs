use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::app::{App, Page};

pub fn ui(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    match &app.page {
        Page::Home(_) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
                .split(f.size());

            header(app, f, chunks[0]);
            body(app, f, chunks[1]);
        }
        Page::News(news) => {
            let block = Block::default()
                .title(news.title.clone())
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(news.content.clone()).block(block);
            f.render_widget(paragraph, f.size())
        }
    }
}

pub fn body(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    if let Page::Home(home) = &app.page {
        let items: Vec<ListItem> = app
            .categories
            .get(home.selected_index)
            .unwrap()
            .news
            .iter()
            .map(|news| {
                let title = Line::from(Span::styled(
                    news.title.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ));

                let mut text = Text::from(title);
                if let Some(preview) = news.preview.clone() {
                    let preview_line =
                        Line::from(Span::styled(preview, Style::default().fg(Color::White)));
                    text.extend(vec![preview_line])
                }

                let mut info_spans = vec![];

                if let Some(source) = news.source.clone() {
                    let source_span = Span::styled(
                        format!("Source: {}", source),
                        Style::default().fg(Color::Green),
                    );
                    info_spans.push(source_span)
                }

                if let Some(author) = news.author.clone() {
                    let source_span = Span::styled(
                        format!("Author: {}", author),
                        Style::default().fg(Color::Red),
                    );
                    info_spans.push(source_span)
                }

                let info_spans: Vec<Span> = info_spans
                    .into_iter()
                    .intersperse(Span::raw(" • "))
                    .collect();

                text.extend(vec![info_spans]);
                text.extend(vec!["\n"]);

                ListItem::new(text)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let mut state = ListState::default().with_selected(Some(home.selected_news));
        f.render_stateful_widget(list, area, &mut state);
    }
}

pub fn header(app: &App, f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
    if let Page::Home(home) = &app.page {
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
                    .style(if home.selected_index == index {
                        highlight_style
                    } else {
                        Style::new()
                    })
                    .block(general.clone());
                f.render_widget(paragraph, header[index]);
            });
    }
}
