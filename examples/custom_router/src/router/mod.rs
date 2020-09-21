use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;
use seed::Url;
use std::fmt::Display;
use strum::{EnumProperty, IntoEnumIterator};
pub mod children;
pub mod route;
// impl Clone for ExampleRoutes {
//     fn clone(&self) -> Self {
//         *self
//     }
// }
// ------ ------
//     Urls
// ------ ------
use crate::router::children::ExtractRoutes;
use heck::SnakeCase;
use seed::{*, *};
struct_urls!();
/// Construct url injected in the web browser with path
impl<'a> Urls<'a> {
    pub fn build_url(self, path: &str) -> Url {
        self.base_url().add_path_part(path.to_snake_case())
    }
}
// pub mod children;
// pub mod route;
pub enum Move {
    IsNavigating,
    IsMovingBack,
    IsMovingForward,
    IsReady,
}

pub struct Router<Routes: IntoEnumIterator + Copy + Clone + PartialEq> {
    pub routes: HashMap<String, Routes>,
    pub current_route: Option<Routes>,
    pub current_history_index: usize,
    pub default_route: Routes,
    base_url: Url,
    pub current_move: Move,
    history: Vec<Routes>,
}

// pub struct RouteBuilder<Routes:  IntoEnumIterator+ Copy + Clone + PartialEq> {
//     route: Routes,
//     path: Option<String>,
//     guard: Option<bool>,
// }
//
// impl<Routes:  IntoEnumIterator+ Copy + Clone + PartialEq> RouteBuilder<Routes> {
//     pub fn new(route: Routes) -> RouteBuilder<Routes> {
//         RouteBuilder {
//             route,
//             path: None,
//             guard: None,
//         }
//     }
// }
pub fn build<
    Routes: IntoEnumIterator + EnumProperty + Copy + Clone + PartialEq + Display + ExtractRoutes,
>() -> HashMap<String, Routes> {
    let mut hash_map = HashMap::new();
    for route in Routes::iter() {
        hash_map.insert(route.to_string(), route);
    }
    hash_map
}

#[derive(Debug)]
pub struct ExtractedRoute<
    Routes: IntoEnumIterator + EnumProperty + EnumProperty + Copy + Clone + PartialEq + ExtractRoutes,
