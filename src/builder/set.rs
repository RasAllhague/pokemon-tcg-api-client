use std::collections::HashMap;

use crate::pokemon_api_client::api_client::SetId;

use super::{Ordering, QueryBuilder};

/// Query builder for creating set api query urls.
pub struct SetQueryBuilder {
    filters: HashMap<String, String>,
    page: Option<u32>,
    page_size: Option<u8>,
    order_by: Vec<Ordering>,
    select_fields: Vec<String>,
}

impl SetQueryBuilder {
    /// Sets the page size of the query builder.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the page.
    #[must_use]
    pub fn with_page_size(mut self, size: u8) -> Self {
        self.page_size = Some(size);
        self
    }

    /// Sets the page from which it should get the results.
    ///
    /// # Arguments
    ///
    /// * `page` - The number of the page to select.
    #[must_use]
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Adds a parameter to the ordering list.
    ///
    /// # Arguments
    ///
    /// * `ordering` - A fields to order by in the data.
    #[must_use]
    pub fn add_ordering(mut self, ordering: Ordering) -> Self {
        self.order_by.push(ordering);
        self
    }

    /// Adds a value to select from the requested data.
    /// The returned data will only contain values with those fields.
    ///
    /// # Arguments
    ///
    /// * `field` - A field to at to the query.
    #[must_use]
    pub fn add_select(mut self, field: &str) -> Self {
        self.select_fields.push(String::from(field));
        self
    }

    /// Adds a id to the query parameter, if used more than once it turns into an OR.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the set you want to filter for.
    #[must_use]
    pub fn add_id(self, id: &SetId) -> Self {
        self.add_or_update_filter("id", &id.0)
    }

    /// Adds a set name to the query parameter, if used more than once it turns into an OR.
    /// Look at Pokemon TCG Api wiki <https://docs.pokemontcg.io/api-reference/cards/search-cards/> for a documentation about wildcards.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the set or the wildcard name you want to query for.
    #[must_use]
    pub fn add_name(self, name: &str) -> Self {
        self.add_or_update_filter("name", name)
    }

    /// Adds a sub series to the query parameter, if used more than once it turns into an OR.
    /// Look at Pokemon TCG Api wiki <https://docs.pokemontcg.io/api-reference/cards/search-cards/> for a documentation about wildcards.
    ///
    /// # Arguments
    ///
    /// * `series` - The name of the sub series or the wildcard name you want to query for.
    #[must_use]
    pub fn add_sub_series(self, series: &str) -> Self {
        self.add_or_update_filter("series", series)
    }

    /// Adds a ptcgo code to the query parameter, if used more than once it turns into an OR.
    /// Look at Pokemon TCG Api wiki <https://docs.pokemontcg.io/api-reference/cards/search-cards/> for a documentation about wildcards.
    ///
    /// # Arguments
    ///
    /// * `series` - The ptcgo code you want to query for.
    #[must_use]
    pub fn add_ptcgo_code(self, ptcgo_code: &str) -> Self {
        self.add_or_update_filter("ptcgoCode", ptcgo_code)
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

impl QueryBuilder for SetQueryBuilder {
    fn new() -> Self {
        SetQueryBuilder {
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
