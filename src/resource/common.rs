use serde::{Deserialize, Serialize};

/// Subresource for the legalities in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Legalities {
    standard: Option<String>,
    unlimited: Option<String>,
    expanded: Option<String>,
}

/// Subresource for the Images in the api.
#[derive(Serialize, Deserialize)]
pub struct Images {
    symbol: String,
    logo: String,
}

/// Subresource for the weaknesses in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Weakness {
    #[serde(rename = "type")]
    weakness_type: PokemonType,
    value: String,
}

/// Subresource for the resistances in the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct Resistance {
    #[serde(rename = "type")]
    resistance_type: PokemonType,
    value: String,
}

/// Subresource for the pokemon types in the api.
#[derive(Serialize, Deserialize, Debug)]
pub enum PokemonType {
    Colorless,
    Grass,
    Fire,
    Water,
    Lightning,
    Fighting,
    Psychic,
    Darkness,
    Metal,
    Dragon,
    Fairy,
}

/// Subresource for the supertypes in the api.
#[derive(Serialize, Deserialize, Debug)]
pub enum Supertype {
    Energy,
    Pokemon,
    Trainer,
}
