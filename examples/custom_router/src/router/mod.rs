// use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;
// use seed::Url;
// use std::fmt::Display;
// use strum::{EnumProperty, IntoEnumIterator};
pub mod children;
pub mod route;
pub mod state;
pub mod super_router;
pub mod url;
pub mod view;
// // impl Clone for ExampleRoutes {
// //     fn clone(&self) -> Self {
// //         *self
// //     }
// // }
// // ------ ------
// //     Urls
// // ------ ------
// use crate::router::children::ExtractRoutes;
// use crate::router::route::Route;
// use heck::SnakeCase;
//
// use seed::{*, *};
// struct_urls!();
// /// Construct url injected in the web browser with path
// impl<'a> Urls<'a> {
//     pub fn build_url(self, segments: Vec<&str>) -> Url {
//         self.base_url().set_path(segments)
//     }
// }
// // pub mod children;
// // pub mod route;
// pub enum Move {
//     IsNavigating,
//     IsMovingBack,
//     IsMovingForward,
//     IsReady,
// }
//
// pub struct Router<Routes: IntoEnumIterator + Copy + Clone + PartialEq> {
//     pub routes_before_init: Vec<Route>,
//     pub current_route: Option<Route>,
//     pub current_route_variant: Option<Routes>,
//     pub current_history_index: usize,
//     pub default_route: Route,
//     base_url: Url,
//     pub current_move: Move,
//     history: Vec<Route>,
//     root_enum_routes: Vec<Routes>,
//     routes: HashMap<String, Route>,
// }
//
// impl<
//         Routes: IntoEnumIterator
//             + std::str::FromStr
//             + EnumProperty
//             + Copy
//             + Clone
//             + PartialEq
//             + Display
//             + ExtractRoutes,
//     > Default for Router<Routes>
// {
//     fn default() -> Self {
//         let mut root_routes_vec = Vec::new();
//
//         for route in Routes::iter() {
//             root_routes_vec.push(route)
//         }
//         let mut routes = Routes::get_hashed_routes();
//         let base_url = Url::new();
//         let mut hashed_ready_route: HashMap<String, Route> = HashMap::new();
//         for map in routes {
//             let mut route_with_url = map.1.clone();
//             let split = map.0.as_str().split('/').collect();
//             route_with_url.url = Some(Urls::new(base_url.clone()).build_url(split));
//             hashed_ready_route.insert(map.0, route_with_url.clone());
//         }
//         // let new_hash =routes.map(||)
//
//         Router {
//             current_history_index: 0,
//             routes_before_init: Routes::get_routes(),
//             default_route: Routes::get_default_route(),
//             history: Vec::new(),
//             current_route_variant: None,
//             current_route: None,
//             base_url: Url::new(), // should replace with current ,aybe ?
//             routes: hashed_ready_route,
//             current_move: Move::IsReady,
//             root_enum_routes: root_routes_vec,
//         }
//     }
// }
//
// impl<
//         Routes: IntoEnumIterator
//             + std::str::FromStr
//             + EnumProperty
//             + Copy
//             + Clone
//             + PartialEq
//             + Display
//             + ExtractRoutes,
//     > Router<Routes>
// {
//     pub fn new() -> Router<Routes> {
//         Router::default()
//     }
//
//     pub fn set_base_url(&mut self, url: Url) -> &mut Self {
//         self.base_url = url;
//         self
//     }
//
//     pub fn init_url_and_navigation(&mut self, url: Url) -> &mut Self {
//         self.set_base_url(url.to_base_url());
//         self.navigate_to_url(url);
//         self
//     }
//     // pub fn routes_values(&'static self) -> Values<String> {
//     //     let mut values = &self.routes.values();
//     //     values.clone()
//     // }
//     // pub fn add_route(&mut self, route: Routes, value: &str) -> &mut Self {
//     //     self.routes.insert(value.to_string(), route);
//     //     self
//     // }
//
//     fn push_to_history(&mut self, route: Route) {
//         self.history.push(route);
//         self.current_history_index = self.history.len() - 1;
//     }
//
//     /// Go back in history and navigate back to the previous route
//     ///  # Note for now it does not add to history since we navigate inside
//     fn back(&mut self) -> bool {
//         if let Some(next_route) = self.can_back_with_route() {
//             self.current_route = Some(next_route);
//             self.current_history_index -= 1;
//             true
//         } else {
//             false
//         }
//     }
//
//     /// Check if you can go back in history and give you the right route
//     pub fn can_back_with_route(&self) -> Option<Route> {
//         // If we have no history, cannot go back
//
//         if self.history.is_empty() {
//             return None;
//         }
//         // If the current route is at index 0, we cannot go back more
//         if self.current_history_index == 0 {
//             return None;
//         }
//         let next_index = self.current_history_index - 1;
//         let route = self.history.get(next_index).unwrap();
//         Some(route.clone())
//     }
//
//     pub fn can_back(&self) -> bool {
//         self.can_back_with_route().is_some()
//     }
//     pub fn can_forward(&self) -> bool {
//         self.can_forward_with_route().is_some()
//     }
//
//     /// Check if you can navigate forward in the history
//     pub fn can_forward_with_route(&self) -> Option<Route> {
//         // if there is no route, cannot go forward
//         if self.history.is_empty() {
//             return None;
//         }
//         // If we are on the last index, we cannot go forward neither
//         if self.current_history_index == self.history.len() - 1 {
//             return None;
//         }
//         let next_index = self.current_history_index + 1;
//
//         let route = self.history.get(next_index).unwrap_or_else(|| {
//             panic!(
//                 "We should have get route but index is failed {}",
//                 next_index
//             )
//         });
//         Some(route.clone())
//     }
//
//     /// to move forward in the history
//     /// # Note for now it does not add to history since we navigate inside
//     fn forward(&mut self) -> bool {
//         if let Some(next_route) = self.can_forward_with_route() {
//             self.current_route = Some(next_route);
//             self.current_history_index += 1;
//             true
//         } else {
//             false
//         }
//     }
//
//     pub fn is_current_route(&self, route: Route) -> bool {
//         if let Some(current_route) = self.current_route.clone() {
//             route.eq(&current_route)
//         } else {
//             false
//         }
//     }
//
//     fn reload_without_cache() {}
//
//     /// Go to the next url with the associated route
//     /// This will push to history. So If you go back multiple time and then use
//     /// navigate and then go back, you will not get the previous page, but the
//     /// one just pushed into history before
//     pub fn navigate_to_new(&mut self, route: Route) {
//         self.current_route = Some(route.clone());
//         self.push_to_history(route.clone());
//         if let Some(url) = &route.url {
//             let path = url.path();
//             if path.len() == 1 {
//                 self.current_route_variant = route.clone().get_variant::<Routes>();
//             } else {
//             }
//         } else {
//             self.current_route_variant = Routes::from_str(self.default_route.path.as_str()).ok()
//         }
//     }
//
//     /// Match the url that change and update the router with the new current
//     /// Route
//     fn navigate_to_url(&mut self, mut url: Url) {
//         let mut path = &mut url.to_string();
//         let other_path = &mut url.path();
//
//         log!(other_path);
//         path.remove(0); // every url start with / so we need to remove it to match the path
//
//         if let Some(route_match) = self.routes.get(path) {
//             // log!("found route");
//             self.navigate_to_new(route_match.clone());
//         } else {
//             // log!("404");
//
//             self.navigate_to_new(self.default_route.clone());
//         }
//     }
//
//     pub fn request_moving_back<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
//         self.current_move = Move::IsMovingBack;
//         if let Some(next_route) = &self.can_back_with_route() {
//             func(next_route.clone().url.unwrap());
//         }
//     }
//     pub fn request_moving_forward<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
//         self.current_move = Move::IsMovingForward;
//         if let Some(next_route) = &self.can_forward_with_route() {
//             func(next_route.clone().url.unwrap());
//         }
//     }
//     pub fn base_url(&self) -> &Url {
//         &self.base_url
//     }
//
//     /// This method accept a given url and choose the appropriate update for the
//     /// history
//     /// It also reset the current move
//     pub fn confirm_navigation(&mut self, url: Url) {
//         match self.current_move {
//             Move::IsNavigating => {
//                 self.navigate_to_url(url);
//             }
//             Move::IsMovingBack => {
//                 self.back();
//             }
//             Move::IsMovingForward => {
//                 self.forward();
//             }
//             Move::IsReady => {
//                 self.navigate_to_url(url);
//             }
//         }
//         self.current_move = Move::IsReady;
//     }
//     pub fn mapped_routes(&self) -> Vec<AvailableRoute> {
//         let mut list: Vec<AvailableRoute> = Vec::new();
//         for hashed_route in self.routes.clone() {
//             let path = hashed_route.0;
//             let route = hashed_route.1.clone();
//             let is_active = self.is_current_route(route.clone());
//             let url = route.clone().url;
//             list.push(AvailableRoute {
//                 url: url.unwrap(),
//                 path,
//                 is_active,
//                 name: route.name,
//             })
//         }
//         list
//     }
// }
// pub struct AvailableRoute {
//     pub url: Url,
//     pub is_active: bool,
//     pub path: String,
//     pub name: String,
// }
// #[cfg(test)]
// mod test {
//     use crate::router::{Router, Urls};
//     extern crate router_macro_derive;
//
//     use crate::router::children::ExtractRoutes;
//     use crate::router::route::Route;
//     use router_macro_derive::Routes;
//     use seed::Url;
//     use std::collections::HashMap;
//     use std::str::FromStr;
//     use strum::{EnumProperty, IntoEnumIterator};
//     use wasm_bindgen_test::*;
//
//     use super::*;
//
//     wasm_bindgen_test_configure!(run_in_browser);
//
//     #[derive(EnumIter, EnumString, EnumProperty, Display, Debug, Copy, Clone, PartialEq, Routes)]
//     #[strum(serialize_all = "snake_case")]
//     pub enum DashboardAdminRoutes {
//         #[strum(serialize = "")]
//         #[strum(props(Default = "true"))]
//         DashRoot,
//         Other2,
//     }
//     impl Default for DashboardAdminRoutes {
//         fn default() -> DashboardAdminRoutes {
//             DashboardAdminRoutes::DashRoot
//         }
//     }
//     #[derive(EnumIter, EnumString, EnumProperty, Display, Debug, Copy, Clone, PartialEq, Routes)]
//     #[strum(serialize_all = "snake_case")]
//     pub enum DashboardRoutes {
//         #[strum(props(Default = "true"))]
//         #[strum(serialize = "")]
//         Root,
//         Other {
//             children: DashboardAdminRoutes,
//         },
//     }
//
//     impl Default for DashboardRoutes {
//         fn default() -> DashboardRoutes {
//             DashboardRoutes::Root
//         }
//     }
//     #[derive(EnumIter, EnumString, EnumProperty, Display, Debug, Copy, Clone, PartialEq, Routes)]
//     #[strum(serialize_all = "snake_case")]
//     enum ExampleRoutes {
//         #[strum(serialize = "")]
//         Home,
//         Login,
//         Register,
//         Stuff,
//         Dashboard {
//             children: DashboardRoutes,
//         },
//         #[strum(props(Default = "true"))]
//         NotFound,
//     }
//
//     #[test]
//     fn test_names() {
//         let name = ExampleRoutes::Dashboard {
//             children: DashboardRoutes::Root,
//         }
//         .to_string();
//
//         assert_eq!(name, "dashboard");
//     }
//     #[wasm_bindgen_test]
//     fn test_iteration() {
//         for route in ExampleRoutes::iter() {
//             // println!("the route is {:?}", route);
//             // println!("stuff {:?}", answer());
//         }
//         assert_eq!(ExampleRoutes::iter().len(), 6);
//     }
//     #[wasm_bindgen_test]
//     fn test_root_routes_generation() {
//         let hashed_mapped_routes = ExampleRoutes::get_hashed_routes();
//         for map in &hashed_mapped_routes {
//             println!("url : {:?} - Route {:?} ", map.0, map.1);
//         }
//         let len: u8 = hashed_mapped_routes.len() as u8;
//         assert_eq!(len, 10);
//     }
//     #[wasm_bindgen_test]
//     fn test_children_routes_generation() {
//         let hashed_mapped_routes = DashboardRoutes::get_hashed_routes();
//         for map in &hashed_mapped_routes {
//             println!("url : {:?} - Route {:?} ", map.0, map.1);
//         }
//         let len: u8 = hashed_mapped_routes.len() as u8;
//         assert_eq!(len, 4);
//     }
//     #[wasm_bindgen_test]
//     fn test_great_children_routes_generation() {
//         let hashed_mapped_routes = DashboardAdminRoutes::get_hashed_routes();
//         for map in &hashed_mapped_routes {
//             println!("url : {:?} - Route {:?} ", map.0, map.1);
//         }
//         let len: u8 = hashed_mapped_routes.len() as u8;
//         assert_eq!(len, 2);
//     }
//
//     #[wasm_bindgen_test]
//     fn test_build_router() {
//         let router: Router<ExampleRoutes> = Router::new();
//         let routes = router.routes;
//
//         for map in &routes {
//             println!("url : {:?} - Route {:?} ", map.0, map.1);
//         }
//         assert_eq!(routes[""].path, ExampleRoutes::Home.to_string());
//         assert_eq!(routes["login"].path, ExampleRoutes::Login.to_string());
//         assert_eq!(routes.get("sdsadsda").is_none(), true);
//         assert_eq!(routes["not_found"].default, true);
//         assert_eq!(router.default_route, routes["not_found"])
//     }
//
//     #[wasm_bindgen_test]
//     fn test_build_url() {
//         let router: Router<ExampleRoutes> = Router::new();
//         let url = router.base_url().clone().add_path_part("");
//         let root_route = &router.routes[""].clone();
//         let url_from_new = Urls::build_url(Urls::new(Url::new()), Vec::from([""]));
//         assert_eq!(url_from_new.path(), url.path());
//
//         let other2_route = &router.routes["dashboard/other/other2"].clone();
//         let url_from_new = Urls::build_url(
//             Urls::new(Url::new()),
//             Vec::from(["dashboard", "other", "other2"]),
//         );
//         // eprintln!("{:?}", url.path());
//         // eprintln!("{:?}", url_from_router.path());
//
//         assert_eq!(
//             url_from_new.path(),
//             other2_route.url.clone().unwrap().path()
//         );
//     }
//     // #[wasm_bindgen_test]
//     #[test]
//     fn test_navigation_to_route() {
//         let mut router: Router<ExampleRoutes> = Router::new();
//
//         router.navigate_to_new(router.routes[""].clone());
//
//         assert_eq!(router.current_route.clone().unwrap(), router.routes[""]);
//         assert_eq!(router.current_history_index, 0);
//
//         // assert_eq!(router.current_route_variant.unwrap(), ExampleRoutes::Home);
//
//         router.navigate_to_new(router.routes["login"].clone());
//
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["login"]
//         );
//         assert_eq!(router.current_history_index, 1);
//         router.navigate_to_new(router.routes["dashboard/other/other2"].clone());
//
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["dashboard/other/other2"].clone()
//         );
//         assert_eq!(router.current_history_index, 2);
//     }
//     #[wasm_bindgen_test]
//     fn test_navigation_to_url() {
//         let mut router: Router<ExampleRoutes> = Router::new();
//
//         router.navigate_to_url(router.routes[""].clone().url.unwrap());
//
//         assert_eq!(router.current_route.clone().unwrap(), router.routes[""]);
//         assert_eq!(router.current_history_index, 0);
//
//         router.navigate_to_url(router.routes["login"].clone().url.unwrap());
//
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["login"]
//         );
//         assert_eq!(router.current_history_index, 1);
//         router.navigate_to_url(router.routes["dashboard/other/other2"].clone().url.unwrap());
//
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["dashboard/other/other2"].clone()
//         );
//         assert_eq!(router.current_history_index, 2);
//     }
//     #[wasm_bindgen_test]
//     fn test_backward() {
//         let mut router: Router<ExampleRoutes> = Router::new();
//
//         let back = router.back();
//         assert_eq!(back, false, "We should Not have gone backwards");
//         assert_eq!(
//             router.current_history_index, 0,
//             "We should have current index 0"
//         );
//         assert_eq!(
//             router.current_route.is_none(),
//             true,
//             "We should not have current rou"
//         );
//
//         router.navigate_to_new(router.routes[""].clone());
//         router.navigate_to_new(router.routes["register"].clone());
//         router.navigate_to_new(router.routes["dashboard/other/other2"].clone());
//
//         assert_eq!(router.current_history_index, 2);
//
//         let back = router.back();
//         assert_eq!(back, true, "We should have gone backwards");
//         assert_eq!(router.current_history_index, 1);
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["register"].clone()
//         );
//         assert_eq!(
//             router.is_current_route(router.routes["register"].clone()),
//             true
//         );
//         let back = router.back();
//         assert_eq!(back, true, "We should have gone backwards");
//         assert_eq!(router.current_history_index, 0);
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes[""].clone()
//         );
//         assert_eq!(router.is_current_route(router.routes[""].clone()), true);
//         router.navigate_to_new(router.routes["dashboard/other/other2"].clone());
//         assert_eq!(
//             router.is_current_route(router.routes["dashboard/other/other2"].clone()),
//             true
//         );
//         println!("{:?}", router.current_route);
//         println!(
//             "{:?}",
//             router.current_route.clone().unwrap().url.unwrap().path()
//         );
//         println!("{:?}", router.current_history_index);
//         let back = router.back();
//         assert_eq!(back, true);
//         // Here is tricky part, after navigate we go back to the end of history, so if
//         // we go back, we go to the previous index
//         assert_eq!(router.current_history_index, 2);
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["dashboard/other/other2"]
//         );
//     }
//
//     #[wasm_bindgen_test]
//     fn test_forward() {
//         let mut router: Router<ExampleRoutes> = Router::new();
//
//         let back = router.back();
//         assert_eq!(back, false, "We should Not have gone backwards");
//         assert_eq!(
//             router.current_history_index, 0,
//             "We should have current index 0"
//         );
//         assert_eq!(
//             router.current_route.is_none(),
//             true,
//             "We should not have current rou"
//         );
//         router.navigate_to_new(router.routes[""].clone());
//         router.navigate_to_new(router.routes["register"].clone());
//         router.navigate_to_new(router.routes["login"].clone());
//         assert_eq!(router.current_history_index, 2);
//
//         let back = router.back();
//         let back = router.back();
//
//         let forward = router.forward();
//         assert_eq!(forward, true, "We should have gone forward");
//         assert_eq!(router.current_history_index, 1);
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["register"].clone()
//         );
//
//         let forward = router.forward();
//         assert_eq!(forward, true, "We should have gone forward");
//         assert_eq!(router.current_history_index, 2);
//         assert_eq!(
//             router.current_route.clone().unwrap(),
//             router.routes["login"].clone()
//         );
//         let forward = router.forward();
//         assert_eq!(forward, false, "We should Not have gone forward");
//     }
// }
