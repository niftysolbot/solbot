use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlphaArtAllCollectionResponse {
    pub collections: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    // pub id: String,
    pub slug: String,
    // pub description: Option<String>,
    // pub authority_pubkey: String,
    pub title: String,
    // pub thumbnail: String,
    // pub banner: Option<String>,
    // pub links: Vec<String>,
    // pub total_items: i64,
    // pub verified: bool,
    // pub symbol: Option<String>,
    // pub owner_count: i64,
    // pub volume: Option<String>,
    // pub alternative_authorities: Vec<String>,
    // pub collaborators: Vec<String>,
    // pub added_at: String,
}