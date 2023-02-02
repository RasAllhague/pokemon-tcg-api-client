use std::{fs::File, io::Cursor};

use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    builder::QueryBuilder,
    resource::{card::Card, set::Set},
};

use super::error::ApiError;

pub static API_URL: &str = "https://api.pokemontcg.io/v2";
pub static API_KEY_HEADER: &str = "X-Api-Key";

pub struct PokemonApiClient {
    api_key: String,
    client: Client,
}

pub trait ApiResource {
    fn url() -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    data: T,
}

pub struct CardId<'a>(pub &'a str);

pub struct SetId<'a>(pub &'a str);

impl PokemonApiClient {
    pub fn new(api_key: &str) -> Self {
        PokemonApiClient {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_resource<'a, T>(&self, resource_path: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let res = self
            .client
            .get(resource_path)
            .header(API_KEY_HEADER, &self.api_key)
            .send()
            .await
            .map_err(|err| ApiError::Reqwest(err))?;

        let json = res.text().await.map_err(|err| ApiError::Reqwest(err))?;
        let api_response: ApiResponse<T> =
            serde_json::from_str(&json).map_err(|err| ApiError::Deserialize(err))?;

        Ok(api_response.data)
    }

    pub async fn get_queryable_resources<'a, T, Q>(
        &self,
        query_builder: &dyn Fn(Q) -> Q,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned + ApiResource,
        Q: QueryBuilder,
    {
        let builder = Q::new();
        let query_url = query_builder(builder).build(&T::url());

        let res = self
            .client
            .get(query_url)
            .header(API_KEY_HEADER, &self.api_key)
            .send()
            .await
            .map_err(|err| ApiError::Reqwest(err))?;

        let json = res.text().await.map_err(|err| ApiError::Reqwest(err))?;
        let api_response: ApiResponse<T> =
            serde_json::from_str(&json).map_err(|err| ApiError::Deserialize(err))?;

        Ok(api_response.data)
    }

    pub async fn download_image(&self, url: &str, destination: &str) -> Result<(), ApiError> {
        let response = reqwest::get(url)
            .await
            .map_err(|err| ApiError::Reqwest(err))?;

        let mut file = File::create(destination).map_err(|err| ApiError::Io(err))?;
        let mut content = Cursor::new(
            response
                .bytes()
                .await
                .map_err(|err| ApiError::Reqwest(err))?,
        );
        std::io::copy(&mut content, &mut file).map_err(|err| ApiError::Io(err))?;

        Ok(())
    }

    pub async fn get_card(&self, id: CardId<'static>) -> Result<Card, ApiError> {
        let card_url = format!("{}/cards/{}", API_URL, id.0);

        self.get_resource(&card_url).await
    }

    pub async fn get_all_cards(&self) -> Result<Vec<Card>, ApiError> {
        let cards_url = format!("{}/cards", API_URL);

        self.get_resource(&cards_url).await
    }

    pub async fn get_set(&self, id: SetId<'static>) -> Result<Vec<Set>, ApiError> {
        let sets_url = format!("{}/sets/{}", API_URL, id.0);

        self.get_resource(&sets_url).await
    }

    pub async fn get_all_sets(&self) -> Result<Vec<Set>, ApiError> {
        let set_url = format!("{}/sets", API_URL);

        self.get_resource(&set_url).await
    }

    pub async fn get_all_types(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{}/types", API_URL);

        self.get_resource(&types_url).await
    }

    pub async fn get_all_subtype(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{}/subtypes", API_URL);

        self.get_resource(&types_url).await
    }

    pub async fn get_all_supertypes(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{}/supertypes", API_URL);

        self.get_resource(&types_url).await
    }

    pub async fn get_all_rarities(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{}/rarities", API_URL);

        self.get_resource(&types_url).await
    }
}
