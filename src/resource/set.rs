use serde::{Deserialize, Serialize};

use crate::pokemon_api_client::api_client::{ApiResource, API_URL};

use super::common::Legalities;

/// `ApiResource` for sets.
#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    pub id: String,
    pub name: String,
    pub series: String,
    #[serde(rename = "printedTotal")]
    pub printed_total: u32,
    pub total: u32,
    pub legalities: Legalities,
    #[serde(rename = "ptcgoCode")]
    pub ptcgo_code: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl ApiResource for Set {
    fn url() -> String {
        format!("{API_URL}/sets")
    }
}
