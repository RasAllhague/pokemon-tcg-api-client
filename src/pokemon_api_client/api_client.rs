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

/// Api client for querying the pokemon tcg api.
pub struct PokemonApiClient {
    api_key: String,
    client: Client,
}

/// Trait for using the api client.
pub trait ApiResource {
    fn url() -> String;
}

/// Container for api responses. Contains data of the generic type T.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    data: T,
}

/// Wrapper for card ids.
pub struct CardId(pub String);

/// Wrapper for set ids.
pub struct SetId(pub String);

impl PokemonApiClient {
    /// Creates a new instance of the `PokemonApiClient`.
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - The to the pokemon tcg api.
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        PokemonApiClient {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    /// Gets results from the api based on a resource path (url).
    /// 
    /// # Arguments
    /// 
    /// * `resource_path` - The part of the url of the resource you want to query.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
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
            .map_err(ApiError::Reqwest)?;

        let json = res.text().await.map_err(ApiError::Reqwest)?;
        let api_response: ApiResponse<T> =
            serde_json::from_str(&json).map_err( ApiError::Deserialize)?;

        Ok(api_response.data)
    }

    /// Gets results from the api based on a `QueryBuilder`.
    /// 
    /// # Arguments
    /// 
    /// * `query_builder` - The query builder which creates the query parameters.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
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
            .map_err(ApiError::Reqwest)?;

        let json = res.text().await.map_err(ApiError::Reqwest)?;
        let api_response: ApiResponse<T> =
            serde_json::from_str(&json).map_err(ApiError::Deserialize)?;

        Ok(api_response.data)
    }

    /// Downloads a image from the api to the designated destination.
    /// 
    /// # Arguments
    /// 
    /// * `url` - The url of the image to download from the api.
    /// * `destination` - The destination where to save the image to.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if the file cannot be downloaded or not be saved to the file system.
    pub async fn download_image(&self, url: &str, destination: &str) -> Result<(), ApiError> {
        let response = reqwest::get(url)
            .await
            .map_err(ApiError::Reqwest)?;

        let mut file = File::create(destination).map_err(ApiError::Io)?;
        let mut content = Cursor::new(
            response
                .bytes()
                .await
                .map_err(ApiError::Reqwest)?,
        );
        std::io::copy(&mut content, &mut file).map_err(ApiError::Io)?;

        Ok(())
    }

    /// Gets a card set from the api based on its id.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The id of the set to retrieve.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_card(&self, id: CardId) -> Result<Card, ApiError> {
        let card_url = format!("{API_URL}/cards/{}", id.0);

        self.get_resource(&card_url).await
    }

    /// Gets a list of all cards from the api.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_all_cards(&self) -> Result<Vec<Card>, ApiError> {
        let cards_url = format!("{API_URL}/cards");

        self.get_resource(&cards_url).await
    }

    /// Gets a specific set from the api based on its id.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The id of the set to retrieve.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_set(&self, id: SetId) -> Result<Set, ApiError> {
        let sets_url = format!("{API_URL}/sets/{}", id.0);

        self.get_resource(&sets_url).await
    }

    /// Gets a list of all sets from the api.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_all_sets(&self) -> Result<Vec<Set>, ApiError> {
        let set_url = format!("{API_URL}/sets");

        self.get_resource(&set_url).await
    }

    /// Gets a list of all types from the api.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_all_types(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{API_URL}/types");

        self.get_resource(&types_url).await
    }

    /// Gets a list of all subtypes from the api.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_all_subtype(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{API_URL}/subtypes");

        self.get_resource(&types_url).await
    }

    /// Gets a list of all supertypes from the api.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_all_supertypes(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{API_URL}/supertypes");

        self.get_resource(&types_url).await
    }

    /// Gets a list of all rarities from the api.
    /// 
    /// # Errors
    /// 
    /// Will return `Err` if an error occures during either api querying or json parsing.
    pub async fn get_all_rarities(&self) -> Result<Vec<String>, ApiError> {
        let types_url = format!("{API_URL}/rarities");

        self.get_resource(&types_url).await
    }
}
