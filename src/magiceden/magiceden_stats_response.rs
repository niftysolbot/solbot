use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MagicEdenResponse {
    pub results: Results,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub symbol: String,
    pub enabled_attributes_filters: bool,
    pub available_attributes: Vec<AvailableAttribute>,
    pub floor_price: i64,
    pub listed_count: i64,
    pub listed_total_value: i64,
    pub avg_price24hr: Option<f64>,
    pub volume24hr: Option<i64>,
    pub volume_all: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAttribute {
    pub count: i64,
    pub floor: i64,
    // pub attribute: Attribute,
}

// TODO: Uncomment this and the Attribute on line 28 above to parse through different attributes
// TODO: Need custom parser because the attributes can be either integer or String
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Attribute {
//     #[serde(rename = "trait_type")]
//     pub trait_type: String,
//     pub value: String,
// }
