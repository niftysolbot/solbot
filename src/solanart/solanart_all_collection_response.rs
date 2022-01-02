use serde::Deserialize;

pub type SolanartAllCollectionResponse = Vec<Root>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    // pub update_auth: String,
    // #[serde(rename = "attributes_trait_types")]
    // pub attributes_trait_types: String,
    // #[serde(rename = "attributes_filters")]
    // pub attributes_filters: String,
    // #[serde(rename = "attributes_values")]
    // pub attributes_values: String,
    // pub dw: i64,
    // pub nb: i64,
    // pub creators: String,
    // pub symbol: String,
    // pub id: i64,
    pub url: String,
    pub urlsoon: Option<String>,
    pub name: String,
    // pub description: String,
    // pub display: String,
    // pub new: String,
    // pub soon: String,
    // pub trending: String,
    // pub date: i64,
    // pub supply: i64,
    // pub regionfix: String,
    pub twitter: Option<String>,
    pub website: Option<String>,
    // pub img: String,
    pub discord: Option<String>,
    // pub imgpreview: String,
    // #[serde(rename = "min_count")]
    // pub min_count: i64,
    // #[serde(rename = "max_count")]
    // pub max_count: i64,
    // pub hyb: i64,
    // pub gb: i64,
    // pub attrib: i64,
    // pub cdn: i64,
}