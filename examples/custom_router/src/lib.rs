mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::models::user::LoggedUser;
use crate::{theme::Theme, top_bar::TopBar};
#[macro_use]
extern crate router_macro_derive;
use crate::pages::dashboard::task_list::TasksRoutes;
use crate::pages::dashboard::DashboardRoutes;
use enum_paths::{AsPath, ParseError, ParsePath};
use router_macro_derive::{InitState, Root, Routing};
pub mod models;
mod pages;
pub mod router;
mod theme;
mod top_bar;
use crate::router::state::StateInit;
use crate::router::super_router::SuperRouter;
use crate::router::url::Navigation;
use crate::router::view::{Guarded, OnView};

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UserLogged);

    let mut router: SuperRouter<Routes> = SuperRouter::new();
    router.init_url_and_navigation(url);

    Model {
        theme: Theme::default(),
        state: Default::default(),
        router,
        logged_user: None,
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Routing, Root, InitState)]
// need to make a derive (Routing) or something maybe
pub enum Routes {
    #[state_scope = "state.login => pages::login::init"]
    Login,
    Register,
    Dashboard(DashboardRoutes),
    // #[default_route]
    // Admin {
    //     query: IndexMap<String, String>,
    // },
    #[default_route]
    NotFound,
    #[as_path = ""]
    Home,
}
// impl StateInit<Routes, State, Msg> for Routes {
//     fn init<'b, 'c>(
//         self,
//         previous_state: &'b mut State,
//         orders: &'c mut impl Orders<Msg>,
//     ) -> &'b mut State {
//         match self {
//             Routes::Login => {
//                 previous_state.login = pages::login::init(
//                     self.to_url(),
//                     &mut previous_state.login,
//                     &mut orders.proxy(Msg::Login),
//                 )
//             }
//             Routes::Register => {}
//             Routes::Dashboard(routes) => {}
//             Routes::NotFound => {}
//             Routes::Home => {}
//         }
//         previous_state
//     }
// }

// impl Routes {
//     fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
//         match url.remaining_path_parts().as_slice() {
//             [] => Self::Home,
//             [CLIENTS_AND_PROJECTS] => Self::ClientsAndProjects(page::clients_and_projects::init(
//                 url,
//                 &mut orders.proxy(Msg::ClientsAndProjectsMsg),
//             )),
//             [TIME_TRACKER] => Self::TimeTracker(page::time_tracker::init(
//                 url,
//                 &mut orders.proxy(Msg::TimeTrackerMsg),
//             )),
//             [TIME_BLOCKS] => Self::TimeBlocks(page::time_blocks::init(
//                 url,
//                 &mut orders.proxy(Msg::TimeBlocksMsg),
//             )),
//             [SETTINGS] => Self::Settings(page::settings::init(
//                 url,
//                 &mut orders.proxy(Msg::SettingsMsg),
//             )),
//             _ => Self::NotFound,
//         }
//     }
// }
// ------ ------
//     Model
// ------ ------

struct Model {
    state: State,
    router: SuperRouter<Routes>,
    logged_user: Option<LoggedUser>,
    theme: Theme,
}

// ------ State for component ------
#[derive(Default)]
pub struct State {
    pub register: pages::register::Model,
    pub login: pages::login::Model,
    pub dashboard: pages::dashboard::Model,
}

// ------ ------
//    Update
// ------ ------
/// Root actions for your app.
/// Each component will have single action/message mapped to its message later
/// in update

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    UrlRequested(subs::UrlRequested),
    Register(pages::register::Msg),
    Login(pages::login::Msg),
    UserLogged(LoggedUser),
    Dashboard(pages::dashboard::Msg),
    GoBack,
    GoForward,
    SwitchToTheme(Theme),
}

/// Main update for the entire APP, every component action/message should me
/// mapped there because of single truth of path
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("URL has changed");
            model.router.confirm_navigation(url);

            if let Some(current_route) = model.router.current_route {
                current_route.init(model, orders);
            }
            // model.router.current_route.unwrap().init(model, orders);

            // model.state =  Routes::init_state::<State>( previous_state, url , orders: &mut impl Orders<Msg>);
        }
        Msg::UrlRequested(request) => {
            log!("URL requested");

            // let url = request.0;

            //side effect is bad i think
        }
        Msg::Register(register_message) => pages::register::update(
            register_message,
            &mut model.state.register,
            &mut orders.proxy(Msg::Register),
        ),
        Msg::Login(login_message) => pages::login::update(
            login_message,
            &mut model.state.login,
            &mut orders.proxy(Msg::Login),
        ),
        Msg::Dashboard(dashboard_message) => pages::dashboard::update(
            dashboard_message,
            &mut model.state.dashboard,
            &mut orders.proxy(Msg::Dashboard),
        ),
        Msg::UserLogged(user) => {
            log!("got user logged");
            model.logged_user = Some(user);
            // orders.notify(subs::UrlRequested::new(
            //     Urls::new(&model.base_url).build_url(DASHBOARD),
            // ));
        }

        Msg::SwitchToTheme(theme) => model.theme = theme,

        Msg::GoBack => {
            model
                .router
                .request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::GoForward => {
            model
                .router
                .request_moving_forward(|r| orders.notify(subs::UrlRequested::new(r)));
        }
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model),
        if let Some(route) = &model.router.current_route {
            route.view(model)
        } else {
            home(&model.theme)
        },
    ]
}

