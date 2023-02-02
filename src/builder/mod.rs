pub mod pokemon;

use std::fmt::Display;


pub enum Ordering {
    Ascending(String),
    Descending(String),
}

pub trait QueryBuilder {
    fn new() -> Self;
    fn build(self, url: &str) -> String;
}

fn build_query_param<T: Display>(builder: &mut String, query_index: &mut Option<usize>, key: &str, value: T) {
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