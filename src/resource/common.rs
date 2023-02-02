use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Legalities {
    standard: Option<String>,
    unlimited: Option<String>,
    expanded: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Images {
    symbol: String,
    logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weakness {
    #[serde(rename = "type")]
    weakness_type: PokemonType,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resistance {
    #[serde(rename = "type")]
    resistance_type: PokemonType,
    value: String,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub enum Supertype {
    Energy,
    Pokemon,
    Trainer,
}
