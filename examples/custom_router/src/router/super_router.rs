use seed::Url;
use std::fmt::{Debug, Display};
use strum::IntoEnumIterator;

// impl Clone for ExampleRoutes {
//     fn clone(&self) -> Self {
//         *self
//     }
// }
// ------ ------
//     Urls
// ------ ------

use enum_paths::{AsPath, ParsePath};
use seed::{*, *};
use std::borrow::Borrow;
struct_urls!();
/// Construct url injected in the web browser with path
impl<'a> Urls<'a> {
    pub fn build_url(self, segments: Vec<&str>) -> Url {
        self.base_url().set_path(segments)
    }
}
// pub mod children;
// pub mod route;
pub enum SuperMove {
    IsNavigating,
    IsMovingBack,
    IsMovingForward,
    IsReady,
}

pub struct SuperRouter<Routes: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone> {
    pub current_route: Option<Routes>,
    pub current_history_index: usize,
    pub default_route: Option<Routes>,
    base_url: Url,
    pub current_move: SuperMove,
    history: Vec<Routes>,
    routes: Vec<Routes>,
}

impl<Routes: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone> Default
    for SuperRouter<Routes>
{
    fn default() -> Self {
        SuperRouter {
            current_history_index: 0,
            default_route: None,
            history: Vec::new(),
            current_route: None,
            base_url: Url::new(), // should replace with current ,aybe ?
            routes: Routes::iter().collect(),
            current_move: SuperMove::IsReady,
        }
    }
}

impl<Routes: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone> SuperRouter<Routes> {
    pub fn new() -> SuperRouter<Routes> {
        SuperRouter::default()
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
    // pub fn add_route(&mut self, route: Routes, value: &str) -> &mut Self {
    //     self.routes.insert(value.to_string(), route);
    //     self
    // }

    fn push_to_history(&mut self, route: Routes) {
        self.history.push(route);
        self.current_history_index = self.history.len() - 1;
    }

    /// Go back in history and navigate back to the previous route
    ///  # Note for now it does not add to history since we navigate inside
    fn back(&mut self) -> bool {
        if let Some(next_route) = self.can_back_with_route() {
            self.current_route = Routes::parse_path(next_route.as_path().as_str()).ok();
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
        Some(route.clone())
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
        Some(route.clone())
    }

    /// to move forward in the history
    /// # Note for now it does not add to history since we navigate inside
    fn forward(&mut self) -> bool {
        if let Some(next_route) = &self.can_forward_with_route() {
            let path: String = next_route.as_path().to_string();
            self.current_route = Routes::parse_path(&path).ok();
            self.current_history_index += 1;
            true
        } else {
            false
        }
    }

    pub fn is_current_route(&self, route: &Routes) -> bool {
        if let Some(current_route) = &self.current_route {
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
    pub fn navigate_to_new(&mut self, route: &Routes) {
        self.current_route = Routes::parse_path(route.as_path().as_str()).ok();
        self.push_to_history(Routes::parse_path(route.as_path().as_str()).unwrap());
    }

    /// Match the url that change and update the router with the new current
    /// Route
    fn navigate_to_url(&mut self, url: Url) {
        let path = &mut url.to_string();

        if let Ok(route_match) = Routes::parse_path(path) {
            // log!("found route");
            self.navigate_to_new(&route_match);
        } else {
            // log!("404");
            self.navigate_to_new(&self.default_route.unwrap());
        }
    }

    pub fn request_moving_back<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
        self.current_move = SuperMove::IsMovingBack;
        if let Some(next_route) = &self.can_back_with_route() {
            func(self.url(next_route));
        }
    }
    pub fn request_moving_forward<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
        self.current_move = SuperMove::IsMovingForward;
        if let Some(next_route) = &self.can_forward_with_route() {
            func(self.url(next_route));
        }
    }
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn url(&self, route: &Routes) -> Url {
        let full_path = route.as_path();
        let segments: Vec<&str> = full_path.as_str().split('/').collect();
        let url = Urls::new(self.base_url.clone()).build_url(segments);
        url
    }
    /// This method accept a given url and choose the appropriate update for the
    /// history
    /// It also reset the current move
    pub fn confirm_navigation(&mut self, url: Url) {
        match self.current_move {
            SuperMove::IsNavigating => {
                self.navigate_to_url(url);
            }
            SuperMove::IsMovingBack => {
                self.back();
            }
            SuperMove::IsMovingForward => {
                self.forward();
            }
            SuperMove::IsReady => {
                self.navigate_to_url(url);
            }
        }
        self.current_move = SuperMove::IsReady;
    }
    pub fn mapped_routes(&self) -> Vec<AvailableRoute> {
        let mut list: Vec<AvailableRoute> = Vec::new();
        for route in &self.routes {
            let is_active = self.is_current_route(route);
            list.push(AvailableRoute {
                url: self.url(route),
                path: route.as_path(),
                is_active,
                name: route.as_path(),
            })
        }
        list
    }
}
pub struct AvailableRoute {
    pub url: Url,
    pub is_active: bool,
    pub path: String,
    pub name: String,
}
#[cfg(test)]
mod test {
    use crate::router::{Router, Urls};
    extern crate enum_paths;
    extern crate router_macro_derive;

    use super::*;
    use enum_paths::{AsPath, ParseError, ParsePath};
    use seed::Url;
    use strum::IntoEnumIterator;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Copy, Clone, AsPath)]
    pub enum DashboardAdminRoutes {
        Other,
        #[name = ""]
        Root,
    }
    impl Default for DashboardAdminRoutes {
        fn default() -> DashboardAdminRoutes {
            DashboardAdminRoutes::Root
        }
    }

