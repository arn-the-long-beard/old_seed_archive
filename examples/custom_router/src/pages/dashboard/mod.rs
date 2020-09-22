use seed::{prelude::*, *};
pub mod message;
use crate::router::children::ExtractRoutes;
use crate::router::route::Route;
use crate::router::Router;
use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;
use std::str::FromStr;
pub mod statistics;
use router_macro_derive::Routes;
use strum::{EnumProperty, IntoEnumIterator};

#[derive(EnumIter, EnumString, EnumProperty, Display, Debug, Copy, Clone, PartialEq, Routes)]
#[strum(serialize_all = "snake_case")]
pub enum DashboardRoutes {
    #[strum(props(Default = "true"))]
    Root,
    Message,
    Statistics,
}

impl Default for DashboardRoutes {
    fn default() -> DashboardRoutes {
        DashboardRoutes::Root
    }
}
#[derive(Default)]
pub struct Model {
    pub name: String,
    pub state: State,
}
#[derive(Default)]
pub struct State {
    message: message::Model,
    statistics: statistics::Model,
}

pub enum Msg {
    ChangeName,
    Message(message::Msg),
    Statistic(statistics::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeName => {}
        Msg::Message(message) => message::update(
            message,
            &mut model.state.message,
            &mut orders.proxy(Msg::Message),
        ),
        Msg::Statistic(statistics) => statistics::update(
            statistics,
            &mut model.state.statistics,
            &mut orders.proxy(Msg::Statistic),
        ),
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    div![&model.name]
}

pub fn cross(dashboard_routes: DashboardRoutes, model: &Model) -> Node<Msg> {
    match dashboard_routes {
        DashboardRoutes::Root => view(model),
        DashboardRoutes::Message => message::view(&model.state.message).map_msg(Msg::Message),
        DashboardRoutes::Statistics => {
            statistics::view(&model.state.statistics).map_msg(Msg::Statistic)
        }
    }
}
