use seed::{prelude::*, *};
#[derive(Default)]
pub struct Model {
    pub routes_history_count: u32,
}

pub enum Msg {
    AddMessage(String),
}
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}
pub fn view(model: &Model) -> Node<Msg> {
    div!["route visited => {}", &model.routes_history_count]
}
