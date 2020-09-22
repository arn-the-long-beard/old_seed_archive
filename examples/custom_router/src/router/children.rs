use crate::router::route::Route;
use std::collections::HashMap;

pub trait ExtractRoutes {
    fn get_routes() -> Vec<Route>;
    fn get_hashed_routes() -> HashMap<String, Route>;
}
