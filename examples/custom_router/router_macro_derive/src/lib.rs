#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate convert_case;
extern crate proc_macro;
extern crate proc_macro_error;
use heck::SnakeCase;

use crate::root::get_default_route;
use crate::routing::routing_variant_snippets;
use proc_macro::TokenStream;

use crate::state::init_snippets;
use crate::view::view_snippets;
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::quote;
use syn::{
    export::TokenStream2, parse::Result, parse_macro_input, punctuated::Iter, Attribute, Data,
    DataEnum, DeriveInput, Error, Field, Fields, Ident, Lit, LitStr, Meta, MetaNameValue, Variant,
};

mod root;
mod routing;
mod state;
mod view;

#[proc_macro_derive(Routes)]
pub fn routes(input: TokenStream) -> TokenStream {
    // Parse the Input
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Error out if we're not annotating an enum

    let data: DataEnum = match ast.data {
        Data::Enum(d) => d,
        _ => panic!("Can generate Routes only for enum"),
    };

    let name = &ast.ident;
    let variants = data.variants.iter();

    let extracted_routes = extract_routes(variants, name);

    let extract_route = quote! {
         impl ExtractRoutes for #name {
        fn get_routes() -> Vec<Route> {
            let mut vec : Vec<Route> = Vec::new();
            let future_routes : Vec<Route>  = vec![#(#extracted_routes)*];
            for r in future_routes {
                vec.push(r);
            }
            vec
        }
        fn get_hashed_routes() -> HashMap<String, Route> {
            let mut hash_map: HashMap<String, Route> = HashMap::new();
            let future_routes : Vec<Route>  = vec![#(#extracted_routes)*];
                for r in  future_routes {
                    hash_map.insert(r.path.to_string(), r.clone());

                    for sub_hashed_route in r.extract_hashed_recursive_children() {
                       hash_map.insert(format!("{}", sub_hashed_route.0 ), sub_hashed_route.1.clone());
                    }

                }
            hash_map
        }
    }

        };

    extract_route.into()
}

fn extract_routes(variants: Iter<Variant>, name: &Ident) -> Vec<TokenStream2> {
    let mut extracted_routes = Vec::new();
    for v in variants {
        let var_id = &v.ident;
        let route_name = var_id.to_string().to_snake_case();
        match &v.fields {
            Fields::Named(children) => {
                let children_type = children.named.first().cloned().unwrap().ty.clone();
                let path = quote! {#name::#var_id{ children : Default::default()}};
                let tokens = quote! {
                        Route {
                        path: #path.to_string(),
                        name: #route_name.to_string(),
                        url :None,
                        children: #children_type::get_routes(),
                        guarded: false,
                        default: false,
                    },
                };
                extracted_routes.push(tokens);
            }
            _ => {
                let path = quote! {#name::#var_id};
                let tokens = quote! {
                        Route {
                        path: #path.to_string(),
                        name: #route_name.to_string(),
                        url :None,
                        children: Vec::new(),
                        guarded: false,
                        default: #name::#var_id.get_str("Default").is_some(),
                    },
                };
                extracted_routes.push(tokens);
            } /* Implement traits for the new struct and stuff */
        }
    }
    extracted_routes
}

// #[proc_macro_derive(Default)]
// pub fn create_router(input: TokenStream) -> TokenStream {
//     // Parse the Input
//     let ast: DeriveInput = syn::from_str(input).unwrap();
//
//     // Error out if we're not annotating an enum
//
//     let data: DataEnum = match ast.data {
//         Data::Enum(d) => d,
//         _ => panic!("Can generate Routes only for enum"),
//     };
//
//     let name = &ast.ident;
//     let variants = data.variants.iter();
//
//     let mut extracted_routes = extract_routes(variants, name);
//
//     let extract_route = quote! {
//
//     #[topo::nested]
//     pub fn get_router() {
//         let id = topo::CallId::current();
//         id
//     }
//         };
//
//     extract_route.into()
// }
// /// Specify if a route is default and used as 404
// #[proc_macro_derive(DefaultRoute,)]
// pub fn derive_default_route_attr(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

/// Derive an enum as Routing for navigation
/// You can change the value of a path for a given route this way
///
/// ```rust
///
///     #[derive(Debug, PartialEq, Copy, Clone, Routing)]
///     pub enum DashboardAdminRoutes {
///         #[as_path = "my_stuff"]  // "/my_stuff"
///         Other,
///         #[as_path = ""]
///         Root,  // "/"
///     }
/// ```
///
#[proc_macro_error]
#[proc_macro_derive(Routing, attributes(as_path))]
pub fn derive_as_path(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };
    let variants = variants.iter();
    let (as_snippets, parse_snippets) = routing_variant_snippets(variants.clone());
    let name = ident.to_string();
    TokenStream::from(quote! {
     impl Navigation for #ident {
        fn to_url(&self) -> Url {
         let url : Url =    match self {
                    #(#as_snippets),*
                    }.parse().unwrap();
                    url
        }

        fn from_url(url: Url) -> std::result::Result<Self, ParseError>
         where
        Self: Sized + ParsePath {
        let string_url = url.to_string();
          Self::parse_path(&string_url)
        }
    }
            impl AsPath for #ident {
            fn as_path(self) -> String {
                match self {
                    #(#as_snippets),*
                }
            }
        }
        impl ParsePath for #ident {
            fn parse_path(path: &str) -> std::result::Result<Self, ParseError> {
                let next = path.trim_start_matches("/");
                Err(ParseError::NoMatch)
                    #(.or_else(|err|
                        #parse_snippets
                        )
                    )*
                    .map_err(|err| ParseError::By(#name.to_string(), Box::new(err)))
            }
        }
    })
}
fn get_string_from_attribute(attribute_name: &str, attr: &Attribute) -> Result<Option<LitStr>> {
    if !attr.path.is_ident(attribute_name) {
        return Ok(None); // not our attribute
    }
    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(name),
            ..
        }) => Some(Some(name)),
        _ => None,
    }
    .ok_or_else(|| Error::new_spanned(attr, &format!("expected #[{} = \"...\"]", attribute_name)))
}

