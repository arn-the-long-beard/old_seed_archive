use enum_paths::ParseError;
use seed::{prelude::IndexMap, Url};

pub trait Navigation {
    fn from_url(url: Url) -> std::result::Result<Self, ParseError>
    where
        Self: Sized;
    fn to_url(&self) -> Url;
}

#[cfg(test)]
mod test {
    use crate::router::{Router, Urls};

    extern crate enum_paths;
    extern crate router_macro_derive;

    use super::*;
    use enum_paths::{AsPath, ParseError, ParsePath};

    #[derive(Debug)]
    struct UserTask {
        id: String,
        query: IndexMap<String, String>,
        children: String,
    }
    #[derive(Debug)]
    struct UserTask2 {
        id: String,
        query: IndexMap<String, String>,
    }
    #[test]
    fn test_string_to_index_map() {
        let string = "/task?user=arn&role=programmer";

        let query = extract_url_payload(string.to_string());

        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();

        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(query.1.unwrap(), query_to_compare);
    }

    #[test]
    fn test_strings() {
        let string = "/task/12?user=arn&role=programmer";

        let task: UserTask2 = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string()))
            .map(|(id, query, _)| (id.unwrap(), query.unwrap()))
            .map(|(id, query)| UserTask2 { id, query })
            .unwrap();

        eprintln!("{:?}", task);
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();
        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(task.id, "12");
        assert_eq!(task.query, query_to_compare);

        let string = "?user=arn&role=programmer";

        let query = extract_url_payload(string.to_string());

        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();

        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(query.1.unwrap(), query_to_compare);
    }
    #[test]
    fn test_strings_with_children() {
        let string = "/task/12/stuff?user=arn&role=programmer";

        let task: UserTask = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string()))
            .map(|(id, query, children)| (id.unwrap(), query.unwrap(), children.unwrap()))
            .map(|(id, query, children)| UserTask {
                id,
                query,
                children,
            })
            .unwrap();

        eprintln!("{:?}", task);
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();
        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(task.id, "12");
        assert_eq!(task.query, query_to_compare);
    }
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
) -> (
    Option<String>,
    Option<IndexMap<String, String>>,
    Option<String>,
) {
    let mut query: IndexMap<String, String> = IndexMap::new();

    let params: Vec<&str> = query_string.split('?').collect();
    let mut params_iter = params.iter();

    let mut root_paths = params_iter.next().unwrap().split('/');

    let root = root_paths.next();

    if root.is_some() && !root.unwrap().is_empty() {
        eprintln!("root path should be like ''");
    }
    // make error if root is not empty

    let path = root_paths.next().map(|r| r.to_string());
    let children_path = root_paths.next().map(|r| r.to_string());

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

    (path, result, children_path)
}
