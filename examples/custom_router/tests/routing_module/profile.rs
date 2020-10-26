extern crate custom_router;
extern crate router_macro_derive;

use super::*;
use custom_router::*;
use router_macro_derive::*;

use seed::{prelude::*, *};
use wasm_bindgen_test::*;

pub fn init() {}

pub struct Model {}
pub enum Msg {}
pub fn view(model: &Model) -> Node<Msg> {
    div![]
}

fn not_found(model: &Model) -> Node<Msg> {
    div![]
}

fn settings(model: &Model) -> Node<Msg> {
    div![]
}
