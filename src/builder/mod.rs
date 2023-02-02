//! Contains the implementation for the query builders used for advanced querying to the api.
pub mod energy;
pub mod pokemon;
pub mod set;
pub mod trainer;

use std::{collections::HashMap, fmt::Display};

/// Container for the ordering of query fields of the results.
#[derive(Clone)]
pub enum Ordering {
    Ascending(String),
    Descending(String),
}

/// Trait for implementing query builders for the api client.
pub trait QueryBuilder {
    /// Creates a new instance of the query builder.
    fn new() -> Self;
    /// Returns the current page the query builder is set to.
    fn page(&self) -> Option<u32>;
    /// Returns the current page size the query builder is set to.
    fn page_size(&self) -> Option<u8>;
    /// Returns the filters set for the query builder.
    fn filters(&self) -> HashMap<String, String>;
    /// Returns the ordering information of the query builder.
    fn order_by(&self) -> Vec<Ordering>;
    /// Returns the selected fields for the returning json of the query builder.
    fn select_fields(&self) -> Vec<String>;

    /// Builds and returns the query url for the api request.
    ///
    /// * `url` - The base url of the query url.
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

        if !order_by.is_empty() {
            todo!()
        }

        let fields = self.select_fields();

        if !fields.is_empty() {
            let select_query = fields
                .iter()
                .map(|x| format!("{x},"))
                .collect::<Vec<String>>()
                .concat()
                .trim_end_matches(',')
                .to_owned();

            build_query_param(&mut builder, &mut query_index, "select", select_query);
        }

        let filters = self.filters();

        if !filters.is_empty() {
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

        if split.is_empty() {
            *builder += &urlencoding::encode(filter_key);
            *builder += ":";
            *builder += &urlencoding::encode(&enclose_whitespace_strings(filter_value));
        }

        for sub_split in &split {
            *builder += &urlencoding::encode(filter_key);
            *builder += ":";
            *builder += &urlencoding::encode(&enclose_whitespace_strings(sub_split));

            if let Some(last) = split.last() {
                if sub_split != last {
                    *builder += &urlencoding::encode(" or ");
                }
            }
        }

        query_index = Some(0);
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
    if value.chars().any(char::is_whitespace) {
        return format!("\"{value}\"");
    }

    value.to_string()
}
