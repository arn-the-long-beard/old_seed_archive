#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[macro_use]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::{quote, ToTokens};
use syn::{Data, DataEnum, DeriveInput, Fields, Type};

#[proc_macro_derive(MyProcMacro)]
pub fn derive_my_proc_macro(input: TokenStream) -> TokenStream {
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
//
// #[proc_macro_derive(HelloMacro)]
// pub fn router_macro_derive(input: TokenStream) -> TokenStream {
//     // Construct a representation of Rust code as a syntax tree
//     // that we can manipulate
//     let ast = syn::parse(input).unwrap();
//
//     // Build the trait implementation
//     impl_children_route(&ast)
// }
// fn impl_children_route(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let gen = quote! {
//         impl ChildrenRoute for #name {
//             fn children_routes(&self) {
//                  if let Some(childrem) = self.get_str("Children") {
//
//             }
//             }
//         }
//     };
//     gen.into()
// }
