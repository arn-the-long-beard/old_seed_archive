use seed::prelude::wasm_bindgen::__rt::std::collections::HashMap;
use seed::Url;

#[derive(Debug)]
pub struct Route {
    pub path: String,
    pub url: Option<Url>,
    pub name: String,
    pub children: Vec<Route>,
    pub guarded: bool,
    pub default: bool,
}

impl Route {
    /// Extracted Hashed ready to use children with String path from the parent Root
    ///
    ///
    pub fn extract_hashed_recursive_children(&self) -> HashMap<String, Route> {
        let mut hash_map = HashMap::new();

        for child_route in self.children.iter() {
            hash_map.insert(
                format!("{}/{}", self.path, child_route.path),
                child_route.clone(),
            );

            for hashed_grand_children in child_route.extract_hashed_recursive_children() {
                hash_map.insert(
                    format!("{}/{}", self.path, hashed_grand_children.0),
                    hashed_grand_children.1.clone(),
                );
            }
        }
        hash_map
    }
}
/// todo I am afraid than cloning can be expansive if large app
impl Clone for Route {
    fn clone(&self) -> Self {
        Route {
            path: self.path.to_string(),
            name: self.name.to_string(),
            url: self.url.clone(),
            children: self.children.clone(),
            guarded: self.guarded,
            default: self.default,
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        if self.url.is_none() && other.url.is_none() {
            self.path == other.path && self.name == other.name && self.children == other.children
        } else {
            self.url.clone().unwrap() == other.url.clone().unwrap()
                && self.path == other.path
                && self.name == other.name
                && self.children == other.children
        }
    }
}
#[cfg(test)]
mod test {
    use crate::router::route::Route;

    #[test]
    fn test_hash_extraction() {
        let random_page = Route {
            path: "random".to_string(),
            url: None,
            name: "random".to_string(),
            children: vec![],
            guarded: false,
            default: false,
        };
        let random_page_2 = Route {
            path: "random_2".to_string(),
            url: None,
            name: "random_2".to_string(),
            children: vec![],
            guarded: false,
            default: false,
        };
        let random_page_3 = Route {
            path: "random_3".to_string(),
            url: None,
            name: "random_3".to_string(),
            children: vec![],
            guarded: false,
            default: false,
        };
        let private = Route {
            path: "private".to_string(),
            url: None,
            name: "private".to_string(),
            children: vec![],
            guarded: false,
            default: false,
        };
        let dashboard = Route {
            path: "dashboard".to_string(),
            url: None,
            name: "dashboard".to_string(),
            children: vec![private.clone(), random_page.clone()],
            guarded: false,
            default: false,
        };

        let not_found = Route {
            path: "not_found".to_string(),
            url: None,
            name: "not_found".to_string(),
            children: vec![],
            guarded: false,
            default: false,
        };
        let admin = Route {
            path: "admin".to_string(),
            url: None,
            name: "admin".to_string(),
            children: vec![dashboard.clone()],
            guarded: true,
            default: false,
        };

        let root = Route {
            path: "".to_string(),
            url: None,
            name: "root".to_string(),
            children: vec![
                admin.clone(),
                random_page_2.clone(),
                random_page_3.clone(),
                not_found.clone(),
            ]
            .clone(),
            guarded: false,
            default: false,
        };

        let hashed_routes = root.extract_hashed_recursive_children();

        let mut string_hashmap = "".to_string();
        for route in &hashed_routes {
            println!("url : {:?} - Route {:?} ", route.0, route.1);
            string_hashmap += (format!("url : {} - Route {:?} ", route.0, route.1)
                .as_str()
                .to_owned()
                + "\n")
                .as_str();
        }
        let len: u8 = hashed_routes.len() as u8;
        eprintln!("{:?}", string_hashmap);

        assert_eq!(hashed_routes["/admin/dashboard/private"], private);
        assert_eq!(hashed_routes["/admin/dashboard/random"], random_page);
        assert_eq!(hashed_routes["/admin/dashboard"], dashboard);
        assert_eq!(hashed_routes["/admin"], admin);
        assert_eq!(hashed_routes["/not_found"], not_found);
        assert_eq!(hashed_routes["/random_2"], random_page_2);
        assert_eq!(hashed_routes["/random_3"], random_page_3);
        assert_eq!(len, 7);
    }
}
