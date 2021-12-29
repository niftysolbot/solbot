use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlphaArtResponse {
    // pub collection: Collection,
    // pub traits: Vec<Trait>,
    pub floor_price: String,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Collection {
//     pub id: String,
//     pub slug: String,
//     pub description: String,
//     pub authority_pubkey: String,
//     pub title: String,
//     pub thumbnail: String,
//     pub banner: String,
//     pub links: Vec<Value>,
//     pub total_items: i64,
//     pub verified: bool,
//     pub symbol: String,
//     pub owner_count: i64,
//     pub volume: String,
//     pub alternative_authorities: Vec<String>,
//     pub collaborators: Vec<Value>,
//     pub added_at: String,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Trait {
//     pub key: String,
//     pub numbers: Vec<Number>,
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Number {
//     pub value: String,
//     pub amount: i64,
//     pub floor: Option<String>,
// }