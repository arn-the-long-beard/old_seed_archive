use seed::{prelude::*, *};
#[derive(Default)]
pub struct Model {
    pub messages: Vec<String>,
}

pub enum Msg {
    AddMessage(String),
}
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddMessage(name) => {}
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    div!["messages list"]
}
