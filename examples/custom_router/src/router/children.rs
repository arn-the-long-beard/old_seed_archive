use crate::router::route::Route;
use std::collections::HashMap;

pub trait ExtractRoutes {
    fn get_routes() -> Vec<Route>;
    /// This method get the children routes in hash as well recursively
    fn get_hashed_routes() -> HashMap<String, Route>;
    fn get_default_route() -> Route {
        let routes = Self::get_routes();
        let default_route = routes.iter().find(|r| r.default);
        if default_route.is_none() {
            panic!("You need a default route for your routing to redirect when wrong url/path");
        }
        let def = default_route.unwrap().clone();
        def.clone()
    }
}
