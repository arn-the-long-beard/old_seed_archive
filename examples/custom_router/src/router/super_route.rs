use crate::router::Urls;
use enum_paths::ParsePath;
use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;
use seed::Url;
use std::fmt::Debug;
use strum::IntoEnumIterator;

pub struct Route<Routes: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone> {
    pub path: String,
    pub url: Option<Url>,
    pub guarded: bool,
    pub query_parameters: HashMap<String, String>,
    pub default: bool,
    pub value: Routes,
}

impl<Routes: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone> Route<Routes> {
    pub fn new(guarded: bool, default: bool, value: Routes) -> Self {
        let full_path = value.as_path();
        let segments: Vec<&str> = full_path.split('/').collect();
        Route {
            path: full_path.as_str().to_string(),
            url: Option::from(Urls::new(Url::new()).build_url(segments)),
            guarded,
            query_parameters: Default::default(),
            default,
            value,
        }
    }
}
