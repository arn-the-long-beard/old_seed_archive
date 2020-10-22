use crate::{build_advanced, build_string_payload, build_structs, get_string_from_attribute};
use convert_case::{Case, Casing};
use proc_macro_error::{abort, Diagnostic, Level};

use quote::quote;
use syn::{export::TokenStream2, punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

pub fn query_variant_snippets(variants: Iter<'_, Variant>) -> Vec<TokenStream2> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        match fields {
            Fields::Unit => query_as_unit_variant(ident.clone()),
            Fields::Unnamed(fields) => query_as_tuple_variant(ident.clone(), fields.unnamed.iter()),
            Fields::Named(fields) => query_as_struct_variant(ident.clone(), fields.named.iter()),
            _ => abort!(Diagnostic::new(
                Level::Error,
                "Only unit or single tuple variants allowed.".into()
            )),
        }
    });
    snippets.fold(Vec::with_capacity(len), |mut acc, x| {
        acc.push(x);
        acc
    })
}

fn query_as_unit_variant(ident: Ident) -> TokenStream2 {
    quote! {
        Self::#ident =>  { None }
    }
}
fn query_as_tuple_variant(ident: Ident, fields: Iter<'_, Field>) -> TokenStream2 {
    let field = fields
        .into_iter()
        .next()
        .expect("Should get a field in single tuple");

    let sub_type = &field.ty;

    let check_t32 = quote! { #sub_type};
    // todo need to make real stuff there with more checks
    let token = if check_t32.to_string().eq("u32") || check_t32.to_string().eq("&u32") {
        quote! { None }
    } else {
        quote! {  nested.get_query_parameters() }
    };
    // Do stuff about nested init maybe ?
    quote! {
        Self::#ident(nested) => {#token}
    }
}

fn query_as_struct_variant(ident: Ident, fields: Iter<'_, Field>) -> TokenStream2 {
    let mut fields_to_extract = fields.clone();

    let query_parameters = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query");

    let query_param = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "id");

    let children = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "children");

    let structs_tuple = (query_param, query_parameters, children);

    let structs = build_structs(structs_tuple);

    // do stuff also for children init maybe
    //  let string_enum = build_string(structs_tuple, name.clone());
    // let payload: String = build_string_payload(structs_tuple);

    let format = build_query_return(structs_tuple);
    quote! {
        Self::#ident{#structs} => #format
    }
}

fn build_query_check() -> TokenStream2 {
    quote! {
    if query.len() > 0 {
        Some(query)
    } else {
    None}
    }
}
fn build_query_return(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let query_return = build_query_check();
            quote! { #query_return }
        }

        (id, query, _) if id.is_some() && query.is_some() => {
            let query_return = build_query_check();
            quote! { #query_return }
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            quote! { None}
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            quote! { None}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { None}
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            let query_return = build_query_check();
            quote! { #query_return }
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            quote! { None}
        }

        (id, query, children) if query.is_none() && id.is_none() & children.is_none() => {
            quote! {}
        }
        (_, _, _) => {
            quote! {}
        }
    }
}
