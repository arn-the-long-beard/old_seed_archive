use crate::{build_string_payload, build_structs, get_string_from_attribute};

use proc_macro_error::{abort, Diagnostic, Level};

use quote::quote;
use syn::{export::TokenStream2, punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

pub fn init_snippets(variants: Iter<'_, Variant>) -> Vec<TokenStream2> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let model_scope = variant_model_path_tuple(ident.clone(), attrs.iter());
        match fields {
            Fields::Unit => model_as_unit_variant(ident.clone(), model_scope),
            Fields::Unnamed(fields) => {
                model_as_tuple_variant(ident.clone(), model_scope, fields.unnamed.iter())
            }
            Fields::Named(fields) => {
                model_as_struct_variant(ident.clone(), model_scope, fields.named.iter())
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

fn variant_model_path_tuple(
    ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String)> {
    println!("got attribut for variant => {:?}", ident.to_string());
    let mut attrs = attrs.filter_map(
        |attr| match get_string_from_attribute("model_scope", attr) {
            Ok(op) => op,
            Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
        },
    );
    let model_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if model_scope.is_empty() {
        None
    } else {
        println!("got attribut => {:?}", model_scope);
        let string_to_parse = model_scope;
        let model_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut model_scope_string_iter = model_scope_string.iter();
        let model_path = model_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[model_path = PATH => INIT] but got this {:?}",
                string_to_parse
            )
        });
        let model_init = model_scope_string_iter.next().expect(
            format!(
                "expect init for  #[model_path = PATH => INIT] but got this {:?}",
                string_to_parse
            )
            .as_str(),
        );
        Some((model_path.trim().to_string(), model_init.trim().to_string()))
    }
}

fn model_as_unit_variant(ident: Ident, model_scope: Option<(String, String)>) -> TokenStream2 {
    let format = match model_scope {
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
fn model_as_tuple_variant(
    ident: Ident,
    model_scope: Option<(String, String)>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }

    // Do stuff about nested init maybe ?
    let format = match model_scope {
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

fn model_as_struct_variant(
    ident: Ident,
    model_scope: Option<(String, String)>,
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
    let payload: String = build_string_payload(structs_tuple);
    let format = match model_scope {
        Some((path, init)) => {
            let token: TokenStream2 = if payload.is_empty() {
                format!(
                    " previous_state.{} ={}(self.to_url(),
                    &mut previous_state.{},
                        &mut orders.proxy(Msg::{}),)  ",
                    path,
                    init,
                    path,
                    ident.to_string()
                )
                .parse()
                .unwrap()
            } else {
                format!(
                    " previous_state.{} ={}(self.to_url(),
                    &mut previous_state.{},
                    {},
                        &mut orders.proxy(Msg::{}),)  ",
                    path,
                    init,
                    path,
                    payload,
                    ident.to_string()
                )
                .parse()
                .unwrap()
            };
            quote! {
            #token  }
        }
        None => quote! { {} },
    };
    quote! {
        Self::#ident{#structs} => #format
    }
}
