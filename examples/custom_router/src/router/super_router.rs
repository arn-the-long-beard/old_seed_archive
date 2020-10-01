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
        self.current_route = Some(*route);
        self.push_to_history(*route);
    }

    /// Match the url that change and update the router with the new current
    /// Route
    fn navigate_to_url(&mut self, url: Url) {
        let path = &mut url.to_string();
        path.remove(0);
        if let Ok(route_match) = Routes::parse_path(path) {
            // log!("found route");
            self.navigate_to_new(&route_match);
        } else {
            self.navigate_to_new(&self.default_route.expect("Should go back to default route"));
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
        for route in Routes::iter() {
            let is_active = self.is_current_route(&route);
            list.push(AvailableRoute {
                url: self.url(&route),
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
        Profile(u32),
        #[name = ""]
        Root,
    }
    impl Default for DashboardRoutes {
        fn default() -> DashboardRoutes {
            DashboardRoutes::Root
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, EnumIter, AsPath)]
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

    // fn test_router_build() {
    //     // let mut router = SuperRouter::<ExampleRoutes>::new();
    //     // for route in ExampleRoutes::iter() {
    //     //     log!(route);
    //     // }
    //     //
    //     // let home = ExampleRoutes::parse_path("").unwrap();
    //     // let not_found = ExampleRoutes::parse_path("not_found").unwrap();
    //     // let admin = ExampleRoutes::parse_path("dashboard/admin/other").unwrap();
    //     // log!(home);
    //     // log!(admin);
    //     // log!(not_found);
    //     // let compare = ExampleRoutes::Dashboard(DashboardRoutes::Admin(DashboardAdminRoutes::Other));
    //     // assert_eq!(res.unwrap(), &ExampleRoutes::Home);
    //     // assert_eq!(res_2.unwrap(), &ExampleRoutes::NotFound);
    //     // assert_eq!(
    //     //     res_3.unwrap(),
    //     //     &ExampleRoutes::Dashboard(DashboardRoutes::Admin(DashboardAdminRoutes::Other))
    //     // );
    // }
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
    //
    #[wasm_bindgen_test]
    fn test_build_url() {
        let mut router: SuperRouter<ExampleRoutes> = SuperRouter::new();
        let url = router.base_url().clone().add_path_part("");
        router.navigate_to_url(url);
        assert_eq!(
            router.current_route.unwrap(),
            ExampleRoutes::parse_path("").unwrap()
        );

        let admin_url = router
            .base_url()
            .clone()
            .set_path("dashboard/admin/other".split("/"));

        router.navigate_to_url(admin_url);
        assert_eq!(
            router.current_route.unwrap(),
            ExampleRoutes::parse_path("/dashboard/admin/other").unwrap()
        );

        let admin_url = router
            .base_url()
            .clone()
            .set_path("dashboard/profile/1".split("/"));

        router.navigate_to_url(admin_url);
        assert_eq!(
            router.current_route.unwrap(),
            ExampleRoutes::parse_path("/dashboard/profile/1").unwrap()
        );
        // eprintln!("{:?}", url.path());
        // eprintln!("{:?}", url_from_router.path());
    }

    #[wasm_bindgen_test]
    fn test_navigation_to_route() {
        let mut router: SuperRouter<ExampleRoutes> = SuperRouter::new();

        router.navigate_to_new(&ExampleRoutes::parse_path("/dashboard/profile/1").unwrap());

        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
        );
        assert_eq!(router.current_history_index, 0);

        // assert_eq!(router.current_route_variant.unwrap(), ExampleRoutes::Home);

        router.navigate_to_new(&ExampleRoutes::parse_path("/dashboard/profile/55").unwrap());

        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(55))
        );
        assert_eq!(router.current_history_index, 1);
        router.navigate_to_new(&ExampleRoutes::Home);

        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("").unwrap()
        );
        assert_eq!(router.current_history_index, 2);
    }

    // #[test]

    #[wasm_bindgen_test]
    fn test_backward() {
        let mut router: SuperRouter<ExampleRoutes> = SuperRouter::new();

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

        router.navigate_to_new(&ExampleRoutes::parse_path("").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("register").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("dashboard/admin/other").unwrap());

        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Register
        );
        assert_eq!(router.is_current_route(&ExampleRoutes::Register), true);
        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 0);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("").unwrap()
        );

        router.navigate_to_new(&ExampleRoutes::Dashboard(DashboardRoutes::Root));
        assert_eq!(
            router.is_current_route(&ExampleRoutes::parse_path("dashboard/").unwrap()),
            true
        );

        let back = router.back();
        assert_eq!(back, true);
        // Here is tricky part, after navigate we go back to the end of history, so if
        // we go back, we go to the previous index
        assert_eq!(router.current_history_index, 2);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("dashboard/admin/other").unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_forward() {
        let mut router: SuperRouter<ExampleRoutes> = SuperRouter::new();

        let back = router.forward();
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
        router.navigate_to_new(&ExampleRoutes::parse_path("").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("register").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("/dashboard/profile/55").unwrap());
        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        let back = router.back();

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Register
        );

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 2);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(55))
        );
        let forward = router.forward();
        assert_eq!(forward, false, "We should Not have gone forward");
    }
}
