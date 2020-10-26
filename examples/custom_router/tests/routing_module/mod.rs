use crate::routing_module::admin::Msg;

use seed::{prelude::*, *};

pub mod admin;
pub mod dashboard;
pub mod other;
pub mod profile;

#[cfg(test)]
pub mod test {
    extern crate custom_router;
    extern crate router_macro_derive;

    use super::*;
    use custom_router::*;
    use router_macro_derive::*;

    use seed::browser::service::fetch::FailReason;
    use seed::{prelude::*, *};
    use wasm_bindgen_test::*;

    pub struct UserLogged {
        name: String,
    }

    pub struct Model {
        user: Option<UserLogged>,
        other: other::Model,
        dashboard: dashboard::Model,
        admin: admin::Model,
        profile: profile::Model,
    }

    pub enum Msg {
        other(other::Msg),
        dashboard(dashboard::Msg),
        admin(admin::Msg),
        profile(profile::Msg),
    }

    #[derive(Debug, PartialEq, Clone, RoutingModules)]
    pub enum SuperExampleRoutes {
        Other {
            id: String,
            children: other::Routes,
        },
        Admin {
            query: IndexMap<String, String>,
        },
        #[guard = " user => guard => forbidden"]
        Dashboard(dashboard::Routes),
        Profile {
            id: String,
        },
        #[default_route]
        #[view = " => not_found"]
        NotFound,
        #[as_path = ""]
        #[view = " => home"]
        Root,
    }

    pub fn init() {}

    pub fn view(model: &Model) -> Node<Msg> {
        div![]
    }

    pub fn home(model: &Model) -> Node<Msg> {
        div![]
    }

    pub fn update() {}

    pub fn not_found(model: &Model) -> Node<Msg> {
        div![]
    }
    pub fn forbidden(model: &Model) -> Node<Msg> {
        div![]
    }
    pub fn guard(user: Option<&UserLogged>) -> Option<bool> {
        if user.is_some() {
            Some(true)
        } else {
            None
        }
    }
}