/// Auto generated by proc macro attribute
/// Can be called manually or automatically if sub routes/children use also OnView
impl OnView<Routes, Model, Msg> for Routes {
    fn view(&self, scoped_state: &Model) -> Node<Msg> {
        match self {
            Routes::Home => home(&scoped_state.theme),
            // Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
            Routes::NotFound => div!["404"],
            Routes::Login => pages::login::view(&scoped_state.state.login).map_msg(Msg::Login),
            Routes::Register => {
                pages::register::view(&scoped_state.state.register).map_msg(Msg::Register)
            }
            Routes::Dashboard(routes) => {
                if let Some(authenticated) = self.check_before_load(&scoped_state) {
                    pages::dashboard::cross(
                        *routes,
                        &scoped_state.state.dashboard,
                        &scoped_state.router,
                    )
                    .map_msg(Msg::Dashboard)
                } else {
                    div!["401"]
                }
            }
            _ => div!["404"],
        }
    }
}

/// Auto generated by proc macro attribute and called inside view
impl Guarded<Routes, Model, Msg> for Routes {
    fn check_before_load(&self, scoped_state: &Model) -> Option<bool> {
        if scoped_state.logged_user.is_some() {
            // this party will be a function which the user has full control on, could be use for user permission as well
            Some(true)
        } else {
            None
        }
    }
}

fn header(model: &Model) -> Node<Msg> {
    // let page = &model.page;
    // let base_url = &model.base_url;
    // let mut list: Vec<Node<Msg>> = Vec::new();

    // for route in &model.router.mapped_routes() {
    //     list.push(render_route(route));
    // }
    // list =
    div![
        TopBar::new("Welcome Guest")
            .style(model.theme.clone())
            .content(div![
                style! {St::Display => "block" },
                button![
                    "back",
                    attrs! {
                        At::Disabled  =>   (!model.router.can_back()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoBack)
                ],
                button![
                    "forward",
                    attrs! {
                        At::Disabled =>  (!model.router.can_forward()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoForward)
                ]
            ]),
        render_route(model)
    ]
}
//

fn render_route(model: &Model) -> Node<Msg> {
    ul![
        li![a![
            C![
                "route",
                IF!( model.router.is_current_route(&Routes::Login) => "active-route" )
            ],
            attrs! { At::Href => model.router.url(&Routes::Login) },
            "Login",
        ]],
        li![a![
            C![
                "route",
                IF!(model.router.is_current_route(&Routes::Register) => "active-route" )
            ],
            attrs! { At::Href => model.router.url(&Routes::Register) },
            "Register",
        ]],
        li![a![
            C![
                "route",
                IF!(model.router.is_current_route(&Routes::NotFound) => "active-route" )
            ],
            attrs! { At::Href => model.router.url(&Routes::NotFound) },
            "NotFound",
        ]],
        li![a![
            C![
                "route",
                IF!(model.router.is_current_route(&Routes::Home) => "active-route" )
            ],
            attrs! { At::Href => model.router.url(&Routes::Home) },
            "Home",
        ]],
        li![a![C!["route"], "Dashboard",]],
        ul![
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Root)) => "active-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Root)) },
                "Profile",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Message)) => "active-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Message)) },
                "Messages",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Statistics)) => "active-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Statistics)) },
                "Statistics",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Root))) => "active-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Root))) },
                "Tasks",
            ]],
        ],
    ]
}
// fn render_route(route: &AvailableRoute) -> Node<Msg> {
//     li![a![
//         C!["route", IF!( route.is_active => "active-route" )],
//         attrs! { At::Href => route.url },
//         &route.name,
//     ]]
// }
// // /// Render a route
// fn render_route(router : &Router<Routes>, route : Routes) -> Node<Msg> {
//     li![a![
//         C![
//             "route",
//             IF!(router. ) => "active-route" )
//         ],
//         attrs! { At::Href => Urls::new(base_url).build_url(path) },
//         path,
//     ]]
// }

// fn authenticated_header(base_url: &Url, page: &Route) -> Node<Msg> {
//     ul![route(base_url, page, "Dashboard"),]
// }

fn home(theme: &Theme) -> Node<Msg> {
    div![
        div!["Welcome home!"],
        match theme {
            Theme::Dark => {
                button![
                    "Switch to Light",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Light))
                ]
            }
            Theme::Light => {
                button![
                    "Switch to Dark",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Dark))
                ]
            }
        }
    ]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
