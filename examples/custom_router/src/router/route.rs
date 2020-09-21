use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;

#[derive(Debug)]
pub struct Route {
    pub path: String,
    pub parent_route_path: String,
    pub children: HashMap<String, Route>,
    pub guarded: bool,
    pub default: bool,
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route {
            path: self.path.to_string(),
            parent_route_path: "".to_string(),
            children: self.children.clone(),
            guarded: false,
            default: false,
        }
    }
}