    #[derive(Debug, PartialEq, Copy, Clone, AsPath)]
    pub enum DashboardRoutes {
        Admin(DashboardAdminRoutes),
        #[name = ""]
        Root,
    }
    impl Default for DashboardRoutes {
        fn default() -> DashboardRoutes {
            DashboardRoutes::Root
        }
    }
    #[derive(Debug, EnumProperty, PartialEq, Copy, Clone, EnumIter, AsPath)]
    enum ExampleRoutes {
        Login,
        Register,
        Stuff,
        Dashboard(DashboardRoutes),
        NotFound,
        #[name = ""]
        Home,
    }

    #[wasm_bindgen_test]
    fn test_iteration() {
        for route in ExampleRoutes::iter() {
            // println!("the route is {:?}", route);
            // println!("stuff {:?}", answer());
        }
        assert_eq!(ExampleRoutes::iter().len(), 6);
    }
    #[wasm_bindgen_test]
    fn test_router_build() {
        let mut router = SuperRouter::<ExampleRoutes>::new();
        for route in ExampleRoutes::iter() {
            log!(route);
        }
        let res = router
            .routes
            .iter()
            .find(|r| <ExampleRoutes>::clone(r).eq(&ExampleRoutes::Home));
        let home = ExampleRoutes::parse_path("").unwrap();
        let not_found = ExampleRoutes::parse_path("not_found").unwrap();
        let admin = ExampleRoutes::parse_path("dashboard/admin/other").unwrap();
        log!(home);
        log!(admin);
        log!(not_found);
        let compare = ExampleRoutes::Dashboard(DashboardRoutes::Admin(DashboardAdminRoutes::Other));
        log!(compare);
        let res = router
            .routes
            .iter()
            .find(|r| <ExampleRoutes>::clone(r).eq(&home));

        let res_2 = router
            .routes
            .iter()
            .find(|r| <ExampleRoutes>::clone(r).eq(&not_found));

        let res_3 = router
            .routes
            .iter()
            .find(|r| <ExampleRoutes>::clone(r).eq(&admin));

        assert_eq!(res.unwrap(), &ExampleRoutes::Home);
        assert_eq!(res_2.unwrap(), &ExampleRoutes::NotFound);
        assert_eq!(
            res_3.unwrap(),
            &ExampleRoutes::Dashboard(DashboardRoutes::Admin(DashboardAdminRoutes::Other))
        );
    }
    #[wasm_bindgen_test]
    fn test_router_default_route() {
        let mut router = SuperRouter::<ExampleRoutes>::new();
        //todo we should have attribute in the enum for default route
        router.default_route = Some(ExampleRoutes::NotFound);

        let url = Url::new().add_path_part("example");
        router.navigate_to_url(url);
        assert_eq!(router.current_route.unwrap(), router.default_route.unwrap());
    }
    // #[test]
    // fn test_children_routes_generation() {
    //     let hashed_mapped_routes = DashboardRoutes::get_hashed_routes();
    //     for map in &hashed_mapped_routes {
    //         println!("url : {:?} - Route {:?} ", map.0, map.1);
    //     }
    //     let len: u8 = hashed_mapped_routes.len() as u8;
    //     assert_eq!(len, 4);
    // }
    // #[test]
    // fn test_great_children_routes_generation() {
    //     let hashed_mapped_routes = DashboardAdminRoutes::get_hashed_routes();
    //     for map in &hashed_mapped_routes {
    //         println!("url : {:?} - Route {:?} ", map.0, map.1);
    //     }
    //     let len: u8 = hashed_mapped_routes.len() as u8;
    //     assert_eq!(len, 2);
    // }
    //
    // #[test]
    // fn test_build_router() {
    //     let router: Router<ExampleRoutes> = Router::new();
    //     let routes = router.routes;
    //
    //     for map in &routes {
    //         println!("url : {:?} - Route {:?} ", map.0, map.1);
    //     }
    //     assert_eq!(routes[""].path, ExampleRoutes::Home.to_string());
    //     assert_eq!(routes["login"].path, ExampleRoutes::Login.to_string());
    //     assert_eq!(routes.get("sdsadsda").is_none(), true);
    //     assert_eq!(routes["not_found"].default, true);
    //     assert_eq!(router.default_route, routes["not_found"])
    // }
    //
    // #[test]
    // fn test_build_url() {
    //     let router: Router<ExampleRoutes> = Router::new();
    //     let url = router.base_url().clone().add_path_part("");
    //     let root_route = &router.routes[""].clone();
    //     let url_from_new = Urls::build_url(Urls::new(Url::new()), Vec::from([""]));
    //     assert_eq!(url_from_new.path(), url.path());
    //
    //     let other2_route = &router.routes["dashboard/other/other2"].clone();
    //     let url_from_new = Urls::build_url(
    //         Urls::new(Url::new()),
    //         Vec::from(["dashboard", "other", "other2"]),
    //     );
    //     // eprintln!("{:?}", url.path());
    //     // eprintln!("{:?}", url_from_router.path());
    //
    //     assert_eq!(
    //         url_from_new.path(),
    //         other2_route.url.clone().unwrap().path()
    //     );
    // }
    // // #[test]
    // #[test]
    // fn test_navigation_to_route() {
    //     let mut router: Router<ExampleRoutes> = Router::new();
    //
    //     router.navigate_to_new(router.routes[""].clone());
    //
    //     assert_eq!(router.current_route.clone().unwrap(), router.routes[""]);
    //     assert_eq!(router.current_history_index, 0);
    //
    //     // assert_eq!(router.current_route_variant.unwrap(), ExampleRoutes::Home);
    //
    //     router.navigate_to_new(router.routes["login"].clone());
    //
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["login"]
    //     );
    //     assert_eq!(router.current_history_index, 1);
    //     router.navigate_to_new(router.routes["dashboard/other/other2"].clone());
    //
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["dashboard/other/other2"].clone()
    //     );
    //     assert_eq!(router.current_history_index, 2);
    // }
    // #[test]
    // fn test_navigation_to_url() {
    //     let mut router: Router<ExampleRoutes> = Router::new();
    //
    //     router.navigate_to_url(router.routes[""].clone().url.unwrap());
    //
    //     assert_eq!(router.current_route.clone().unwrap(), router.routes[""]);
    //     assert_eq!(router.current_history_index, 0);
    //
    //     router.navigate_to_url(router.routes["login"].clone().url.unwrap());
    //
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["login"]
    //     );
    //     assert_eq!(router.current_history_index, 1);
    //     router.navigate_to_url(router.routes["dashboard/other/other2"].clone().url.unwrap());
    //
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["dashboard/other/other2"].clone()
    //     );
    //     assert_eq!(router.current_history_index, 2);
    // }
    // #[test]
    // fn test_backward() {
    //     let mut router: Router<ExampleRoutes> = Router::new();
    //
    //     let back = router.back();
    //     assert_eq!(back, false, "We should Not have gone backwards");
    //     assert_eq!(
    //         router.current_history_index, 0,
    //         "We should have current index 0"
    //     );
    //     assert_eq!(
    //         router.current_route.is_none(),
    //         true,
    //         "We should not have current rou"
    //     );
    //
    //     router.navigate_to_new(router.routes[""].clone());
    //     router.navigate_to_new(router.routes["register"].clone());
    //     router.navigate_to_new(router.routes["dashboard/other/other2"].clone());
    //
    //     assert_eq!(router.current_history_index, 2);
    //
    //     let back = router.back();
    //     assert_eq!(back, true, "We should have gone backwards");
    //     assert_eq!(router.current_history_index, 1);
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["register"].clone()
    //     );
    //     assert_eq!(
    //         router.is_current_route(router.routes["register"].clone()),
    //         true
    //     );
    //     let back = router.back();
    //     assert_eq!(back, true, "We should have gone backwards");
    //     assert_eq!(router.current_history_index, 0);
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes[""].clone()
    //     );
    //     assert_eq!(router.is_current_route(router.routes[""].clone()), true);
    //     router.navigate_to_new(router.routes["dashboard/other/other2"].clone());
    //     assert_eq!(
    //         router.is_current_route(router.routes["dashboard/other/other2"].clone()),
    //         true
    //     );
    //     println!("{:?}", router.current_route);
    //     println!(
    //         "{:?}",
    //         router.current_route.clone().unwrap().url.unwrap().path()
    //     );
    //     println!("{:?}", router.current_history_index);
    //     let back = router.back();
    //     assert_eq!(back, true);
    //     // Here is tricky part, after navigate we go back to the end of history, so if
    //     // we go back, we go to the previous index
    //     assert_eq!(router.current_history_index, 2);
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["dashboard/other/other2"]
    //     );
    // }
    //
    // #[test]
    // fn test_forward() {
    //     let mut router: Router<ExampleRoutes> = Router::new();
    //
    //     let back = router.back();
    //     assert_eq!(back, false, "We should Not have gone backwards");
    //     assert_eq!(
    //         router.current_history_index, 0,
    //         "We should have current index 0"
    //     );
    //     assert_eq!(
    //         router.current_route.is_none(),
    //         true,
    //         "We should not have current rou"
    //     );
    //     router.navigate_to_new(router.routes[""].clone());
    //     router.navigate_to_new(router.routes["register"].clone());
    //     router.navigate_to_new(router.routes["login"].clone());
    //     assert_eq!(router.current_history_index, 2);
    //
    //     let back = router.back();
    //     let back = router.back();
    //
    //     let forward = router.forward();
    //     assert_eq!(forward, true, "We should have gone forward");
    //     assert_eq!(router.current_history_index, 1);
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["register"].clone()
    //     );
    //
    //     let forward = router.forward();
    //     assert_eq!(forward, true, "We should have gone forward");
    //     assert_eq!(router.current_history_index, 2);
    //     assert_eq!(
    //         router.current_route.clone().unwrap(),
    //         router.routes["login"].clone()
    //     );
    //     let forward = router.forward();
    //     assert_eq!(forward, false, "We should Not have gone forward");
    // }
}
