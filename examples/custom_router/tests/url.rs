mod init_state;
mod routing_module;
#[cfg(test)]
mod test {
    extern crate custom_router;
    extern crate router_macro_derive;
    use super::*;
    use custom_router::*;
    use router_macro_derive::*;
    use seed::prelude::{IndexMap, *};
    use seed::util::log;
    use seed::{*, *};
    use std::str::FromStr;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Clone, Url, Root)]
    pub enum ExampleRoutes {
        Other {
            id: String,
            children: Settings,
        },
        Admin {
            query: IndexMap<String, String>,
        },
        Dashboard(DashboardRoutes),
        Profile {
            id: String,
        },
        #[default_route]
        NotFound,
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, Url)]
    pub enum DashboardRoutes {
        #[as_path = "my_stuff"]
        Stuff { id: String },
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, Url)]
    pub enum Settings {
        Api(Apis),
        Projects {
            id: String,
            query: IndexMap<String, String>,
            children: Apis,
        },
    }

    #[derive(Debug, PartialEq, Clone, Url)]
    pub enum Apis {
        Facebook,
        Google,
        Microsoft,
    }

    #[wasm_bindgen_test]
    fn test_to_url() {
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url = ExampleRoutes::Admin {
            query: query_search.clone(),
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

        let url: Url = ExampleRoutes::Other {
            id: "2".to_string(),
            children: Settings::Projects {
                id: "14".to_string(),
                query: query_search.clone(),
                children: Apis::Facebook,
            },
        }
        .to_url();

        let url_to_compare: Url =
            "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway"
                .parse()
                .unwrap();
        assert_eq!(url, url_to_compare);

        let url: Url = ExampleRoutes::Other {
            id: "2".to_string(),
            children: Settings::Api(Apis::Facebook),
        }
        .to_url();

        let url_to_compare: Url = "/other/2/api/facebook".parse().unwrap();
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
        let string = "/profile/1/repos";

        let route = ExampleRoutes::parse_path(string).unwrap();
        assert_eq!(
            route,
            ExampleRoutes::Profile {
                id: "1".to_string(),
            }
        );

        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());

        let string_to_compare =
            "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway";
        assert_eq!(
            ExampleRoutes::parse_path(string_to_compare).unwrap(),
            ExampleRoutes::Other {
                id: "2".to_string(),
                children: Settings::Projects {
                    id: "14".to_string(),
                    query: query.clone(),
                    children: Apis::Facebook
                },
            }
        );
    }
    #[wasm_bindgen_test]
    fn test_convert_to_url() {
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url = ExampleRoutes::Dashboard(DashboardRoutes::Root).to_url();
        let url_to_compare: Url = "/dashboard/".parse().unwrap();

        assert_eq!(url, url_to_compare);
        let url = ExampleRoutes::Admin {
            query: query_search,
        }
        .to_url();
        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        assert_eq!(url, url_to_compare);
    }
    #[wasm_bindgen_test]
    fn test_convert_from_url() {
        let url_to_compare: Url = "/dashboard/".parse().unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Root);
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());
        let route = ExampleRoutes::Admin { query };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/profile/1".parse().unwrap();

        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Profile {
            id: "1".to_string(),
        };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/dashboard/my_stuff/123".parse().unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Stuff {
            id: "123".to_string(),
        });

        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());

        let url_to_compare: Url =
            "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway"
                .parse()
                .unwrap();
        assert_eq!(
            ExampleRoutes::from_url(url_to_compare).unwrap(),
            ExampleRoutes::Other {
                id: "2".to_string(),
                children: Settings::Projects {
                    id: "14".to_string(),
                    query: query.clone(),
                    children: Apis::Facebook
                },
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_convert_from_url_with_children() {
        let url_to_compare: Url = "/dashboard/".parse().unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Root);
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());
        let route = ExampleRoutes::Admin { query };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/profile/1".parse().unwrap();

        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Profile {
            id: "1".to_string(),
        };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/dashboard/my_stuff/123".parse().unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Stuff {
            id: "123".to_string(),
        });
    }

    #[wasm_bindgen_test]
    fn test_default_route() {
        assert_eq!(ExampleRoutes::default(), ExampleRoutes::NotFound);
    }
}
