use seed::{prelude::*, *};
pub mod message;
pub mod statistics;
pub mod task_list;
use crate::pages::dashboard::task_list::TasksRoutes;
use crate::router;
pub use router::View;
use router::*;
use router_macro_derive::AsUrl;
#[derive(Debug, PartialEq, Clone, AsUrl)]
pub enum DashboardRoutes {
    Message,
    Tasks(TasksRoutes),
    Statistics,
    #[as_path = ""]
    Root,
}
pub fn init(url: Url, model: &mut Model, orders: &mut impl Orders<Msg>) -> Model {
    Model::default()
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
    tasks: task_list::Model,
}

pub enum Msg {
    ChangeName,
    Message(message::Msg),
    Statistic(statistics::Msg),
    Tasks(task_list::Msg),
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
        Msg::Tasks(task) => {
            task_list::update(task, &mut model.state.tasks, &mut orders.proxy(Msg::Tasks))
        }
    }
}
pub fn view(dashboard_routes: &DashboardRoutes, model: &Model) -> Node<Msg> {
    match dashboard_routes {
        DashboardRoutes::Root => root(dashboard_routes, model),
        DashboardRoutes::Message => message::view(&model.state.message).map_msg(Msg::Message),
        DashboardRoutes::Statistics => {
            statistics::view(&model.state.statistics).map_msg(Msg::Statistic)
        }
        DashboardRoutes::Tasks(task_routes) => {
            task_list::view(task_routes, &model.state.tasks).map_msg(Msg::Tasks)
        }
    }
}

pub fn root(dashboard_routes: &DashboardRoutes, model: &Model) -> Node<Msg> {
    div!["root for dashboard"]
}
