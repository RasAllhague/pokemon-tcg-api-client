
#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
    Deserialize(serde_json::Error),
    Io(std::io::Error),
}