/// Rebuild the content of a variant depending of the fields present in the original enum
fn build_structs(structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>)) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            quote! { id,query,children}
        }

        (id, query, _) if id.is_some() && query.is_some() => {
            quote! { id, query}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            quote! { query , children}
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            quote! { id, children }
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { id }
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            quote! { query}
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            quote! { children }
        }

        (id, query, children) if query.is_none() && id.is_none() & children.is_none() => {
            quote! {}
        }
        (_, _, _) => {
            quote! {}
        }
    }
}

/// Assign only the payload defined by the field in the enu,
fn build_advanced(structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>)) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let sub_enum = &children.clone().unwrap().ty;
            quote! { id : id.unwrap(),query : query.unwrap(),children :  #sub_enum::parse_path(&children.unwrap()).unwrap()}
        }

        (id, query, _) if id.is_some() && query.is_some() => {
            quote! { id : id.unwrap(),query : query.unwrap()}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let sub_enum = &children.clone().unwrap().ty;
            quote! { query : query.unwrap(),children :  #sub_enum::parse_path(&children.unwrap()).unwrap()}
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            let sub_enum = &children.clone().unwrap().ty;
            quote! { id : id.unwrap(),children : #sub_enum::parse_path(&children.unwrap()).unwrap()}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { id : id.unwrap()}
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            quote! { query : query.unwrap()}
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            let sub_enum = &children.clone().unwrap().ty;
            quote! { children :#sub_enum::parse_path(&children.unwrap().clone()).unwrap()}
        }

        (_, _, _) => {
            quote! {}
        }
    }
}

/// Define a routing config as root for your navigation.
/// It will contain the default route used by the router when it cannot find the right url
/// ```rust
///
///     #[derive(Debug, PartialEq, Copy, Clone, Root)]
///     pub enum DashboardAdminRoutes {
///         #[default_route]
///         NotFound,  // -> /blablablalbla -> /not_found
///         Root,  
///     }
/// ```
///
#[proc_macro_error]
#[proc_macro_derive(Root, attributes(default_route))]
pub fn define_as_root(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };
    let variants = variants.iter();
    let default_route = get_default_route(variants.clone());

    if default_route.is_err() {
        abort!(Diagnostic::new(
            Level::Error,
            "Could not find default_route".into()
        ))
    }

    let default_variant = default_route.unwrap();

    match default_variant.fields {
        Fields::Named(_) => abort!(Diagnostic::new(
            Level::Error,
            "Default route need to be simple".into()
        )),
        Fields::Unnamed(_) => abort!(Diagnostic::new(
            Level::Error,
            "Default route need to be simple".into()
        )),
        Fields::Unit => {}
    }

    let default_variant_ident = default_variant.ident;

    TokenStream::from(quote! {

      impl Default for #ident {
            fn default() -> #ident {
                #ident::#default_variant_ident
            }
        }
    })
}

