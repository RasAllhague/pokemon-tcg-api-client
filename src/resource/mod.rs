use crate::pokemon_api_client::api_client::ApiResource;

pub mod card;
pub mod common;
pub mod set;

impl<T: ApiResource> ApiResource for Vec<T> {
    fn url() -> String {
        T::url()
    }
}