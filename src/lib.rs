//! Api client crate for querying the [https://pokemontcg.io/] API.
//!
//! Provides an abstraction over the api requests. 
//! Can either query by id or resources a bit more specific.
//! 
//! Features:
//! - Central api client
//! - Methods for all api paths
//! - Query builders for the advanced querying of data 
//!     - Pokemon cards
//!     - Sets
//!     - Trainer cards
//!     - Energies
pub mod builder;
pub mod pokemon_api_client;
pub mod resource;
