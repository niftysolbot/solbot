use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DigitalEyesResponse {
    // pub count: i64,
    // #[serde(rename = "next_cursor")]
    // pub next_cursor: Value,
    // pub offers: Vec<Offer>,
    #[serde(rename = "price_floor")]
    pub price_floor: i64,
}

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Offer {
//     pub add_epoch: i64,
//     pub bump: Option<i64>,
//     pub collection: String,
//     pub contract: String,
//     pub creators: Vec<Creator>,
//     #[serde(rename = "is_nft")]
//     pub is_nft: Option<bool>,
//     pub metadata: Metadata,
//     pub mint: String,
//     pub offer_name: String,
//     pub owner: String,
//     pub pk: String,
//     pub price: i64,
//     pub tags: Vec<String>,
//     pub uri: String,
//     pub verifeyed: bool,
//     pub last_price: Option<i64>,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Creator {
//     #[serde(rename = "Address")]
//     pub address: String,
//     #[serde(rename = "Share")]
//     pub share: i64,
//     #[serde(rename = "Verified")]
//     pub verified: bool,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Metadata {
//     pub attributes: Vec<Attribute>,
//     #[serde(rename = "background_color")]
//     pub background_color: String,
//     pub description: String,
//     pub edition: String,
//     #[serde(rename = "external_url")]
//     pub external_url: String,
//     pub image: String,
//     pub name: String,
//     pub properties: Properties,
//     #[serde(rename = "seller_fee_basis_points")]
//     pub seller_fee_basis_points: i64,
//     pub symbol: String,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Attribute {
//     #[serde(rename = "trait_type")]
//     pub trait_type: String,
//     // pub value: Value,
//     #[serde(rename = "display_type")]
//     pub display_type: Option<String>,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Properties {
//     pub category: String,
//     pub creators: Vec<Creator2>,
//     pub files: Vec<File>,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Creator2 {
//     pub address: String,
//     pub share: i64,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct File {
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub uri: String,
// }
