use serde::Deserialize;

pub type DigitalEyesAllCollectionResponse = Vec<Root>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    // #[serde(rename = "24h_sales")]
    // pub n24h_sales: i64,
    // pub is_nsfw: bool,
    // pub description: String,
    pub website: Option<String>,
    // pub volume_past24h: i64,
    // pub volume_last_updated_at: String,
    pub name: String,
    // pub thumbnail: String,
    // pub volume_past7days: i64,
    // pub volume_total: i64,
    // pub disputed_message: String,
    // pub is_curated: bool,
    // pub published_epoch: i64,
}

