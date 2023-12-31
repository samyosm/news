use std::io;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Margin,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::Title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation,
    },
    Frame,
};

use super::app::{App, NewsPage, Page};

pub fn ui(app: &mut App, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    match &mut app.page {
        Page::Home(_) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
                .split(f.size());

            header(app, f, chunks[0]);
            body(app, f, chunks[1]);
        }
        Page::News(NewsPage {
            news,
            scroll_state,
            scroll_position,
            ..
        }) => {
            let block = Block::default()
                .title(Title::from(news.title.blue().bold()).alignment(Alignment::Center))
                .title(
                    Title::from("j/↓ to go down, k/↑ to go up, esc to go back, q to quit".blue())
                        .alignment(Alignment::Left),
                )
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(news.content.clone())
                .block(block)
                .scroll((*scroll_position, 0));
            f.render_widget(paragraph, f.size());

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalLeft)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                f.size().inner(&Margin {
                    vertical: 10,
                    horizontal: 10,
                }),
                scroll_state,
            );
        }
        Page::Message(message) => {
            let block = Block::default()
                .title(Title::from("Message").alignment(Alignment::Center))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let paragraph = Paragraph::new(Text::from(message.as_str()))
                .block(block)
                .alignment(Alignment::Center);

            f.render_widget(paragraph, f.size());
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
                let color = if news.seen {
                    Some(Color::DarkGray)
                } else {
                    None
                };
                let title = Line::from(Span::styled(
                    news.title.clone(),
                    match news.seen {
                        true => Style::default().fg(color.unwrap()),
                        false => Style::default().add_modifier(Modifier::BOLD),
                    },
                ));

                let mut text = Text::from(title);
                if let Some(preview) = news.preview.clone() {
                    let preview_line = Line::from(Span::styled(
                        preview,
                        Style::default().fg(color.unwrap_or(Color::White)),
                    ));
                    text.extend(vec![preview_line])
                }

                let mut info_spans = vec![];

                if let Some(source) = news.source.clone() {
                    let source_span = Span::styled(
                        format!("Source: {}", source),
                        Style::default().fg(color.unwrap_or(Color::Green)),
                    );
                    info_spans.push(source_span)
                }

                if let Some(author) = news.author.clone() {
                    let source_span = Span::styled(
                        format!("Author: {}", author),
                        Style::default().fg(color.unwrap_or(Color::Red)),
                    );
                    info_spans.push(source_span)
                }

                let info_spans: Vec<Span> =
                    itertools::intersperse(info_spans, Span::raw(" • ")).collect();

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
        let constraints = vec![Constraint::Percentage(tab_width as u16); tab_count];

        let header = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
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
