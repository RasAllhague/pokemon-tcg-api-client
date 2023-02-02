//! Contains the most common data structs for the api resources

pub mod card;
pub mod common;
pub mod set;


/// Trait for using the api client.
pub trait ApiResource {
    fn url() -> String;
}

impl<T: ApiResource> ApiResource for Vec<T> {
    fn url() -> String {
        T::url()
    }
}
