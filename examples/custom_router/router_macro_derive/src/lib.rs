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

#[proc_macro_derive(ToStruct)]
pub fn derive_to_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // Error out if we're not annotating an enum
    let data: DataEnum = match ast.data {
        Data::Enum(d) => d,
        _ => panic!("My structs can only be derived for enums"),
    };
    let variants = data.variants.iter();
    let variant_structs = variants.map(|v| {
        let var_id = &v.ident;

        let strut_name = format_ident!("{}Route", var_id);

        let fields = v.fields.clone().into_token_stream();

        match &v.fields {
            Fields::Named(children) => {
                let children_type = children.named.first().cloned().unwrap().ty.clone();
                quote! {
                    pub struct #strut_name #fields
                    impl #strut_name {
                            pub fn children(&self) -> #children_type {
                                 self.children
                            }
                    }
                }
            }
            _ => {
                quote! {
                    pub struct #strut_name #fields;
                }
            } /* Implement traits for the new struct and stuff */
        }
    });
    let gen = quote! {
        #(#variant_structs)*
    };

    println!("{:?}", gen);
    gen.into()
}
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
                 fn get_routes() -> HashMap<String,Route> {
                 let mut hash_map: HashMap<String, Route> = HashMap::new();
                 let future_routes  = vec![#(#extracted_routes)*];
                 for r in future_routes {
                       hash_map.insert(r.path.to_string(), r );
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

        match &v.fields {
            Fields::Named(children) => {
                let children_type = children.named.first().cloned().unwrap().ty.clone();
                println!("SUUUUUUUUUUUUUUUUUUUUUUUB_Routes");
                let path = quote! {#name::#var_id{ children : Default::default()}};
                let tokens = quote! {
                        Route {
                        path: #path.to_string(),
                        children: #children_type::get_routes(),
                        parent_route_path: "".to_string(),
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
                         children: HashMap::new(),
                        parent_route_path: "".to_string(),
                        guarded: false,
                        default: false,
                    },
                };
                extracted_routes.push(tokens);
            } /* Implement traits for the new struct and stuff */
        }
    }
    extracted_routes
}
