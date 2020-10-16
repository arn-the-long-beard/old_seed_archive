use seed::Url;
use seed::{prelude::*, *};

pub enum Msg {
    DoingStuff,
    EventHappened,
}
pub struct Model {
    stuff: String,
}
pub fn init(url: Url, previous_state: &Model, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        stuff: "doing stuff".to_string(),
    }
}
