#[cfg(test)]
mod test {
    extern crate custom_router;

    extern crate enum_paths;
    extern crate router_macro_derive;
    use self::custom_router::router::url::{convert_to_string, extract_url_payload};
    use super::*;
    use custom_router::router::url::Navigation;
    use enum_paths::{AsPath, ParseError, ParsePath};
    use router_macro_derive::Routing;
    use seed::prelude::IndexMap;
    use seed::prelude::*;
    use std::str::FromStr;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug)]
    struct UserTask {
        id: String,
        query: IndexMap<String, String>,
    }
    #[derive(Debug, PartialEq, Clone, Routing)]
    pub enum ExampleRoutes {
        Other,
        Admin {
            query: IndexMap<String, String>,
        },

        Dashboard(DashboardRoutes),
        Profile {
            id: String,
        },
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, Copy, Routing)]
    pub enum DashboardRoutes {
        Other,
        #[as_path = ""]
        Root,
    }

    #[wasm_bindgen_test]
    fn test_to_url() {
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url = ExampleRoutes::Admin {
            query: query_search,
        }
        .to_url();
        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        assert_eq!(url, url_to_compare);

        let url: Url = ExampleRoutes::Profile {
            id: "1".to_string(),
        }
        .to_url();

        let url_to_compare: Url = "/profile/1".parse().unwrap();
        assert_eq!(url, url_to_compare);
    }

    #[wasm_bindgen_test]
    fn test_from_path_to_enum() {
        let string = "/admin?user=arn&role=baby_programmer&location=norway";

        let route = ExampleRoutes::parse_path(string).unwrap();
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        assert_eq!(
            route,
            ExampleRoutes::Admin {
                query: query_search
            }
        );
        let string = "/profile/1";

        let route = ExampleRoutes::parse_path(string).unwrap();
        assert_eq!(
            route,
            ExampleRoutes::Profile {
                id: "1".to_string()
            }
        );
    }
    #[wasm_bindgen_test]
    fn test_url_navigation() {
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url = ExampleRoutes::Dashboard(DashboardRoutes::Root).to_url();
        let url_to_compare: Url = "/dashboard/".parse().unwrap();
        // let url = ExampleRoutes::Admin { query: query_search }.to_url();
        // let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
        //     .parse()
        //     .unwrap();
        assert_eq!(url, url_to_compare);
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

        let task: UserTask = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string()))
            .map(|(id, query)| (id.unwrap(), query.unwrap()))
            .map(|(id, query)| UserTask { id, query })
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
}