> {
    pub url: Url,
    pub is_active: bool,
    pub path: String,
    pub route: Routes,
}
impl<
        Routes: IntoEnumIterator
            + std::str::FromStr
            + EnumProperty
            + Copy
            + Clone
            + PartialEq
            + Display
            + ExtractRoutes,
    > Router<Routes>
{
    pub fn new() -> Router<Routes> {
        let mut hash_map: HashMap<String, Routes> = HashMap::new();
        let mut default_route: Option<Routes> = None;

        for route in Routes::iter() {
            if route.get_str("Children").is_some() {
                let value = route.get_str("Children").unwrap();
                // println!("{:?}", value);
                // println!("{:?}", route.to_string());
                hash_map.insert(route.to_string(), route);
            } else {
                hash_map.insert(route.to_string(), route);
            }

            if route.get_str("Default").is_some() {
                default_route = Some(route);
            }
        }
        if default_route.is_none() {
            panic!("You need a default route for your routing to redirect when wrong url/path")
        }
        let def = default_route.unwrap();
        Router {
            current_history_index: 0,
            routes: hash_map,
            default_route: def,
            history: Vec::new(),
            current_route: None,
            base_url: Url::new(), // should replace with current ,aybe ?
            current_move: Move::IsReady,
        }
    }

    pub fn set_base_url(&mut self, url: Url) -> &mut Self {
        self.base_url = url;
        self
    }

    pub fn init_url_and_navigation(&mut self, url: Url) -> &mut Self {
        self.set_base_url(url.to_base_url());
        self.navigate_to_url(url);
        self
    }
    // pub fn routes_values(&'static self) -> Values<String> {
    //     let mut values = &self.routes.values();
    //     values.clone()
    // }
    pub fn add_route(&mut self, route: Routes, value: &str) -> &mut Self {
        self.routes.insert(value.to_string(), route);
        self
    }

    fn push_to_history(&mut self, route: Routes) {
        self.history.push(route);
        self.current_history_index = self.history.len() - 1;
    }

    /// Go back in history and navigate back to the previous route
    ///  # Note for now it does not add to history since we navigate inside
    fn back(&mut self) -> bool {
        if let Some(next_route) = self.can_back_with_route() {
            self.current_route = Some(next_route);
            self.current_history_index -= 1;
            true
        } else {
            false
        }
    }

    /// Check if you can go back in history and give you the right route
    pub fn can_back_with_route(&self) -> Option<Routes> {
        // If we have no history, cannot go back

        if self.history.is_empty() {
            return None;
        }
        // If the current route is at index 0, we cannot go back more
        if self.current_history_index == 0 {
            return None;
        }
        let next_index = self.current_history_index - 1;
        let route = self.history.get(next_index).unwrap();
        Some(*route)
    }

    pub fn can_back(&self) -> bool {
        self.can_back_with_route().is_some()
    }
    pub fn can_forward(&self) -> bool {
        self.can_forward_with_route().is_some()
    }

    /// Check if you can navigate forward in the history
    pub fn can_forward_with_route(&self) -> Option<Routes> {
        // if there is no route, cannot go forward
        if self.history.is_empty() {
            return None;
        }
        // If we are on the last index, we cannot go forward neither
        if self.current_history_index == self.history.len() - 1 {
            return None;
        }
        let next_index = self.current_history_index + 1;

        let route = self.history.get(next_index).unwrap_or_else(|| {
            panic!(
                "We should have get route but index is failed {}",
                next_index
            )
        });
        Some(*route)
    }

    /// to move forward in the history
    /// # Note for now it does not add to history since we navigate inside
    fn forward(&mut self) -> bool {
        if let Some(next_route) = self.can_forward_with_route() {
            self.current_route = Some(next_route);
            self.current_history_index += 1;
            true
        } else {
            false
        }
    }

    pub fn is_current_route(&self, route: Routes) -> bool {
        if let Some(current_route) = self.current_route {
            route.eq(&current_route)
        } else {
            false
        }
    }

    fn reload_without_cache() {}

    /// Go to the next url with the associated route
    /// This will push to history. So If you go back multiple time and then use
    /// navigate and then go back, you will not get the previous page, but the
    /// one just pushed into history before
    pub fn navigate_to_new(&mut self, route: Routes) {
        self.current_route = Some(route);
        self.push_to_history(route);
    }

    /// Match the url that change and update the router with the new current
    /// Route
    fn navigate_to_url(&mut self, mut url: Url) {
        let path_result = url.next_path_part();
        if let Some(path) = path_result {
            if let Some(route_match) = self.mapped_routes().iter().find(|r| r.path == path) {
                self.navigate_to_new(route_match.route);
            } else {
                self.navigate_to_new(self.default_route);
            }
        } else {
            self.navigate_to_new(self.default_route);
        }
    }

    pub fn url(&self, route: Routes) -> Url {
        Urls::new(&self.base_url).build_url(route.to_string().as_str())
    }

    pub fn request_moving_back<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
        self.current_move = Move::IsMovingBack;
        if let Some(next_route) = &self.can_back_with_route() {
            func(self.url(*next_route));
        }
    }
    pub fn request_moving_forward<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
        self.current_move = Move::IsMovingForward;
        if let Some(next_route) = &self.can_forward_with_route() {
            func(self.url(*next_route));
        }
    }
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// This method accept a given url and choose the appropriate update for the
    /// history
    /// It also reset the current move
    pub fn confirm_navigation(&mut self, url: Url) {
        match self.current_move {
            Move::IsNavigating => {
                self.navigate_to_url(url);
            }
            Move::IsMovingBack => {
                self.back();
            }
            Move::IsMovingForward => {
                self.forward();
            }
            Move::IsReady => {
                self.navigate_to_url(url);
            }
        }
        self.current_move = Move::IsReady;
    }
    pub fn mapped_routes(&self) -> Vec<ExtractedRoute<Routes>> {
        let mut list: Vec<ExtractedRoute<Routes>> = Vec::new();
        for route in Routes::iter() {
            let path = route.to_string();
            println!(" path ---> {:?}", path);
            let is_active = self.is_current_route(route);
            list.push(ExtractedRoute {
                url: self.url(route),
                path,
                is_active,
                route,
            })
        }
        list
    }
}

#[cfg(test)]
mod test {
    use crate::router::Router;
    extern crate router_macro_derive;

    use crate::router::children::ExtractRoutes;
    use crate::router::route::Route;
    use router_macro_derive::MyProcMacro;
    use router_macro_derive::Routes;
    use std::collections::HashMap;
    use std::str::FromStr;
    use strum::{EnumProperty, IntoEnumIterator};

    #[derive(EnumIter, EnumString, EnumProperty, Display, Debug, Copy, Clone, PartialEq, Routes)]
    #[strum(serialize_all = "snake_case")]
    pub enum DashboardRoutes {
        #[strum(props(Default = "true"))]
        Root,
        Other,
    }

    impl Default for DashboardRoutes {
        fn default() -> DashboardRoutes {
            DashboardRoutes::Root
        }
    }

