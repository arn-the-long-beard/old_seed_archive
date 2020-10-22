use crate::router::state::StateInit;
use crate::router::url::Navigation;
use crate::router::view::ToView;
use enum_paths::{AsPath, ParseError, ParsePath};
use router_macro_derive::{InitState, OnView, Root, Routing};
use seed::{prelude::*, *};

pub fn init(url: Url, previous_state: &Model, order: &mut impl Orders<Msg>) -> Model {
    Model {
        id: "".to_string(),
        name: "".to_string(),
        description: "".to_string(),
    }
}
#[derive(Default)]
pub struct Model {
    id: String,
    name: String,
    description: String,
}

pub enum Msg {}

#[derive(Debug, PartialEq, Clone, Copy, Routing, Root, InitState, OnView)]
pub enum AdminRoutes {
    #[local_view = " => root"]
    Root,
    #[local_view = " => manager"]
    Manager,
    #[default_route]
    #[local_view = " => not_found"]
    NotFound,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}

pub fn view(routes: &AdminRoutes, model: &Model) -> Node<Msg> {
    routes.view(model)
}
fn manager(model: &Model) -> Node<Msg> {
    div!["manage stuff"]
}
fn root(model: &Model) -> Node<Msg> {
    div!["root of admin panel"]
}
fn not_found(model: &Model) -> Node<Msg> {
    div!["not found in admin"]
}
