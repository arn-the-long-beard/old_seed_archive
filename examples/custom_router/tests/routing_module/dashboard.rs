extern crate custom_router;
extern crate router_macro_derive;

use super::*;
use custom_router::*;
use router_macro_derive::*;

use seed::{prelude::*, *};
use wasm_bindgen_test::*;

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    #[default_route]
    #[view = " => not_found"]
    NotFound,
    #[view = " => settings"]
    Settings,
}
pub enum Msg {}
pub fn init() {}

pub struct Model {
    stuff: String,
}

pub fn view(nested: &Routes, model: &Model) -> Node<Msg> {
    div![]
}

fn not_found(model: &Model) -> Node<Msg> {
    div![]
}

fn settings(model: &Model) -> Node<Msg> {
    div![]
}
