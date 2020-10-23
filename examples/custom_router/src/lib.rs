mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::models::user::{LoggedUser, Role};
use crate::{theme::Theme, top_bar::TopBar};
#[macro_use]
extern crate router_macro_derive;
use crate::pages::dashboard::task_list::TasksRoutes;
use crate::pages::dashboard::DashboardRoutes;
use crate::router::model::Init;
use crate::router::url::Navigation;
use crate::router::view::View;
use enum_paths::{AsPath, ParseError, ParsePath};
use router_macro_derive::{Init, Root, Routing, View};
pub mod models;
mod pages;
pub mod router;
mod theme;
mod top_bar;
use crate::pages::admin::AdminRoutes;

use crate::router::url::{convert_to_string, extract_url_payload};

use crate::router::Router;
use std::fmt::Debug;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UserLogged);

    let mut router: Router<Routes> = Router::new();
    router.init_url_and_navigation(url);

    Model {
        theme: Theme::default(),
        state: Default::default(),
        router,
        logged_user: None,
    }
}

#[derive(Debug, PartialEq, Clone, Routing, Root, Init, View)]
// need to make a derive (Routing) or something maybe
pub enum Routes {
    #[model_scope = "state.login => pages::login::init"]
    #[view_scope = "state.login => pages::login::view"]
    Login { query: IndexMap<String, String> },
    #[view_scope = "state.register => pages::register::view"]
    Register,
    #[view_scope = "state.dashboard => pages::dashboard::cross"]
    #[guard_scope = " => guard => forbidden"]
    Dashboard(DashboardRoutes),
    // #[default_route]
    #[model_scope = "state.admin => pages::admin::init"]
    #[view_scope = "state.admin => pages::admin::view"]
    #[guard_scope = " => admin_guard => forbidden_user"]
    Admin { id: String, children: AdminRoutes },
    #[default_route]
    #[local_view = " => not_found"]
    NotFound,
    #[local_view = " => forbidden"]
    Forbidden,
    #[as_path = ""]
    #[local_view = "theme => home"]
    Home,
}

fn guard(model: &Model) -> Option<bool> {
    // could check local storage, cookie or what ever you want
    if model.logged_user.is_some() {
        Some(true)
    } else {
        None
    }
}

fn admin_guard(model: &Model) -> Option<bool> {
    // could check local storage, cookie or what ever you want
    if let Some(user) = &model.logged_user {
        match user.role {
            Role::StandardUser => Some(false),
            Role::Admin => Some(true),
        }
    } else {
        None
    }
}

fn not_found(model: &Model) -> Node<Msg> {
    div!["404"]
}

fn forbidden(model: &Model) -> Node<Msg> {
    div!["401"]
}

