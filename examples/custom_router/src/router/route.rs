#[derive(Debug)]
pub struct Route {
    pub path: String,
    pub parent_route_path: String,
    pub guarded: bool,
    pub default: bool,
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route {
            path: self.path.to_string(),
            parent_route_path: "".to_string(),
            guarded: false,
            default: false,
        }
    }
}