    #[derive(EnumIter, EnumString, EnumProperty, Display, Debug, Copy, Clone, PartialEq, Routes)]
    #[strum(serialize_all = "snake_case")]
    enum ExampleRoutes {
        #[strum(serialize = "")]
        Home,
        Login,
        Register,
        Stuff,
        Dashboard {
            children: DashboardRoutes,
        },
        #[strum(props(Default = "true"))]
        NotFound,
    }
    #[test]
    fn test_iteration() {
        for route in ExampleRoutes::iter() {
            // println!("the route is {:?}", route);
            // println!("stuff {:?}", answer());
        }

        assert_eq!(ExampleRoutes::iter().len(), 6);
    }
    #[test]
    fn test_build_router() {
        let router: Router<ExampleRoutes> = Router::new();

        let routes = router.routes.clone();

        // let dashboard = DashboardRoute {
        //     children: DashboardRoutes::Root,
        // };
        // let sub_routes = dashboard.children();

        // assert_eq!(sub_routes, DashboardRoutes::Root);

        // let stuff = dashboard.get_children();
        // dashboard.get_children();

        // let shoe :  = Shoes {
        //     yes: "nice".to_string(),
        // };

        assert_eq!(router.routes[""], ExampleRoutes::Home);
        assert_eq!(router.routes["login"], ExampleRoutes::Login);
        // assert_eq!(
        //     router.routes["dashboard"],
        //     ExampleRoutes::Dashboard {
        //         children: Default::default()
        //     }
        // );

        assert_eq!(router.default_route, ExampleRoutes::NotFound);
        let r = ExampleRoutes::from_str("login").unwrap();

        let routes: HashMap<String, Route> = ExampleRoutes::get_routes();
        let sub_routes = DashboardRoutes::get_routes();

        if !routes["dashboard"].children.is_empty() {
            eprintln!("{:?}", routes["dashboard"].children)
        }
        for r in routes {
            println!(" parent routes {:?}", r);
        }

        for s_r in sub_routes {
            println!(" children routes {:?}", s_r);
        }

        let fake_route = Route {
            path: "fake".to_string(),
            parent_route_path: "".to_string(),
            children: ExampleRoutes::get_routes(),
            guarded: false,
            default: false,
        };
        println!("fake route {:?}", fake_route);

        assert_eq!(router.routes["login"], r);
    }

    #[test]
    fn test_build_url() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home")
            .add_route(ExampleRoutes::Login, "login");

        let url = router.base_url().clone().add_path_part("");
        let url_from_router = router.url(ExampleRoutes::Home);

        // eprintln!("{:?}", url.path());
        // eprintln!("{:?}", url_from_router.path());

        assert_eq!(url_from_router.path(), url.path());
    }
    #[test]
    fn test_navigation() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "")
            .add_route(ExampleRoutes::Login, "login");

        router.navigate_to_new(ExampleRoutes::Home);

        let is_home = matches!(router.current_route.unwrap(), ExampleRoutes::Home);

        assert_eq!(is_home, true);
        assert_eq!(router.current_history_index, 0);

        router.navigate_to_new(ExampleRoutes::Login);

        let is_home = matches!(router.current_route.unwrap(), ExampleRoutes::Home);

        assert_eq!(is_home, false);
        assert_eq!(router.current_history_index, 1);
    }
    #[test]
    fn test_backward() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home")
            .add_route(ExampleRoutes::Login, "login")
            .add_route(ExampleRoutes::Register, "register")
            .add_route(ExampleRoutes::Stuff, "stuff");

        let back = router.back();
        assert_eq!(back, false, "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index, 0,
            "We should have current index 0"
        );
        assert_eq!(
            router.current_route.is_none(),
            true,
            "We should not have current rou"
        );

        router.navigate_to_new(ExampleRoutes::Home);
        router.navigate_to_new(ExampleRoutes::Register);
        router.navigate_to_new(ExampleRoutes::Login);

        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Register);
        assert_eq!(router.is_current_route(ExampleRoutes::Register), true);
        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 0);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Home);
        assert_eq!(router.is_current_route(ExampleRoutes::Home), true);
        router.navigate_to_new(ExampleRoutes::Stuff);
        println!("{:?}", router.current_history_index);
        let back = router.back();
        assert_eq!(back, true);
        // Here is tricky part, after navigate we go back to the end of history, so if
        // we go back, we go to the previous index
        assert_eq!(router.current_history_index, 2);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Login);
    }

    #[test]
    fn test_forward() {
        let mut router: Router<ExampleRoutes> = Router::new();

        let back = router.back();
        assert_eq!(back, false, "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index, 0,
            "We should have current index 0"
        );
        assert_eq!(
            router.current_route.is_none(),
            true,
            "We should not have current rou"
        );

        router.navigate_to_new(ExampleRoutes::Home);
        router.navigate_to_new(ExampleRoutes::Register);
        router.navigate_to_new(ExampleRoutes::Login);

        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        let back = router.back();

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Register);

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 2);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Login);
        let forward = router.forward();
        assert_eq!(forward, false, "We should Not have gone forward");
    }
}
