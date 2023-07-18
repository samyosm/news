use std::{
    fs::{create_dir_all, File},
    io::BufReader,
};

use chrono::Utc;
use directories::ProjectDirs;

use crate::news::NewsCategories;

#[derive(Debug)]
struct CacheExist(String);

impl std::fmt::Display for CacheExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CacheExist {}

pub fn get_news_categories() -> Result<Vec<NewsCategories>, Box<dyn std::error::Error>> {
    if let Some(categories) = cache_news_categories()? {
        Ok(categories)
    } else {
        let categories = fetch_news_categories()?;
        save_news_categories_to_cache(&categories)?;
        Ok(categories)
    }
}

pub fn cache_news_categories() -> Result<Option<Vec<NewsCategories>>, Box<dyn std::error::Error>> {
    if let Some(directories) = ProjectDirs::from("com", "samyosm", "news") {
        let now = Utc::now().format("%d-%m-%y").to_string();
        // TODO: Create a function for this to be the same everywhere
        let path = directories.cache_dir().join(format!("{}.json", now));

        if !path.is_file() {
            return Ok(None);
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let categories: Vec<NewsCategories> = serde_json::from_reader(reader)?;

        return Ok(Some(categories));
    }

    Ok(None)
}

pub fn save_news_categories_to_cache(
    categories: &Vec<NewsCategories>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(directories) = ProjectDirs::from("com", "samyosm", "news") {
        let now = Utc::now().format("%d-%m-%y").to_string();
        let path = directories.cache_dir().join(format!("{}.json", now));

        if path.is_file() {
            return Err(Box::new(CacheExist("cache file is already present".into())));
        }

        create_dir_all(directories.cache_dir())?;
        let file = File::create(&path)?;
        serde_json::to_writer(file, categories)?;
    }

    Ok(())
}

pub fn fetch_news_categories() -> Result<Vec<NewsCategories>, Box<dyn std::error::Error>> {
    let res = ureq::get("https://news-ie.uk.r.appspot.com/api/v1/today")
        .call()?
        .into_reader();

    let categories: Vec<NewsCategories> = serde_json::from_reader(res)?;

    Ok(categories)
}
