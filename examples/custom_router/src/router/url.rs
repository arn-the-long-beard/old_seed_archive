use crate::router::super_router::Urls;
use enum_paths::ParsePath;
use seed::Url;
use std::fmt::Debug;
use strum::IntoEnumIterator;

pub trait Routing {
    fn from_url(url: Url) -> Option<Self>
    where
        Self: Sized;
    fn to_url(&self) -> Url;
}

impl<T: Debug + IntoEnumIterator + PartialEq + ParsePath + Copy + Clone> Routing for T {
    fn from_url(url: Url) -> Option<Self>
    where
        Self: Sized,
    {
        let mut path = url.to_string();
        path.remove(0);
        T::parse_path(path.as_str()).ok()
    }

    fn to_url(&self) -> Url {
        let full_path = &self.as_path();
        let segments: Vec<&str> = full_path.as_str().split('/').collect();
        Urls::new(Url::new()).build_url(segments)
    }
}
#[cfg(test)]
mod test {
    use crate::router::{Router, Urls};

    extern crate enum_paths;
    extern crate router_macro_derive;

    use super::*;
    use enum_paths::{AsPath, ParseError, ParsePath};

    use crate::router::super_router::SuperRouter;
    use crate::router::url::Routing;
    use seed::Url;
    use strum::IntoEnumIterator;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Copy, Clone, AsPath)]
    pub enum DashboardAdminRoutes {
        Other,
        #[as_path = ""]
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
        #[as_path = ""]
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
        #[as_path = ""]
        Home,
    }

    #[wasm_bindgen_test]
    fn test_to_urls() {
        let mut router = SuperRouter::<ExampleRoutes>::new();
        //todo we should have attribute in the enum for default route
        router.default_route = Some(ExampleRoutes::NotFound);

        assert_eq!(
            ExampleRoutes::Login.to_url(),
            router.url(&ExampleRoutes::Login)
        );
        assert_eq!(ExampleRoutes::Login.to_url().to_string(), "/login");

        assert_eq!(
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
                .to_url()
                .to_string(),
            "/dashboard/profile/1"
        )
    }

    #[wasm_bindgen_test]
    fn test_from_urls() {
        let mut router = SuperRouter::<ExampleRoutes>::new();
        //todo we should have attribute in the enum for default route
        router.default_route = Some(ExampleRoutes::NotFound);

        assert_eq!(
            ExampleRoutes::Login.to_url(),
            router.url(&ExampleRoutes::Login)
        );

        let url: Url = "/login".parse().unwrap();
        assert_eq!(ExampleRoutes::from_url(url).unwrap(), ExampleRoutes::Login);

        let url: Url = "/dashboard/profile/1".parse().unwrap();
        assert_eq!(
            ExampleRoutes::from_url(url).unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
        );
    }
}
