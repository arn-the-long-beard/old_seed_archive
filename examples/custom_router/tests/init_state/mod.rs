pub mod other;
pub mod profile;

pub use {other::*, profile::*};

#[cfg(test)]
pub mod test {
    extern crate custom_router;
    extern crate router_macro_derive;

    use super::*;
    use custom_router::*;
    use router_macro_derive::*;

    use seed::{prelude::*, *};
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    pub enum Msg {
        Other(other::Msg),
        Profile(profile::Msg),
        GoBack,
        GoForward,
    }

    #[derive(Default)]
    pub struct Model {
        state: State,
    }

    #[derive(Default)]
    pub struct State {
        profile: profile::Model,
        stuff: profile::Model,
    }

    #[derive(Debug, PartialEq, Clone, Url, Root, OnInit)]
    #[model_scope = "state.stuff => profile::init"]
    pub enum ExampleRoutes {
        Other {
            id: String,
            children: Settings,
        },

        Admin {
            query: IndexMap<String, String>,
        },
        Dashboard(DashboardRoutes),
        #[model_scope = "state.profile => profile::init"]
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
        let route = ExampleRoutes::Other {
            id: "2".to_string(),
            children: Settings::Api(Apis::Google),
        };
    }
}
