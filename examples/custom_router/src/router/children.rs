use crate::router::route::Route;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;

//
// pub trait ChildrenRoutes<T> {
//     fn children_routes(&self) -> Option<T>
//     where
//         Self: IntoEnumIterator
//             + std::str::FromStr
//             + EnumProperty
//             + Copy
//             + Clone
//             + PartialEq
//             + Display;

// fn children_routes_for_route<C>(&self, route: T) -> Option<C>
// where
//     Self: IntoEnumIterator
//         + std::str::FromStr
//         + EnumProperty
//         + Copy
//         + Clone
//         + PartialEq
//         + Display;
// }
pub trait ExtractRoutes {
    fn get_routes() -> HashMap<String, Route>;
}
