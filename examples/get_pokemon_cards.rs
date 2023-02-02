use std::env;

use pokemon_tcg_api_client::{
    builder::pokemon::PokemonQueryBuilder,
    pokemon_api_client::{api_client::PokemonApiClient, error::ApiError},
    resource::card::Card,
};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let api_token =
        env::var("POKEMON_TCG_API_TOKEN").expect("Expected a api token in the environment.");
    let api_client = PokemonApiClient::new(&api_token);

    let bulbs = api_client
        .get_queryable_resources::<Vec<Card>, PokemonQueryBuilder>(&|builder| {
            builder.add_name("bulb*").with_page_size(10)
        })
        .await?;

    for bulb in bulbs {
        println!(
            "Id: {}, name: {}, image: {}, desc: {:?}",
            bulb.id, bulb.name, bulb.images.small, bulb.flavor_text
        );
    }

    Ok(())
}
