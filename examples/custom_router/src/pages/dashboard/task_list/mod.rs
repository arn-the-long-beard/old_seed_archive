use crate::pages::dashboard::DashboardRoutes;
use crate::router::super_router::{AvailableRoute, SuperRouter};
use crate::Routes;
use enum_paths::{AsPath, Named, ParseError, ParsePath};
use seed::{prelude::*, *};

pub mod task;

pub struct Model {
    pub tasks: Vec<task::Model>,
    pub selected_task_no: Option<u32>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            selected_task_no: None,
            tasks: get_dummy_data(),
        }
    }
}
#[derive(Debug, EnumIter, PartialEq, Copy, Clone, AsPath, Named)]
pub enum TasksRoutes {
    Task(u32),
    #[as_path = ""]
    Root,
}
#[derive(Debug, Copy, Clone)]
pub enum Msg {
    ClickTask(u32),
    Task(task::Msg),
    LoadTasks,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClickTask(no) => {}
        Msg::LoadTasks => model.tasks = get_dummy_data(),
        Msg::Task(task) => {
            let index: usize = model.selected_task_no.unwrap() as usize;
            task::update(task, model.tasks.get_mut(index).unwrap())
        }
    }
}
// pub fn view(model: &Model, router: &SuperRouter<Routes>) -> Node<Msg> {
//     div!["my tasks", render_tasks(model, router),]
// }

fn render_tasks(model: &Model, router: &SuperRouter<Routes>) -> Node<Msg> {
    ul![list(&model.tasks, router)]
}

pub fn list(tasks: &[task::Model], router: &SuperRouter<Routes>) -> Vec<Node<Msg>> {
    let mut tasks_list = Vec::new();

    log!(tasks_list);

    for t in tasks {
        tasks_list.push(render_task(t, router));
    }
    tasks_list
}

pub fn render_task(task: &task::Model, router: &SuperRouter<Routes>) -> Node<Msg> {
    let task_url = SuperRouter::<Routes>::url_static(&Routes::Dashboard(DashboardRoutes::Tasks(
        TasksRoutes::Task(task.task_no),
    )));
    let route = Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Task(task.task_no)));
    li![a![
        C![
            "route",
            IF!(router.is_current_route(&route) => "active-route")
        ],
        attrs! { At::Href => task_url  },
        task.task_title.to_string(),
    ]]
}

pub fn get_dummy_data() -> Vec<task::Model> {
    vec![
        task::Model {
            task_no: 0,
            task_title: "Nested routing".to_string(),
            task_description: "Try to find an easy way to manipulate nested route".to_string(),
        },
        task::Model {
            task_no: 1,
            task_title: "Guard & permission".to_string(),
            task_description: "FInd a way to set Guard for protected routes".to_string(),
        },
        task::Model {
            task_no: 2,
            task_title: "Stuff".to_string(),
            task_description: "Additional stuff to do".to_string(),
        },
    ]
}
pub fn view(task_routes: TasksRoutes, model: &Model, router: &SuperRouter<Routes>) -> Node<Msg> {
    div![vec![
        render_tasks(model, router),
        match task_routes {
            TasksRoutes::Task(task_no) => {
                let task = model.tasks.iter().find(|t| t.task_no == task_no);
                task::view(task.unwrap()).map_msg(Msg::Task)
            }
            TasksRoutes::Root => div!["no task selected"],
        },
    ]]
}