fn forbidden_user(model: &Model) -> Node<Msg> {
    if let Some(user) = &model.logged_user {
        p![format!("Sorry {} {} , but you are missing the Admin Role. Ask your administrator for for information. ", user.first_name, user.last_name )]
    } else {
        div!["401"]
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    state: State,
    router: Router<Routes>,
    logged_user: Option<LoggedUser>,
    theme: Theme,
}

// ------ State for component ------
#[derive(Default)]
pub struct State {
    pub register: pages::register::Model,
    pub login: pages::login::Model,
    pub dashboard: pages::dashboard::Model,
    pub admin: pages::admin::Model,
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
    Admin(pages::admin::Msg),
    UserLogged(LoggedUser),
    Dashboard(pages::dashboard::Msg),
    GoBack,
    GoForward,
    Logout,
    GoLogin,
    SwitchToTheme(Theme),
}

/// Main update for the entire APP, every component action/message should me
/// mapped there because of single truth of path
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("URL has changed");

            model.router.confirm_navigation(url);

            if let Some(current_route) = model.router.current_route.clone() {
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

        Msg::Admin(admin_msg) => pages::admin::update(
            admin_msg,
            &mut model.state.admin,
            &mut orders.proxy(Msg::Admin),
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
        Msg::Logout => model.logged_user = None,
        Msg::GoLogin => {
            model.router.current_route = Some(Routes::Login {
                query: IndexMap::new(),
            })
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

// /// Auto generated by proc macro attribute and called inside view
// impl Guarded<Routes, Model, Msg> for Routes {
//     fn check_before_load(&self, scoped_state: &Model) -> Option<bool> {
//         if scoped_state.logged_user.is_some() {
//             // this party will be a function which the user has full control on, could be use for user permission as well
//             Some(true)
//         } else {
//             None
//         }
//     }
// }

fn header(model: &Model) -> Node<Msg> {
    div![
        TopBar::new(who_is_connected(model))
            .style(model.theme.clone())
            .set_user_login_state(model.logged_user.is_some())
            .content(div![
                style! {St::Display => "flex" },
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
                ],
                span![style! {St::Flex => "5" },],
                build_account_button(model.logged_user.is_some())
            ]),
        render_route(model)
    ]
}

fn who_is_connected(model: &Model) -> String {
    if let Some(user) = &model.logged_user {
        let full_welcome = format!("Welcome {} {}", user.first_name, user.last_name);
        full_welcome
    } else {
        "Welcome Guest".to_string()
    }
}

fn build_account_button(user_logged_in: bool) -> Node<Msg> {
    if user_logged_in {
        span![button![
            "logout ",
            ev(Ev::Click, |_| Msg::Logout),
            C!["user_button"],
            i![C!["far fa-user-circle"]]
        ]]
    } else {
        span![button![
            "sign in ",
            ev(Ev::Click, |_| Msg::GoLogin),
            C!["user_button"],
            i![C!["fas fa-user-circle"]]
        ]]
    }
}

fn make_query_for_john_doe() -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    query.insert("name".to_string(), "JohnDoe".to_string());
    query
}

fn render_route(model: &Model) -> Node<Msg> {
    ul![
        li![a![
            C![
                "route",
                IF!( model.router.is_current_route(&Routes::Login { query : IndexMap::new() }) => "active-route" )
            ],
            attrs! { At::Href => Routes::Login { query : IndexMap::new() }.to_url() },
            "Login",
        ]],
        li![a![
            C![
                "route",
                IF!( model.router.is_current_route(&Routes::Login { query : make_query_for_john_doe() }) => "active-route" )
            ],
            attrs! { At::Href => Routes::Login { query : make_query_for_john_doe() }.to_url()  },
            "Login for JohnDoe",
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
        li![a![C!["route"], "Admin",]],
        ul![
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "1".to_string() , children : AdminRoutes::Root}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Admin { id : "1".to_string() , children : AdminRoutes::Root}) },
                "Admin project 1",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "2".to_string() , children : AdminRoutes::Root}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Admin { id : "2".to_string() , children : AdminRoutes::Root}) },
                "Admin project 2",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "3".to_string() , children : AdminRoutes::Root}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Admin { id : "3".to_string() , children : AdminRoutes::Root}) },
                "Admin project 3",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "3".to_string() , children : AdminRoutes::NotFound}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Admin { id : "3".to_string() , children : AdminRoutes::NotFound}) },
                "Not found project 3",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "1".to_string() , children : AdminRoutes::Manager}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => model.router.url(&Routes::Admin { id : "1".to_string() , children : AdminRoutes::Manager}) },
                "Manage project 1",
            ]],
        ],
        li![a![C!["route"], "Dashboard",]],
        ul![
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Root)) => "active-route" ),
                    IF!(guard(model).is_none() => "locked-route"   )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Root)) },
                "Profile",
            ]],
            li![a![
                C![
                     "route",
                     IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Message)) => "active-route" )
                IF!(guard(model).is_none() => "locked-route"   )
                 ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Message)) },
                "Messages",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Statistics)) => "active-route" )
                       IF!(guard(model).is_none() => "locked-route"   )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Statistics)) },
                "Statistics",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Root))) => "active-route" )
                    IF!(guard(model).is_none() => "locked-route"   )
                ],
                attrs! { At::Href => model.router.url(&Routes::Dashboard(DashboardRoutes::Tasks(TasksRoutes::Root))) },
                "Tasks",
            ]],
        ],
    ]
}

// fn cannot_user_access_dashboard(model: &Model) -> bool {
//     Routes::Dashboard(DashboardRoutes::Root)
//         .check_before_load(model)
//         .is_none()
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
