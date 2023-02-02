use std::env;

use pokemon_tcg_api_client::{pokemon_api_client::{
    api_client::{PokemonApiClient},
}, builder::pokemon::PokemonQueryBuilder, resource::card::Card};

#[tokio::main]
async fn main() {
    env_logger::init();
    let api_token =
        env::var("POKEMON_TCG_API_TOKEN").expect("Expected a api token in the environment.");

    let client = PokemonApiClient::new(&api_token);
    let cards = client
        .get_queryable_resources::<Vec<Card>, PokemonQueryBuilder>(&|builder| {
            builder
                .add_name("char*")
                .with_page_size(10)
        })
        .await
        .unwrap();

    for card in cards {
        println!("{}, {}", card.name, card.images.large);
        client.download_image(&card.images.small, &format!("./tests/{}_{}.png", card.name, card.id)).await.unwrap()
    }
}
