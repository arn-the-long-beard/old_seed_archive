use crate::router::super_router::Urls;
use enum_paths::{ParseError, ParsePath};
use seed::{prelude::IndexMap, Url};
use std::{fmt::Debug, slice::Iter};
use strum::IntoEnumIterator;

pub trait Navigation {
    fn from_url(url: Url,) -> std::result::Result<Self, ParseError,>
    where
        Self: Sized;
    fn to_url(&self,) -> Url;
}

// impl<T: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone>
// Routing for T {     fn from_url(url: Url) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         let mut path = url.to_string();
//         path.remove(0);
//         T::parse_path(path.as_str()).ok()
//     }
//
//     fn to_url(&self) -> Url {
//         let full_path = &self.as_path();
//         let segments: Vec<&str> = full_path.as_str().split('/').collect();
//         Urls::new(Url::new()).build_url(segments)
//     }
// }
#[cfg(test)]
mod test {
    use crate::router::{Router, Urls};

    extern crate enum_paths;
    extern crate router_macro_derive;

    use super::*;
    use enum_paths::{AsPath, ParseError, ParsePath};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Copy, Clone, AsPath)]
    pub enum DashboardAdminRoutes {
        Other,
        #[as_path = ""]
        Root,
    }
    impl Default for DashboardAdminRoutes {
        fn default() -> DashboardAdminRoutes {
            DashboardAdminRoutes::Root
        }
    }

    #[derive(Debug, PartialEq, Copy, Clone, AsPath)]
    pub enum DashboardRoutes {
        Admin(DashboardAdminRoutes,),
        Profile(u32,),
        #[as_path = ""]
        Root,
    }
    impl Default for DashboardRoutes {
        fn default() -> DashboardRoutes {
            DashboardRoutes::Root
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, EnumIter, AsPath)]
    enum ExampleRoutes {
        Login,
        Register,
        Stuff,
        Dashboard(DashboardRoutes),
        NotFound,
        #[as_path = ""]
        Home,
    }

    // #[wasm_bindgen_test]
    // fn test_to_urls() {
    //     let mut router = SuperRouter::<ExampleRoutes>::new();
    //     //todo we should have attribute in the enum for default route
    //     router.default_route = Some(ExampleRoutes::NotFound);
    //
    //     assert_eq!(
    //         ExampleRoutes::Login.to_url(),
    //         router.url(&ExampleRoutes::Login)
    //     );
    //     assert_eq!(ExampleRoutes::Login.to_url().to_string(), "/login");
    //
    //     assert_eq!(
    //         ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
    //             .to_url()
    //             .to_string(),
    //         "/dashboard/profile/1"
    //     )
    // }

    // #[wasm_bindgen_test]
    // fn test_from_urls() {
    //     let mut router = SuperRouter::<ExampleRoutes>::new();
    //     //todo we should have attribute in the enum for default route
    //     router.default_route = Some(ExampleRoutes::NotFound);
    //
    //     assert_eq!(
    //         ExampleRoutes::Login.to_url(),
    //         router.url(&ExampleRoutes::Login)
    //     );
    //
    //     let url: Url = "/login".parse().unwrap();
    //     assert_eq!(ExampleRoutes::from_url(url).unwrap(), ExampleRoutes::Login);
    //
    //     let url: Url = "/dashboard/profile/1".parse().unwrap();
    //     assert_eq!(
    //         ExampleRoutes::from_url(url).unwrap(),
    //         ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
    //     );
    // }
    //
    // #[test]
    // fn test_convert_to_string() {
    //     let mut hash: IndexMap<String, String> = IndexMap::new();
    //
    //     hash.insert("user".to_string(), "arn".to_string());
    //     hash.insert("role".to_string(), "baby_programmer".to_string());
    //     hash.insert("location".to_string(), "norway".to_string());
    //
    //     assert_eq!(
    //         "user=arn&role=baby_programmer&location=norway",
    //         convert_to_string(hash)
    //     );
    // }
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
        Some(query,)
    } else {
        None
    };

    (path, result)
}
