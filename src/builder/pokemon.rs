use std::collections::HashMap;

use crate::pokemon_api_client::api_client::CardId;

use super::{Ordering, QueryBuilder};

/// Query builder for pokemon card api requests.
pub struct PokemonQueryBuilder {
    filters: HashMap<String, String>,
    page: Option<u32>,
    page_size: Option<u8>,
    order_by: Vec<Ordering>,
    select_fields: Vec<String>,
}

impl PokemonQueryBuilder {
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
    /// * `id` - The id of the pokemon card you want to filter for.
    #[must_use]
    pub fn add_id(self, id: &CardId) -> Self {
        self.add_or_update_filter("id", &id.0)
    }

    /// Adds a card name to the query parameter, if used more than once it turns into an OR.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the pokemon card you want to filter for.
    #[must_use]
    pub fn add_name(self, name: &str) -> Self {
        self.add_or_update_filter("name", name)
    }

    /// Adds a sub type to the query parameter, if used more than once it turns into an OR.
    /// 
    /// # Arguments
    /// 
    /// * `sub_types` - The sub type you want to query for.
    #[must_use]
    pub fn add_sub_types(self, sub_type: &str) -> Self {
        self.add_or_update_filter("subtypes", sub_type)
    }

    /// Adds a hp range to the query parameter. Can be inclusive or exclusive.
    /// 
    /// # Arguments
    /// 
    /// * `low_value` - The low value of the hp range.
    /// * `high_value` - The high value of the hp range.
    /// * `is_inclusive` - Sets whether the range is inclusive.
    #[must_use]
    pub fn add_hp_range(self, low_value: &str, high_value: &str, is_inclusive: bool) -> Self {
        if is_inclusive {
            return self.add_or_update_filter("hp", &format!("[{low_value} TO {high_value}]"));
        }

        self.add_or_update_filter("hp", &format!("{{{low_value} TO {high_value}}}"))
    }

    /// Adds a type to the query parameter.
    /// 
    /// # Arguments
    /// 
    /// * `types` - The type of to add to the query.
    #[must_use]
    pub fn add_types(self, types: &str) -> Self {
        self.add_or_update_filter("types", types)
    }

    /// Adds a pokemon name the searched pokemon evolves from to the query parameter.
    /// 
    /// # Arguments
    /// 
    /// * `evolves_from` - The evolved from pokemon to add to the query.
    #[must_use]
    pub fn add_evolves_from(self, evolves_from: &str) -> Self {
        self.add_or_update_filter("evolvesFrom", evolves_from)
    }

    /// Adds a pokemon name the searched pokemon evolves tp to the query parameter.
    /// 
    /// # Arguments
    /// 
    /// * `evolves_to` - The evolved to pokemon to add to the query.
    #[must_use]
    pub fn add_evolves_to(self, evolves_to: &str) -> Self {
        self.add_or_update_filter("evolvesTo", evolves_to)
    }

    /// Adds a attack cost range to the query parameter. Can be inclusive or exclusive.
    /// 
    /// # Arguments
    /// 
    /// * `low_value` - The low value of the attack range.
    /// * `high_value` - The high value of the attack range.
    /// * `is_inclusive` - Sets whether the range is inclusive.
    #[must_use]
    pub fn add_attack_cost_range(
        self,
        low_value: &str,
        high_value: &str,
        is_inclusive: bool,
    ) -> Self {
        if is_inclusive {
            return self.add_or_update_filter(
                "attacks.convertedEnergyCost",
                &format!("[{low_value} TO {high_value}]"),
            );
        }

        self.add_or_update_filter(
            "attacks.convertedEnergyCost",
            &format!("{{{low_value} TO {high_value}}}"),
        )
    }

    /// Adds a set series to the query parameter, if used more than once it turns into an OR.
    /// Look at [Pokemon TCG Api wiki] (https://docs.pokemontcg.io/api-reference/cards/search-cards/) for a documentation about wildcards.
    /// 
    /// # Arguments
    /// 
    /// * `set_series` - The name of set series of the card you want to query for.
    #[must_use]
    pub fn add_set_name(self, set_name: &str) -> Self {
        self.add_or_update_filter("set.series", set_name)
    }

    /// Adds a rarity to the query.
    /// 
    /// # Arguments
    /// 
    /// * `rarity` - The rarity of the pokemon card.
    #[must_use]
    pub fn add_rarity(self, rarity: &str) -> Self {
        self.add_or_update_filter("rarity", rarity)
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

impl QueryBuilder for PokemonQueryBuilder {
    fn new() -> Self {
        PokemonQueryBuilder {
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
