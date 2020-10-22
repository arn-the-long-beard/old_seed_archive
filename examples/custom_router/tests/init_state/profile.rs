use seed::Url;
use seed::{prelude::*, *};
#[derive(Default)]
pub struct Model {
    pub id: String,
    pub user_name: String,
}

pub enum Msg {
    Update,
    AddApi,
}

pub fn init(url: Url, previous_state: &Model, id: &String, orders: &mut impl Orders<Msg>) -> Model {
    log!("login init with previous state");
    Model {
        id: "".to_string(),
        user_name: "".to_string(),
    }
}
