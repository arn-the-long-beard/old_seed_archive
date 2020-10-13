#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[macro_use]
extern crate proc_macro_error;
extern crate convert_case;
extern crate proc_macro;
use heck::SnakeCase;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::format_ident;
use quote::{quote, ToTokens};
use syn::{
    export::TokenStream2, parse::Result, parse_macro_input, punctuated::Iter, Attribute, Data,
    DataEnum, DeriveInput, Error, Field, Fields, Ident, Lit, LitStr, Meta, MetaNameValue, Variant,
};

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
#[proc_macro_error]
#[proc_macro_derive(Routing, attributes(as_path, default_route))]
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
    let (as_snippets, parse_snippets) = variant_snippets(variants);
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
fn variant_path_segment(ident: Ident, attrs: std::slice::Iter<'_, Attribute>) -> Option<String> {
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("as_path", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let name = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple path names defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        ident.to_string().to_case(Case::Snake)
    };
    if name.to_string().is_empty() {
        None
    } else {
        Some(name)
    }
}
fn variant_snippets(variants: Iter<'_, Variant>) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let name = variant_path_segment(ident.clone(), attrs.iter());
        match fields {
            Fields::Unit => {
                if let None = name {
                    if (i + 1) != len {
                        abort!(Diagnostic::new(
                            Level::Error,
                            "Unit variant without a name must be declared last.".into()
                        ))
                    }
                }
                unit_variant_snippets(ident.clone(), name)
            }
            Fields::Unnamed(fields) => {
                tuple_variant_snippets(ident.clone(), name, fields.unnamed.iter())
            }
            Fields::Named(fields) => {
                struct_variant_snippets(ident.clone(), name, fields.named.iter())
            }
            _ => abort!(Diagnostic::new(
                Level::Error,
                "Only unit or single tuple variants allowed.".into()
            )),
        }
    });
    snippets.fold(
        (Vec::with_capacity(len), Vec::with_capacity(len)),
        |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        },
    )
}
fn unit_variant_snippets(ident: Ident, name: Option<String>) -> (TokenStream2, TokenStream2) {
    (
        as_unit_variant(ident.clone(), name.clone()),
        parse_unit_variant(ident, name),
    )
}
fn as_unit_variant(ident: Ident, name: Option<String>) -> TokenStream2 {
    let format = match name {
        Some(name) => quote! { format!("/{}", #name) },
        None => quote! { String::new() },
    };
    quote! {
        Self::#ident => #format
    }
}

fn parse_unit_variant(ident: Ident, name: Option<String>) -> TokenStream2 {
    let parser = match name {
        Some(name) => quote! {
            next.strip_prefix(#name).ok_or(err)
        },
        None => quote! {
            if next.is_empty() {
                Some(())
            } else {
                None
            }
            .ok_or(ParseError::RemainingSegments)
        },
    };
    quote! {
        #parser.map(|_| Self::#ident)
    }
}
fn tuple_variant_snippets(
    ident: Ident,
    name: Option<String>,
    fields: Iter<'_, Field>,
) -> (TokenStream2, TokenStream2) {
    (
        as_tuple_variant(ident.clone(), name.clone(), fields.clone()),
        parse_tuple_variant(ident, name, fields),
    )
}

fn struct_variant_snippets(
    ident: Ident,
    name: Option<String>,
    fields: Iter<'_, Field>,
) -> (TokenStream2, TokenStream2) {
    (
        as_struct_variant(ident.clone(), name.clone(), fields.clone()),
        parse_struct_variant(ident, name, fields),
    )
}
fn as_tuple_variant(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }
    let format = match name {
        Some(name) => quote! { format!("/{}{}", #name, nested.as_path()) },
        None => quote! { nested.as_path() },
    };
    quote! {
        Self::#ident(nested) => #format
    }
}

fn as_struct_variant(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> TokenStream2 {
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

    let string_enum = build_string(structs_tuple, name.clone());

    let format = match &name {
        Some(_) => quote! { #string_enum },
        None => quote! {},
    };
    quote! {
        Self::#ident{#structs} => #format
    }
}

fn build_query() -> TokenStream2 {
    quote! {convert_to_string(query.clone())}
}
fn build_string(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
    name: Option<String>,
) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let query_string = build_query();
            quote! { format!("/{}/{}{}?{}", #name, id, children.as_path() , #query_string)}
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            let query_string = build_query();

            quote! { format!("/{}/{}?{}", #name, id, #query_string)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let query_string = build_query();

            quote! { format!("/{}/{}={}", #name,  children.as_path(),#query_string)}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_some() => {
            quote! { format!("/{}/{}{}", #name, id,  children.as_path())}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { format!("/{}/{}", #name, id)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_none() => {
            let query_string = build_query();
            quote! { format!("/{}?{}", #name,#query_string)}
        }
        (id, query, children) if id.is_none() && query.is_none() && children.is_some() => {
            quote! { format!("/{}/{}", #name,   children.as_path())}
        }

        (_, _, _) => {
            quote! { format!("/{}", #name)}
        }
    }
}

fn parse_tuple_variant(
    ident: Ident,
    name: Option<String>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }

    let parser = match name {
        Some(name) => quote! {
            next.strip_prefix(#name).ok_or(err)
                .and_then(|rest|
                    ParsePath::parse_path(rest)
                )
        },
        None => quote! {
            ParsePath::parse_path(next)
        },
    };
    quote! {
        #parser.map( Self::#ident )
    }
}
fn parse_struct_variant(
    ident: Ident,
    name: Option<String>,
    mut fields: Iter<'_, Field>,
) -> TokenStream2 {
    // let children = fields.find(|f| f.ident.as_ref().unwrap() == "children");
    let id_param = fields.clone().find(|f| f.ident.as_ref().unwrap() == "id");
    let query_parameters = fields
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query");

    // update when having children available.
    let children = None;

    let structs_tuple = (id_param, query_parameters, children);
    let structs = build_advanced(structs_tuple);
    println!(" macro for  {}", name.clone().unwrap());
    println!(" tuple  {:?}", structs_tuple);
    println!(" id_param  {:?}", id_param);
    let parser = match name {
        Some(name) => {
            quote! {      next.strip_prefix(#name).ok_or(err)
                     .map(|rest| extract_url_payload(rest.to_string()))
            }
        }
        None => quote! {
            ParsePath::parse_path(next)
        },
    };

    quote! {
        #parser.map(|(id, query)| Self::#ident{#structs})
    }
}

fn extract_id(mut fields: Iter<'_, Field>) -> TokenStream2 {
    let id_param = fields.find(|f| f.ident.as_ref().unwrap() == "id");

    if id_param.is_some() {
        quote! {    .and_then(|id| id.parse().ok()) }
    } else {
        quote! {}
    }
}

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
fn build_advanced(structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>)) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            quote! { id : id.unwrap(),query : query.unwrap(),children : children.unwrap()}
        }

        (id, query, _) if id.is_some() && query.is_some() => {
            quote! { id : id.unwrap(),query : query.unwrap()}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            quote! { query : query.unwrap(),children : children.unwrap()}
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            quote! { id : id.unwrap(),children : children.unwrap()}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { id : id.unwrap()}
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            quote! { query : query.unwrap()}
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            quote! { children : children.unwrap()}
        }

        (_, _, _) => {
            quote! {}
        }
    }
}