/// Give the ability to init states based on the routing
///
/// ```rust
/// #[derive(Debug, PartialEq, Clone, Routing, Root, InitState)]
///     
///     pub enum ExampleRoutes {
///         #[state_scope = "stuff => profile::init"]
///         Other {
///             id: String,
///             children: Settings,
///         },
///
///         Admin {
///             query: IndexMap<String, String>,
///         },
///         Dashboard(DashboardRoutes),
///         #[state_scope = "profile => profile::init"]
///         Profile {
///             id: String,
///         },
///         #[default_route]
///         NotFound,
///         #[as_path = ""]
///         Root,
///     }
/// ```
///
#[proc_macro_error]
#[proc_macro_derive(InitState, attributes(state_scope))]
pub fn derive_add_model_init(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };
    let variants = variants.iter();
    let init_snippets = init_snippets(variants.clone());
    TokenStream::from(quote! {
         impl StateInit<#ident, Model, Msg> for #ident {
        fn init<'b, 'c>(
            &self,
            previous_state: &'b mut Model,
            orders: &'c mut impl Orders<Msg>,
        ) -> &'b mut Model {
            match self {
                #(#init_snippets),*
            }
            previous_state
        }
    }
        })
}

/// Give the ability to init states based on the routing
///
/// ```rust
/// #[derive(Debug, PartialEq, Clone, Routing, Root, OnInit,ToView)]
///    
///     pub enum ExampleRoutes {
///         #[state_scope = "stuff => profile::init"]
///         #[view_scope = "stuff => other::view"]
///         Other {
///             id: String,
///             children: Settings,
///         },
///         #[view_scope = "admin => admin::view"]
///         Admin {
///             query: IndexMap<String, String>,
///         },
///         #[view_scope = "dashboard => profile::view"]
///         Dashboard(DashboardRoutes),
///         #[state_scope = "profile => profile::init"]
///         #[view_guard = "logged_user => guard::user"]
///         #[view_scope = "profile => profile::view"]
///         Profile {
///             id: String,
///         },
///         #[view_scope = "not_found::view"]
///         #[default_route]
///         NotFound,
///         #[view_scope = "home => profile::init"]
///         #[as_path = ""]
///         Root,
///     }
/// ```
///
#[proc_macro_error]
#[proc_macro_derive(View, attributes(view_guard, view_scope, local_view))]
pub fn derive_add_model_view(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };
    let variants = variants.iter();
    let view_snippets = view_snippets(variants.clone());
    TokenStream::from(quote! {
    impl ToView<#ident, Model, Msg> for  #ident {
        fn view(&self, scoped_state: &Model) -> Node<Msg> {
            match self {
                 #(#view_snippets),*
            }
        }
    }

    })
}
