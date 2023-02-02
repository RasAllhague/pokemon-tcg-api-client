use std::collections::HashMap;

use crate::pokemon_api_client::api_client::CardId;

use super::{Ordering, QueryBuilder};

pub struct PokemonQueryBuilder {
    filters: HashMap<String, String>,
    page: Option<u32>,
    page_size: Option<u8>,
    order_by: Vec<Ordering>,
    select_fields: Vec<String>,
}

impl PokemonQueryBuilder {
    pub fn with_page_size(mut self, size: u8) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn add_ordering(mut self, ordering: Ordering) -> Self {
        self.order_by.push(ordering);
        self
    }

    pub fn add_select(mut self, field: &str) -> Self {
        self.select_fields.push(String::from(field));
        self
    }

    pub fn add_id(self, id: &CardId) -> Self {
        self.add_or_update_filter("id", &id.0)
    }

    pub fn add_name(self, name: &str) -> Self {
        self.add_or_update_filter("name", name)
    }

    pub fn add_sub_types(self, sub_type: &str) -> Self {
        self.add_or_update_filter("subtypes", sub_type)
    }

    pub fn add_hp_range(self, low_value: &str, high_value: &str, is_inclusive: bool) -> Self {
        if is_inclusive {
            return self.add_or_update_filter("hp", &format!("[{low_value} TO {high_value}]"));
        }

        self.add_or_update_filter("hp", &format!("{{{low_value} TO {high_value}}}"))
    }

    pub fn add_types(self, types: &str) -> Self {
        self.add_or_update_filter("types", types)
    }

    pub fn add_evolves_from(self, evolves_from: &str) -> Self {
        self.add_or_update_filter("evolvesFrom", evolves_from)
    }

    pub fn add_evolves_to(self, evolves_to: &str) -> Self {
        self.add_or_update_filter("evolvesTo", evolves_to)
    }

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

    pub fn add_set_name(self, set_name: &str) -> Self {
        self.add_or_update_filter("set.series", set_name)
    }

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
