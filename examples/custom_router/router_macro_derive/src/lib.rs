#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[macro_use]
extern crate proc_macro;
use heck::SnakeCase;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::{quote, ToTokens};
use syn::export::quote::__private::Ident;
use syn::export::TokenStream2;
use syn::punctuated::Iter;
use syn::{Data, DataEnum, DeriveInput, Fields, Type, Variant};

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

    let mut extracted_routes = extract_routes(variants, name);

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
