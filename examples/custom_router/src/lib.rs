#![feature(map_into_keys_values)]

mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::models::user::LoggedUser;
use crate::{
    router::{ExtractedRoute, Router},
    theme::Theme,
    top_bar::TopBar,
};
use heck::SnakeCase;
extern crate strum;
#[macro_use]
extern crate strum_macros;
use strum::IntoEnumIterator;

pub mod models;
mod pages;
mod router;
mod theme;
mod top_bar;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UserLogged);

    let mut router: Router<Routes> = Router::new();
    router.set_base_url(url.to_base_url()).build();

    Model {
        theme: Theme::default(),
        state: Default::default(),
        router,
        logged_user: None,
    }
}

#[derive(EnumIter, Debug, Display, Copy, Clone, PartialEq)] // need to make a derive (Routing) or something maybe
pub enum Routes {
    Home,
    Login,
    Register,
    Dashboard,
    NotFound,
    // Admin(page::admin::Model),
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

            // model.page = Route::init(url);
        }
        Msg::UrlRequested(request) => {
            log!("URL requested");
            let url = request.0;
            model.router.confirm_navigation(url);
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
    if model.logged_user.is_none() {
        vec![
            header(&model),
            if let Some(route) = &model.router.current_route {
                match route {
                    Routes::Home => home(&model.theme),
                    // Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
                    Routes::NotFound => div!["404"],
                    Routes::Login => pages::login::view(&model.state.login).map_msg(Msg::Login),
                    Routes::Register => {
                        pages::register::view(&model.state.register).map_msg(Msg::Register)
                    }
                    _ => div!["404"],
                }
            } else {
                home(&model.theme)
            },
        ]
    } else {
        vec![div!["Authenticated Routing not working"]]
        // vec![
        //     authenticated_header(&model.base_url, &model.page),
        //     match &model.page {
        //         Route::Dashboard => div![div!["Welcome Dashboard!"],],
        //         // Page::Admin(admin_model) => page::admin::view(admin_model,
        // &model.ctx),         Route::NotFound => div!["404"],
        //         _ => div!["404"],
        //     },
        // ]
    }
}

fn header(model: &Model) -> Node<Msg> {
    // let page = &model.page;
    // let base_url = &model.base_url;
    let mut list: Vec<Node<Msg>> = Vec::new();

    for route in &model.router.mapped_routes() {
        list.push(render_route(route));
    }
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
        ul![list]
    ]
}
//

fn render_route(route: &ExtractedRoute<Routes>) -> Node<Msg> {
    li![a![
        C!["route", IF!( route.is_active => "active-route" )],
        attrs! { At::Href => route.url },
        route.path.clone(),
    ]]
}
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
