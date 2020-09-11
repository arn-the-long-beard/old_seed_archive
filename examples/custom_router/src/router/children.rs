use std::any::Any;
use std::fmt::Display;
use strum::{EnumProperty, IntoEnumIterator};

pub trait ChildrenRoutes<T> {
    fn children_routes(&self) -> Option<T>
    where
        Self: IntoEnumIterator
            + std::str::FromStr
            + EnumProperty
            + Copy
            + Clone
            + PartialEq
            + Display;

    // fn children_routes_for_route<C>(&self, route: T) -> Option<C>
    // where
    //     Self: IntoEnumIterator
    //         + std::str::FromStr
    //         + EnumProperty
    //         + Copy
    //         + Clone
    //         + PartialEq
    //         + Display;
}
