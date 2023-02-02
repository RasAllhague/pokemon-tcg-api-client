use serde::{Deserialize, Serialize};

use crate::pokemon_api_client::api_client::API_URL;

use super::{
    common::{Legalities, PokemonType, Resistance, Weakness},
    set::Set,
    ApiResource,
};

/// Subresource for cards in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub supertype: String,
    pub subtypes: Option<Vec<String>>,
    pub hp: Option<String>,
    pub types: Option<Vec<PokemonType>>,
    #[serde(rename = "evolvesFrom")]
    pub evolves_from: Option<String>,
    #[serde(rename = "evolvesTo")]
    pub evolves_to: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
    pub abilities: Option<Vec<Ability>>,
    #[serde(rename = "ancientTrait")]
    pub ancient_trait: Option<AncientTrait>,
    pub attacks: Option<Vec<Attack>>,
    pub weaknesses: Option<Vec<Weakness>>,
    pub resistances: Option<Vec<Resistance>>,
    #[serde(rename = "retreatCost")]
    pub retreat_cost: Option<Vec<PokemonType>>,
    #[serde(rename = "convertedRetreatCost")]
    pub converted_retreat_cost: Option<u32>,
    pub set: Set,
    pub number: String,
    pub artist: Option<String>,
    pub rarity: Option<String>,
    #[serde(rename = "flavorText")]
    pub flavor_text: Option<String>,
    #[serde(rename = "nationalPokedexNumbers")]
    pub national_pokedex_numbers: Option<Vec<u32>>,
    pub legalities: Legalities,
    #[serde(rename = "regulationsMark")]
    pub regulations_mark: Option<String>,
    pub images: Image,
    pub tcgplayer: Option<Tcgplayer>,
    pub cardmarket: Option<Market>,
}

impl ApiResource for Card {
    fn url() -> String {
        format!("{API_URL}/cards")
    }
}

/// Subresource for the ancient traits in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct AncientTrait {
    name: String,
    text: String,
}

/// Subresource for the market infos in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
    url: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    prices: CardmarketPrices,
}

/// Subresource for the cardmarket prices in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct CardmarketPrices {
    #[serde(rename = "averageSellPrice")]
    average_sell_price: Option<f32>,
    #[serde(rename = "lowPrice")]
    low_price: Option<f32>,
    #[serde(rename = "trendPrice")]
    trend_price: Option<f32>,
    #[serde(rename = "germanProLow")]
    german_pro_low: Option<f32>,
    #[serde(rename = "suggestedPrice")]
    suggested_price: Option<f32>,
    #[serde(rename = "reverseHoloSell")]
    reverse_holo_sell: Option<f32>,
    #[serde(rename = "reverseHoloLow")]
    reverse_holo_low: Option<f32>,
    #[serde(rename = "reverseHoloTrend")]
    reverse_holo_trend: Option<f32>,
    #[serde(rename = "lowPriceExPlus")]
    low_price_ex_plus: Option<f32>,
    #[serde(rename = "avg1")]
    average_day: Option<f32>,
    #[serde(rename = "avg7")]
    average_week: Option<f32>,
    #[serde(rename = "avg9")]
    average_month: Option<f32>,
    #[serde(rename = "reverseHoloAvg1")]
    reverse_holo_avg1: Option<f32>,
    #[serde(rename = "reverseHoloAvg7")]
    reverse_holo_avg7: Option<f32>,
    #[serde(rename = "reverseHoloAvg30")]
    reverse_holo_avg30: Option<f32>,
}

/// Subresource for the images of cards in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub small: String,
    pub large: String,
}

/// Subresource for the for tcg player data in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Tcgplayer {
    url: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    prices: Option<TcgplayerPrices>,
}

/// Subresource for the tcg player price list in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct TcgplayerPrices {
    normal: Option<TcgplayerPrice>,
    holofoil: Option<TcgplayerPrice>,
    #[serde(rename = "1stEditionHolofoil")]
    first_edition_holofoil: Option<TcgplayerPrice>,
    #[serde(rename = "1stEditionNormal")]
    first_edition_normal: Option<TcgplayerPrice>,
}

/// Subresource for the tcg player prices in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct TcgplayerPrice {
    low: f32,
    mid: f32,
    high: f32,
    market: Option<f32>,
    direct_low: Option<f32>,
}

/// Subresource for the attacks in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    name: String,
    cost: Vec<PokemonType>,
    #[serde(rename = "convertedEnergyCost")]
    converted_energy_cost: u32,
    damage: String,
    text: String,
}

/// Subresource for the abilities in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    name: String,
    text: String,
    #[serde(rename = "type")]
    ability_type: String,
}
