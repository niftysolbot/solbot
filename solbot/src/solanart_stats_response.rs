use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanartResponse {
    pub floor_price: f64,
    #[serde(rename = "count_listed")]
    pub count_listed: i64,
    pub count_total: i64,
}
