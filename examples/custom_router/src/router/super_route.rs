use crate::router::url::Navigation;
use enum_paths::ParsePath;
use seed::Url;
use std::fmt::Debug;
use seed::prelude::IndexMap;

pub struct Route<Routes: Debug + PartialEq + ParsePath + Copy + Clone> {
    pub path: String,
    pub url: Url,
    pub guarded: bool,
    pub query_parameters: Option<IndexMap<String, String>>,
    pub id :String,
    pub default: bool,
    pub value: Routes,
}

impl<Routes: Debug + PartialEq + Navigation + ParsePath + Copy + Clone> Route<Routes> {
    pub fn new(guarded: bool, default: bool, value: Routes) -> Self {
        Route {
            path: value.as_path(),
            url: value.to_url(),
            guarded,
            query_parameters: Default::default(),
            id: "".to_string(),
            default,
            value,
        }
    }
}
