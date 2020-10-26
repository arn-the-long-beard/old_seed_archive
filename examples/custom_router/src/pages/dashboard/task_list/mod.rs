use crate::pages::dashboard::DashboardRoutes;
use crate::router;
use crate::Routes;
use router::*;
use seed::{prelude::*, *};

pub mod task;

pub fn init(url: Url, model: &Model, orders: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

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
#[derive(Debug, PartialEq, Clone, AsUrl)]
pub enum TasksRoutes {
    Task {
        id: String,
    },
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

fn render_tasks(model: &Model) -> Node<Msg> {
    ul![list(&model.tasks)]
}

pub fn list(tasks: &[task::Model]) -> Vec<Node<Msg>> {
    let mut tasks_list = Vec::new();
    for t in tasks {
        tasks_list.push(render_task(t));
    }
    tasks_list
}

pub fn render_task(task: &task::Model) -> Node<Msg> {
    let task_url = Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Task {
        id: task.task_no.to_string(),
    }))
    .to_url();
    // let route = Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Task(task.task_no)));
    li![a![
        C![
            "route",
            IF!(is_current_url(task_url.clone()) => "active-route")
        ],
        attrs! { At::Href => task_url},
        task.task_title.to_string(),
    ]]
}
fn is_current_url(url: Url) -> bool {
    Url::current() == url
}
pub fn get_dummy_data() -> Vec<task::Model> {
    vec![
        task::Model {
            task_no: 0,
            task_title: "Nested Url".to_string(),
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
pub fn view(task_routes: &TasksRoutes, model: &Model) -> Node<Msg> {
    div![vec![
        render_tasks(model),
        match task_routes {
            TasksRoutes::Task { id } => {
                let task = model.tasks.iter().find(|t| t.task_no.to_string() == *id);
                task::view(task.unwrap()).map_msg(Msg::Task)
            }
            TasksRoutes::Root => div!["no task selected"],
        },
    ]]
}
