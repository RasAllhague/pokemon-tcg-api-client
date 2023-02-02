use std::collections::HashMap;

use crate::pokemon_api_client::api_client::CardId;

use super::{Ordering, enclose_whitespace_strings, QueryBuilder, build_query_param};

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

    pub fn add_id(self, id: CardId) -> Self {
        self.add_or_update_filter("id", id.0)
    }

    pub fn add_name(self, name: &str) -> Self {
        self.add_or_update_filter("name", name)
    }

    pub fn add_sub_types(self, sub_type: &str) -> Self {
        self.add_or_update_filter("subtypes", sub_type)
    }

    pub fn add_hp_range(self, low_value: &str, high_value: &str, is_inclusive: bool) -> Self {
        if is_inclusive {
            return self.add_or_update_filter("hp", &format!("[{} TO {}]", low_value, high_value));
        }
        
        self.add_or_update_filter("hp", &format!("{{{} TO {}}}", low_value, high_value))
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

    pub fn add_attack_cost_range(self, low_value: &str, high_value: &str, is_inclusive: bool) -> Self {
        if is_inclusive {
            return self.add_or_update_filter("attacks.convertedEnergyCost", &format!("[{} TO {}]", low_value, high_value));
        }
        
        self.add_or_update_filter("attacks.convertedEnergyCost", &format!("{{{} TO {}}}", low_value, high_value))
    }

    pub fn add_set_name(self, set_name: &str) -> Self {
        self.add_or_update_filter("set.series", set_name)
    }

    pub fn add_rarity(self, rarity: &str) -> Self {
        self.add_or_update_filter("rarity", rarity)
    }

    fn add_or_update_filter(mut self, key: &str, value: &str) -> Self {
        if let Some(old_value) = self.filters.get_mut(key) {
            *old_value = format!("{},{}", old_value, value);
            return self;
        }

        self.filters.insert(String::from(key), String::from(value));
        self
    }

    fn build_filter_query(self, mut query_index: Option<usize>, builder: &mut String) {
        for (filter_key, filter_value) in self.filters {
            if query_index.is_some() {
                *builder += " ";
            }

            let split = filter_value.split(',').collect::<Vec<&str>>();

            if split.len() > 0 {
                for sub_split in split.iter() {
                    *builder += &urlencoding::encode(&filter_key);
                    *builder += ":";
                    *builder += &urlencoding::encode(&enclose_whitespace_strings(&sub_split));

                    if let Some(last) = split.last() {
                        if sub_split != last {
                            *builder += &urlencoding::encode(" or ");
                        }
                    }
                }
            }
            else {
                *builder += &urlencoding::encode(&filter_key);
                *builder += ":";
                *builder += &urlencoding::encode(&enclose_whitespace_strings(&filter_value));
            }

            query_index = Some(0)
        }
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

    fn build(self, url: &str) -> String {
        let url = url;
        let mut query_index = url.find(|x| x == '?');

        let mut builder = String::from(url);

        if let Some(page_size) = self.page_size {
            build_query_param(&mut builder, &mut query_index, "pageSize", page_size);
        }
        if let Some(page) = self.page {
            build_query_param(&mut builder, &mut query_index, "page", page);
        }
        if let Some(ordering) = self.page {
            build_query_param(&mut builder, &mut query_index, "orderBy", ordering);
        }
        if self.select_fields.len() != 0 {
            let select_query  = self.select_fields
            .iter()
            .map(|x| format!("{},", x))
            .collect::<Vec<String>>()
            .concat()
            .trim_end_matches(',')
            .to_owned();
            
            build_query_param(&mut builder, &mut query_index, "select", select_query);
        }
        
        if self.filters.len() != 0 {
            builder += match query_index {
                Some(_) => "&",
                None => "?",
            };
            builder += &urlencoding::encode("q");
            builder += "=";
            query_index = None;

            self.build_filter_query(query_index, &mut builder);
        }

        builder
    }
}