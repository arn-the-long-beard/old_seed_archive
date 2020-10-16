use crate::{build_advanced, build_structs, get_string_from_attribute};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::format_ident;
use quote::{quote, ToTokens};
use syn::{
    export::TokenStream2, parse::Result, parse_macro_input, punctuated::Iter, Attribute, Data,
    DataEnum, DeriveInput, Error, Field, Fields, Ident, Lit, LitStr, Meta, MetaNameValue, Path,
    Variant,
};

pub fn init_snippets(variants: Iter<'_, Variant>) -> Vec<TokenStream2> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let state_scope = variant_state_path_tuple(ident.clone(), attrs.iter());
        match fields {
            Fields::Unit => init_as_unit_variant(ident.clone(), state_scope),
            Fields::Unnamed(fields) => {
                init_as_tuple_variant(ident.clone(), state_scope, fields.unnamed.iter())
            }
            Fields::Named(fields) => {
                init_as_struct_variant(ident.clone(), state_scope, fields.named.iter())
            }
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

fn variant_state_path_tuple(
    ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String)> {
    println!("got attribut for variant => {:?}", ident.to_string());
    let mut attrs = attrs.filter_map(
        |attr| match get_string_from_attribute("state_scope", attr) {
            Ok(op) => op,
            Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
        },
    );
    let state_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if state_scope.is_empty() {
        None
    } else {
        println!("got attribut => {:?}", state_scope);
        let string_to_parse = state_scope;
        let state_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut state_scope_string_iter = state_scope_string.iter();
        let state_path = state_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[state_path = PATH => INIT] but got this {:?}",
                string_to_parse
            )
        });
        let state_init = state_scope_string_iter.next().expect(
            format!(
                "expect init for  #[state_path = PATH => INIT] but got this {:?}",
                string_to_parse
            )
            .as_str(),
        );
        Some((state_path.trim().to_string(), state_init.trim().to_string()))
    }
}

fn init_as_unit_variant(ident: Ident, state_scope: Option<(String, String)>) -> TokenStream2 {
    let format = match state_scope {
        Some((path, init)) => {
            let token: TokenStream2 = format!(
                " previous_state.{} ={}(self.to_url(),
                    &mut previous_state.{},
                        &mut orders.proxy(Msg::{}),)  ",
                path,
                init,
                path,
                ident.to_string()
            )
            .parse()
            .unwrap();
            quote! {
            #token  }
        }
        None => quote! { {} },
    };
    quote! {
        Self::#ident => #format
    }
}
fn init_as_tuple_variant(
    ident: Ident,
    state_scope: Option<(String, String)>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }

    // Do stuff about nested init maybe ?
    let format = match state_scope {
        Some((path, init)) => {
            let token: TokenStream2 = format!(
                " previous_state.{} ={}(self.to_url(),
                    &mut previous_state.{},
                        &mut orders.proxy(Msg::{}),)  ",
                path,
                init,
                path,
                ident.to_string()
            )
            .parse()
            .unwrap();
            quote! {
            #token  }
        }
        None => quote! { {} },
    };
    quote! {
        Self::#ident(nested) => #format
    }
}

fn init_as_struct_variant(
    ident: Ident,
    state_scope: Option<(String, String)>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    let mut fields_to_extract = fields.clone();

    let query_parameters = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query");

    let id_param = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "id");

    let children = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "children");

    let structs_tuple = (id_param, query_parameters, children);

    let structs = build_structs(structs_tuple);

    // do stuff also for children init maybe
    //  let string_enum = build_string(structs_tuple, name.clone());

    let format = match state_scope {
        Some((path, init)) => {
            let token: TokenStream2 = format!(
                " previous_state.{} ={}(self.to_url(),
                    &mut previous_state.{},
                        &mut orders.proxy(Msg::{}),)  ",
                path,
                init,
                path,
                ident.to_string()
            )
            .parse()
            .unwrap();
            quote! {
            #token  }
        }
        None => quote! { {} },
    };
    quote! {
        Self::#ident{#structs} => #format
    }
}
