use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Article {
    pub blocks: Vec<ArticleItem>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum ArticleItem {
    Header { level: u8, text: String },
    Paragraph { text: String },
}
