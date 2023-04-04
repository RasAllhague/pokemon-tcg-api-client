use std::collections::HashMap;

use crate::pokemon_api_client::api_client::CardId;

use super::{Ordering, QueryBuilder};

/// Query builder for trainer cards.
#[derive(Clone)]
pub struct TrainerQueryBuilder {
    filters: HashMap<String, String>,
    page: Option<u32>,
    page_size: Option<u8>,
    order_by: Vec<Ordering>,
    select_fields: Vec<String>,
}

impl TrainerQueryBuilder {
    /// Sets the page size of the query builder.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the page.
    #[must_use]
    pub fn with_page_size(self, size: u8) -> Self {
        let mut new = self.clone();
        new.page_size = Some(size);
        new
    }

    /// Sets the page from which it should get the results.
    ///
    /// # Arguments
    ///
    /// * `page` - The number of the page to select.
    #[must_use]
    pub fn with_page(self, page: u32) -> Self {
        let mut new = self.clone();
        new.page = Some(page);
        new
    }

    /// Adds a parameter to the ordering list.
    ///
    /// # Arguments
    ///
    /// * `ordering` - A fields to order by in the data.
    #[must_use]
    pub fn add_ordering(self, ordering: Ordering) -> Self {
        let mut new = self.clone();
        new.order_by.push(ordering);
        new
    }

    /// Adds a value to select from the requested data.
    /// The returned data will only contain values with those fields.
    ///
    /// # Arguments
    ///
    /// * `field` - A field to at to the query.
    #[must_use]
    pub fn add_select(self, field: &str) -> Self {
        let mut new = self.clone();
        new.select_fields.push(String::from(field));
        new
    }

    /// Adds a id to the query parameter, if used more than once it turns into an OR.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the trainer card you want to filter for.
    #[must_use]
    pub fn add_id(self, id: &CardId) -> Self {
        self.add_or_update_filter("id", &id.0)
    }

    /// Adds a card name to the query parameter, if used more than once it turns into an OR.
    /// Look at Pokemon TCG Api wiki <https://docs.pokemontcg.io/api-reference/cards/search-cards/> for a documentation about wildcards.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the trainer card or the wildcard name you want to query for.
    #[must_use]
    pub fn add_name(self, name: &str) -> Self {
        self.add_or_update_filter("name", name)
    }

    /// Adds a card set series to the query parameter, if used more than once it turns into an OR.
    /// Look at Pokemon TCG Api wiki <https://docs.pokemontcg.io/api-reference/cards/search-cards/> for a documentation about wildcards.
    ///
    /// # Arguments
    ///
    /// * `set_series` - The set series of the card you want to query for.
    #[must_use]
    pub fn add_set_series(self, set_series: &str) -> Self {
        self.add_or_update_filter("set.series", set_series)
    }

    fn add_or_update_filter(mut self, key: &str, value: &str) -> Self {
        if let Some(old_value) = self.filters.get_mut(key) {
            *old_value = format!("{old_value},{value}");
            return self;
        }

        self.filters.insert(String::from(key), String::from(value));
        self
    }
}

impl QueryBuilder for TrainerQueryBuilder {
    fn new() -> Self {
        TrainerQueryBuilder {
            filters: HashMap::new(),
            page: None,
            page_size: None,
            order_by: Vec::new(),
            select_fields: Vec::new(),
        }
    }

    fn page(&self) -> Option<u32> {
        self.page
    }

    fn page_size(&self) -> Option<u8> {
        self.page_size
    }

    fn filters(&self) -> HashMap<String, String> {
        self.filters.clone()
    }

    fn order_by(&self) -> Vec<Ordering> {
        self.order_by.clone()
    }

    fn select_fields(&self) -> Vec<String> {
        self.select_fields.clone()
    }
}
