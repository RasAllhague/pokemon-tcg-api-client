use std::env;

use pokemon_tcg_api_client::{pokemon_api_client::{error::ApiError, api_client::{PokemonApiClient, ApiResource, API_URL, CardId}}, builder::pokemon::PokemonQueryBuilder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BulbId {
    id: String,
}

impl ApiResource for BulbId {
    fn url() -> String {
        format!("{}/cards", API_URL)
    }
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let api_token = env::var("POKEMON_TCG_API_TOKEN").expect("Expected a api token in the environment.");
    let api_client = PokemonApiClient::new(&api_token);
    
    let bulbs_ids = api_client.get_queryable_resources::<Vec<BulbId>, PokemonQueryBuilder>(&|builder| builder
        .add_name("bulb*")
        .add_select("id")
        .add_select("name")
        .with_page_size(10)
    ).await?;

    for bulb_id in bulbs_ids {
        let bulb = api_client.get_card(CardId(bulb_id.id)).await?;

        println!("Id: {}, name: {}, image: {}, desc: {:?}", bulb.id, bulb.name, bulb.images.small, bulb.flavor_text);
    }

    Ok(())
}