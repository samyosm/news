#[derive(Debug)]
pub struct NewsCategories<'a> {
    name: &'a str,
    news: Vec<News<'a>>,
}

#[derive(Debug)]
pub struct News<'a> {
    source: &'a str,
    title: &'a str,
    url: &'a str,
    author: &'a str,
    preview: &'a str,
    content: &'a str,
}
