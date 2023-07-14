use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsCategories {
    pub name: String,
    pub news: Vec<News>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    #[serde(rename = "_id")]
    pub id: String,
    pub source: Option<String>,
    pub author: Option<String>,
    pub url: String,
    pub title: String,
    pub preview: Option<String>,
    pub content: String,
}
