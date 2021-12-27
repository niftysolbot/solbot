use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanartResponse {
    // TODO: Uncomment below for items, but needs dynamic parsing
    // pub items: Vec<Item>,
    pub pagination: Pagination,
}

// TODO: Uncomment below for items, but needs dynamic parsing
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Item {
//     pub id: Option<i64>,
//     #[serde(rename = "token_add")]
//     pub token_add: String,
//     pub price: f64,
//     #[serde(rename = "link_img")]
//     pub link_img: String,
//     #[serde(rename = "for_sale")]
//     pub for_sale: i64,
//     pub name: String,
//     pub escrow_add: String,
//     #[serde(rename = "seller_address")]
//     pub seller_address: String,
//     pub attributes: String,
//     pub skin: Value,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     #[serde(rename = "attrib_count")]
//     pub attrib_count: i64,
//     pub buyer_add: Value,
//     #[serde(rename = "bidder_address")]
//     pub bidder_address: Option<String>,
//     pub current_bid: Option<i64>,
//     pub last_sold_price: Option<f64>,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub current_page: i64,
    pub per_page: i64,
    pub next_page: i64,
    pub max_pages: i64,
    pub max_items: i64,
    #[serde(rename = "Owners")]
    pub owners: i64,
    pub floor_price_filters: f64,
    pub max_price_filters: i64,
}
