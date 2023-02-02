use serde::{Serialize, Deserialize};

use crate::pokemon_api_client::api_client::{API_URL, ApiResource};

use super::common::Legalities;


#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    id: String,
    name: String,
    series: String,
    #[serde(rename = "printedTotal")]
    printed_total: u32,
    total: u32,
    legalities: Legalities,
    #[serde(rename = "ptcgoCode")]
    ptcgo_code: Option<String>,
    #[serde(rename = "releaseDate")]
    release_date: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

impl ApiResource for Set {
    fn url() -> String {
        format!("{}/sets", API_URL)
    }
}

impl ApiResource for Vec<Set> {
    fn url() -> String {
        format!("{}/sets", API_URL)
    }
}