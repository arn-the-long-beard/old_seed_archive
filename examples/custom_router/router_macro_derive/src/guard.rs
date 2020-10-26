use crate::{build_advanced, build_structs, get_string_from_attribute};
use convert_case::Casing;

use proc_macro_error::{abort, Diagnostic, Level};

use quote::quote;
use syn::{export::TokenStream2, punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

pub fn guard_snippets(variants: Iter<'_, Variant>) -> Vec<TokenStream2> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let guard_scope = variant_guard_path_tuple(ident.clone(), attrs.iter());

        match fields {
            Fields::Unit => guard_as_unit_variant(ident.clone(), guard_scope),
            Fields::Unnamed(fields) => {
                guard_as_tuple_variant(ident.clone(), guard_scope, fields.unnamed.iter())
            }
            Fields::Named(fields) => {
                guard_as_struct_variant(ident.clone(), guard_scope, fields.named.iter())
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

pub fn variant_guard_path_tuple(
    ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String, String)> {
    println!("got attribut for variant => {:?}", ident.to_string());
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("guard", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let guard_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if guard_scope.is_empty() {
        None
    } else {
        println!("got attribut => {:?}", guard_scope);
        let string_to_parse = guard_scope;
        let guard_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut guard_scope_string_iter = guard_scope_string.iter();
        let guard_path = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got this {:?}",
                string_to_parse
            )
        });
        let guard_init = guard_scope_string_iter.next().expect(
            format!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got this {:?}",
                string_to_parse
            )
            .as_str(),
        );
        let guard_redirect = guard_scope_string_iter.next().expect(
            format!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got this {:?}",
                string_to_parse
            )
            .as_str(),
        );

        Some((
            guard_path.trim().to_string(),
            guard_init.trim().to_string(),
            guard_redirect.trim().to_string(),
        ))
    }
}

fn guard_as_unit_variant(
    ident: Ident,
    guard_scope: Option<(String, String, String)>,
) -> TokenStream2 {
    let format = match guard_scope {
        None => {
            quote! { {None} }
        }
        Some((path, guard, redirect)) => {
            let token: TokenStream2 = if path.is_empty() {
                format!(" {}(&scoped_state)", guard).parse().unwrap()
            } else {
                format!(" {}(&scoped_state.{})", guard, path,)
                    .parse()
                    .unwrap()
            };
            quote! {
            #token  }
        }
    };

    quote! {
        Self::#ident => #format
    }
}
fn guard_as_tuple_variant(
    ident: Ident,
    guard_scope: Option<(String, String, String)>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }
    // Do stuff about nested init maybe ?
    let format = match guard_scope {
        None => {
            quote! { {None} }
        }
        Some((path, guard, redirect)) => {
            let token: TokenStream2 = if path.is_empty() {
                format!(" {}(&scoped_state)", guard).parse().unwrap()
            } else {
                format!(" {}(&scoped_state.{})", guard, path,)
                    .parse()
                    .unwrap()
            };
            quote! {
            #token  }
        }
    };
    quote! {
        Self::#ident(nested) => #format
    }
}

fn guard_as_struct_variant(
    ident: Ident,
    guard_scope: Option<(String, String, String)>,
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

    let format = match guard_scope {
        None => {
            quote! { {None} }
        }
        Some((path, guard, redirect)) => {
            let token: TokenStream2 = if path.is_empty() {
                format!(" {}(&scoped_state)", guard).parse().unwrap()
            } else {
                format!(" {}(&scoped_state.{})", guard, path,)
                    .parse()
                    .unwrap()
            };
            quote! {
            #token  }
        }
    };
    quote! {
        Self::#ident{#structs} => #format
    }
}
