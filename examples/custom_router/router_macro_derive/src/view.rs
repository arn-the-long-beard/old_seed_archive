use crate::{build_advanced, build_structs, get_string_from_attribute};
use convert_case::Casing;

use proc_macro_error::{abort, Diagnostic, Level};

use crate::guard::variant_guard_path_tuple;
use quote::quote;
use syn::{export::TokenStream2, punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

pub fn view_snippets(variants: Iter<'_, Variant>) -> Vec<TokenStream2> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let view_scope = variant_view_path_tuple(ident.clone(), attrs.iter());
        let local_scope = variant_local_view_tuple(ident.clone(), attrs.iter());
        let guard_scope = variant_guard_path_tuple(ident.clone(), attrs.iter());
        if view_scope.is_some() && local_scope.is_some() {
            abort!(Diagnostic::new(
                Level::Error,
                "You can only use #view_scope or #local_scope.".into()
            ))
        }

        if view_scope.is_none() && local_scope.is_none() {
            abort!(Diagnostic::new(
                Level::Error,
                "You need to define view loaded for your routes with #view_scope or #local_view"
                    .into()
            ))
        }

        match fields {
            Fields::Unit => {
                view_as_unit_variant(ident.clone(), view_scope, local_scope, guard_scope)
            }
            Fields::Unnamed(fields) => view_as_tuple_variant(
                ident.clone(),
                view_scope,
                local_scope,
                guard_scope,
                fields.unnamed.iter(),
            ),
            Fields::Named(fields) => view_as_struct_variant(
                ident.clone(),
                view_scope,
                local_scope,
                guard_scope,
                fields.named.iter(),
            ),
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

fn variant_view_path_tuple(
    ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String)> {
    println!("got attribut for variant => {:?}", ident.to_string());
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("view_scope", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let view_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if view_scope.is_empty() {
        None
    } else {
        println!("got attribut => {:?}", view_scope);
        let string_to_parse = view_scope;
        let view_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut view_scope_string_iter = view_scope_string.iter();
        let view_path = view_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[view_path = PATH => VIEW] but got this {:?}",
                string_to_parse
            )
        });
        let view_init = view_scope_string_iter.next().expect(
            format!(
                "expect view for  #[view_path = PATH => VIEW] but got this {:?}",
                string_to_parse
            )
            .as_str(),
        );
        Some((view_path.trim().to_string(), view_init.trim().to_string()))
    }
}
fn variant_local_view_tuple(
    ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String)> {
    println!("got attribut for variant => {:?}", ident.to_string());
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("local_view", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let view_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if view_scope.is_empty() {
        None
    } else {
        println!("got attribute => {:?}", view_scope);
        let string_to_parse = view_scope;
        let view_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut view_scope_string_iter = view_scope_string.iter();
        let view_path = view_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[view_path = PATH => VIEW] but got this {:?}",
                string_to_parse
            )
        });
        let view_init = view_scope_string_iter.next().expect(
            format!(
                "expect view for  #[view_path = PATH => VIEW] but got this {:?}",
                string_to_parse
            )
            .as_str(),
        );

        eprintln!("Path --> {}  for view {}", view_path, view_init);
        Some((view_path.trim().to_string(), view_init.trim().to_string()))
    }
}
fn view_as_unit_variant(
    ident: Ident,
    view_scope: Option<(String, String)>,
    local_scope: Option<(String, String)>,
    guard_scope: Option<(String, String, String)>,
) -> TokenStream2 {
    let load_view = if let Some((path, view)) = view_scope {
        let token: TokenStream2 = format!(
            " {}(&scoped_state.{}).map_msg(Msg::{})",
            view,
            path,
            ident.to_string()
        )
        .parse()
        .unwrap();
        quote! {
        #token  }
    } else if let Some((path, view)) = local_scope {
        let token: TokenStream2 = if path.is_empty() {
            format!(" {}(&scoped_state)", view).parse().unwrap()
        } else {
            format!(" {}(&scoped_state.{})", view, path,)
                .parse()
                .unwrap()
        };
        quote! {
        #token  }
    } else {
        quote! { {} }
    };
    let with_guard_or_not = match guard_scope {
        None => {
            quote! { #load_view }
        }
        Some((_, _, redirect)) => {
            let redirect: TokenStream2 = format!(" {}(&scoped_state)", redirect).parse().unwrap();
            quote! {
                 if let Some(authenticated) = self.check_before_load(&scoped_state) {
                       if authenticated {
                          #load_view
                        }
                        else {
                          #redirect
                        }
                    } else {
                       #redirect
                    }

            }
        }
    };

    quote! {
        Self::#ident => #load_view
    }
}
fn view_as_tuple_variant(
    ident: Ident,
    view_scope: Option<(String, String)>,
    local_scope: Option<(String, String)>,
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
    let load_view = if let Some((path, view)) = view_scope {
        let token: TokenStream2 = format!(
            " {}(nested, &scoped_state.{}).map_msg(Msg::{})",
            view,
            path,
            ident.to_string()
        )
        .parse()
        .unwrap();
        quote! {
        #token  }
    } else if let Some((path, view)) = local_scope {
        let token: TokenStream2 = if path.is_empty() {
            format!(" {}(nested,&scoped_state)", view).parse().unwrap()
        } else {
            format!(" {}(&scoped_state.{})", view, path,)
                .parse()
                .unwrap()
        };
        quote! {
        #token  }
    } else {
        quote! { {} }
    };

    let with_guard_or_not = match guard_scope {
        None => {
            quote! { #load_view }
        }
        Some((_, _, redirect)) => {
            let redirect: TokenStream2 = format!(" {}(&scoped_state)", redirect).parse().unwrap();
            quote! {
                 if let Some(authenticated) = self.check_before_load(&scoped_state) {
                       if authenticated {
                          #load_view
                        }
                        else {
                          #redirect
                        }
                    } else {
                       #redirect
                    }

            }
        }
    };

    quote! {
        Self::#ident(nested) => #with_guard_or_not
    }
}

fn view_as_struct_variant(
    ident: Ident,
    view_scope: Option<(String, String)>,
    local_scope: Option<(String, String)>,
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

    let load_view = if let Some((path, view)) = view_scope {
        let token: TokenStream2 = if children.is_some() {
            format!(
                " {}(&children,&scoped_state.{}).map_msg(Msg::{})",
                view,
                path,
                ident.to_string()
            )
            .parse()
            .unwrap()
        } else {
            format!(
                " {}(&scoped_state.{}).map_msg(Msg::{})",
                view,
                path,
                ident.to_string()
            )
            .parse()
            .unwrap()
        };
        quote! {
        #token  }
    } else if let Some((path, view)) = local_scope {
        let token: TokenStream2 = if path.is_empty() {
            format!(" {}(&scoped_state)", view).parse().unwrap()
        } else {
            format!(" {}(&scoped_state.{})", view, path,)
                .parse()
                .unwrap()
        };
        quote! {
        #token  }
    } else {
        quote! { {} }
    };

    let with_guard_or_not = match guard_scope {
        None => {
            quote! { #load_view }
        }
        Some((_, _, redirect)) => {
            let redirect: TokenStream2 = format!(" {}(&scoped_state)", redirect).parse().unwrap();
            quote! {
                 if let Some(authenticated) = self.check_before_load(&scoped_state) {
                       if authenticated {
                          #load_view
                        }
                        else {
                          #redirect
                        }
                    } else {
                       #redirect
                    }

            }
        }
    };
    quote! {
        Self::#ident{#structs} => #with_guard_or_not
    }
}
