pub mod other;
pub mod profile;

#[cfg(test)]
pub mod test {
    extern crate custom_router;

    extern crate enum_paths;
    extern crate router_macro_derive;

    use self::custom_router::router::url::{convert_to_string, extract_url_payload};
    use super::*;
    use custom_router::router::state::StateInit;
    use custom_router::router::url::Navigation;
    use enum_paths::{AsPath, ParseError, ParsePath};
    use router_macro_derive::{InitState, Root, Routing};
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

    #[derive(Debug, PartialEq, Clone, Routing, Root, InitState)]
    #[state_scope = "state.stuff => profile::init"]
    pub enum ExampleRoutes {
        Other {
            id: String,
            children: Settings,
        },

        Admin {
            query: IndexMap<String, String>,
        },
        Dashboard(DashboardRoutes),
        #[state_scope = "state.profile => profile::init"]
        Profile {
            id: String,
        },
        #[default_route]
        NotFound,
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, Routing)]
    pub enum DashboardRoutes {
        #[as_path = "my_stuff"]
        Stuff { id: String },
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, Routing)]
    pub enum Settings {
        Api(Apis),
        Projects {
            id: String,
            query: IndexMap<String, String>,
            children: Apis,
        },
    }

    #[derive(Debug, PartialEq, Clone, Routing)]
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
