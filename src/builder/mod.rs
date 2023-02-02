pub mod energy;
pub mod pokemon;

use std::{collections::HashMap, fmt::Display};

#[derive(Clone)]
pub enum Ordering {
    Ascending(String),
    Descending(String),
}

pub trait QueryBuilder {
    fn new() -> Self;
    fn page(&self) -> Option<u32>;
    fn page_size(&self) -> Option<u8>;
    fn filters(&self) -> HashMap<String, String>;
    fn order_by(&self) -> Vec<Ordering>;
    fn select_fields(&self) -> Vec<String>;
    fn build(&self, url: &str) -> String {
        let url = url;
        let mut query_index = url.find(|x| x == '?');

        let mut builder = String::from(url);

        if let Some(page_size) = self.page_size() {
            build_query_param(&mut builder, &mut query_index, "pageSize", page_size);
        }
        if let Some(page) = self.page() {
            build_query_param(&mut builder, &mut query_index, "page", page);
        }

        let order_by = self.order_by();

        if order_by.len() != 0 {
            todo!()
        }

        let fields = self.select_fields();

        if fields.len() != 0 {
            let select_query = fields
                .iter()
                .map(|x| format!("{},", x))
                .collect::<Vec<String>>()
                .concat()
                .trim_end_matches(',')
                .to_owned();

            build_query_param(&mut builder, &mut query_index, "select", select_query);
        }

        let filters = self.filters();

        if filters.len() != 0 {
            builder += match query_index {
                Some(_) => "&",
                None => "?",
            };
            builder += &urlencoding::encode("q");
            builder += "=";
            query_index = None;

            build_filter_query(query_index, &mut builder, &filters);
        }

        builder
    }
}

fn build_filter_query(
    mut query_index: Option<usize>,
    builder: &mut String,
    filters: &HashMap<String, String>,
) {
    for (filter_key, filter_value) in filters {
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
        } else {
            *builder += &urlencoding::encode(&filter_key);
            *builder += ":";
            *builder += &urlencoding::encode(&enclose_whitespace_strings(&filter_value));
        }

        query_index = Some(0)
    }
}

fn build_query_param<T: Display>(
    builder: &mut String,
    query_index: &mut Option<usize>,
    key: &str,
    value: T,
) {
    *builder += match *query_index {
        Some(_) => "&",
        None => "?",
    };
    *builder += &urlencoding::encode(key);
    *builder += "=";
    *builder += &urlencoding::encode(&value.to_string());
    *query_index = Some(0);
}

fn enclose_whitespace_strings(value: &str) -> String {
    if value.chars().any(|x| x.is_whitespace()) {
        return format!("\"{}\"", value);
    }

    return value.to_string();
}
