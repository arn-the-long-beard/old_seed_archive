use enum_paths::ParseError;
use seed::{prelude::IndexMap, Url};

pub trait Navigation {
    fn from_url(url: Url) -> std::result::Result<Self, ParseError>
    where
        Self: Sized;
    fn to_url(&self) -> Url;
}

pub fn convert_to_string(query: IndexMap<String, String>) -> String {
    let mut query_string = "".to_string();
    for (i, q) in query.iter().enumerate() {
        query_string += format!("{}={}", q.0, q.1).as_str();

        if i != query.len() - 1 {
            query_string += format!("&").as_str();
        }
    }
    query_string
}

pub fn extract_url_payload(
    query_string: String,
) -> (Option<String>, Option<IndexMap<String, String>>) {
    let mut query: IndexMap<String, String> = IndexMap::new();

    let params: Vec<&str> = query_string.split('?').collect();
    let mut params_iter = params.iter();

    let mut root_paths = params_iter.next().unwrap().split('/');

    let root = root_paths.next();
    // make error if root is not empty

    let path = root_paths.next().map(|r| r.to_string());

    if let Some(sub_string) = params_iter.next() {
        let key_value: Vec<&str> = sub_string.split('&').collect();

        for pair in key_value {
            let mut sub = pair.split('=');
            let key = sub.next().expect("we should have a key for the parameter");
            let value = sub.next().expect("we should have a value for this key");
            query.insert(key.to_string(), value.to_string());
        }
    }
    let result = if query.iter().len() > 0 {
        Some(query)
    } else {
        None
    };

    (path, result)
}
