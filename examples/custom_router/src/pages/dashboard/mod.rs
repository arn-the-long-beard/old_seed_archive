use seed::{prelude::*, *};
pub mod message;
pub mod statistics;
pub mod task_list;
use crate::pages::dashboard::task_list::TasksRoutes;
use crate::router::super_router::SuperRouter;
use crate::Routes;
use enum_paths::{AsPath, ParseError, ParsePath};

#[derive(Debug, PartialEq, Copy, Clone, AsPath)]
pub enum DashboardRoutes {
    Message,
    Tasks(TasksRoutes),
    Statistics,
    #[as_path = ""]
    Root,
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
pub fn view(model: &Model) -> Node<Msg> {
    div![&model.name]
}

pub fn cross(dashboard_routes: &DashboardRoutes, model: &Model) -> Node<Msg> {
    match dashboard_routes {
        DashboardRoutes::Root => view(model),
        DashboardRoutes::Message => message::view(&model.state.message).map_msg(Msg::Message),
        DashboardRoutes::Statistics => {
            statistics::view(&model.state.statistics).map_msg(Msg::Statistic)
        }
        DashboardRoutes::Tasks(task_routes) => {
            task_list::view(task_routes, &model.state.tasks).map_msg(Msg::Tasks)
        }
    }
}
