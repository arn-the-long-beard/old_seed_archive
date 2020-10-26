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

use crate::root::get_default_route;
use crate::routing::routing_variant_snippets;
use proc_macro::TokenStream;

use crate::routing_modules::{module_init_snippets, modules_path, modules_snippets};

use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::quote;
use syn::{
    export::TokenStream2, parse::Result, parse_macro_input, Attribute, Data, DataEnum, DeriveInput,
    Error, Field, Fields, Ident, Lit, LitStr, Meta, MetaNameValue, Variant,
};

mod guard;

mod root;
mod routing;
mod routing_modules;

/// Derive an enum as Routing for navigation
/// You can change the value of a path for a given route this way
///
/// ```rust
///
///     #[derive(Debug, PartialEq, Copy, Clone, Url)]
///     pub enum DashboardAdminRoutes {
///         #[as_path = "my_stuff"]  // "/my_stuff"
///         Other,
///         #[as_path = ""]
///         Root,  // "/"
///     }
/// ```
///
#[proc_macro_error]
#[proc_macro_derive(Url, attributes(as_path))]
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
     impl router::Navigation for #ident {
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
        impl router::ParsePath for #ident {
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
fn build_string_payload(structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>)) -> String {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            "id,query,children".to_string()
        }

        (id, query, _) if id.is_some() && query.is_some() => "id,query".to_string(),
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            "query,children".to_string()
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            "id,children".to_string()
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            "id".to_string()
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            "query".to_string()
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            "children".to_string()
        }

        (id, query, children) if query.is_none() && id.is_none() & children.is_none() => {
            "".to_string()
        }
        (_, _, _) => "".to_string(),
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

// The RoutingModule makes the enum variants representing modules loaded by the routes
///
///  You can rename the path
///  You can specify routes that do not load module ( no init, no specific Model & Msg and no view )
///
///  When loading the module, a parser will get the init function , Model, Msg, Routes, Update, and View
///
/// ```rust
///
///
///
/// #[derive(Debug, PartialEq, Clone, RoutingModules)]
///     pub enum ExampleRoutes {
///        // #[as_module= "my_stuff"] // the module is name my_stuff.rs
///         Other {
///             id: String,
///             children: Settings,
///         },
///         #[guard = "logged_user => admin_guard => not_authorized_view"]
///         Admin { // will load module "admin.rs"
///          // will load model.admin and as well
///          // equal to  
///          // #[model_scope = "admin => admin ::init"] will check init has correct arguments
///          // #[view_scope = "admin => admin::view"]  will check viewt has correct arguments
///             query: IndexMap<String, String>,
///         },
///         #[guard = "logged_user => user_guard => not_logged_user_view"]
///         Dashboard(DashboardRoutes), // will load module "dashboard"
///         Profile { // will load module "profile"
///             id: String,
///         },
///         #[guard = "logged_user => admin_guard => not_authorized_view"]
///         #[view = " => my_stuff"]
///         MyStuff,
///         #[view = " => not_found"]
///         #[default_route]
///         NotFound,
///         #[view = " => home"]
///         #[as_path = ""]
///         Root,
///     }
///
/// fn view(model: &Model) -> impl IntoNodes<Msg> {
///     vec![
///         header(&model),
///         if let Some(route) = &model.router.current_route {
///             route.view(model)
///         } else {
///             home(&model.theme)
///         },
///     ]
/// }
///
/// ```
///
///
#[proc_macro_error]
#[proc_macro_derive(
    RoutingModules,
    attributes(as_path, view, guard, default_route, modules_path)
)]
pub fn derive_add_module_load(item: TokenStream) -> TokenStream {
    let add_url = derive_as_path(item.clone());
    let root = define_as_root(item.clone());
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };

    let url_impl = TokenStream2::from(add_url);
    let default_route_impl = TokenStream2::from(root);
    let variants = variants.iter();

    let modules_path = modules_path(ident.clone(), attrs.iter());

    let modules_snippets = modules_snippets(variants.clone(), modules_path.clone());

    let init_snippets = module_init_snippets(variants.clone(), modules_path.clone());
    TokenStream::from(quote! {
    #url_impl

    #default_route_impl

    impl router::View<#ident, Model, Msg> for  #ident {
        fn view(&self, scoped_state: &Model) -> Node<Msg> {
            match self {
                 #(#modules_snippets),*
            }
        }
    }

         impl router::Init<#ident, Model, Msg> for #ident {
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
