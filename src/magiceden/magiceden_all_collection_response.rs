use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MagicEdenAllCollectionsResponse {
    pub collections: Vec<Collection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub symbol: String,
    // #[serde(default)]
    // pub candy_machine_ids: Vec<String>,
    pub name: String,
    // pub image: String,
    // pub description: String,
    // pub created_at: String,
    // pub enabled_attributes_filters: Option<bool>,
    // pub is_draft: Option<bool>,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub discord: Option<String>,
    // pub volume_all: f64,
    // pub total_items: Option<i64>,
    // pub flag_message: Option<String>,
    // pub is_flagged: Option<bool>,
}