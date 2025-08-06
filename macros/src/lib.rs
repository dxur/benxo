extern crate proc_macro;

use core::panic;
use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use regex::Regex;
use syn::{
    DeriveInput, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, LitStr, Meta, PatType, Path, Type,
    meta, parse_macro_input, punctuated::Punctuated,
};

#[proc_macro_derive(Model, attributes(model))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut coll = None;
    let mut indb = None;
    let mut find = None;
    let mut fetch = None;
    let mut listable = None;
    let mut create = None;
    let mut update = None;
    let mut delete = None;

    for attr in input.attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {
            if attr_meta_name == "model" {
                let attr_meta = attr.meta;
                match attr_meta {
                    Meta::List(list) => {
                        list.parse_nested_meta(|meta| {
                            let key = meta.path.get_ident().unwrap().to_string();
                            match key.as_str() {
                                "coll" => coll = Some(meta.value()?.parse::<LitStr>()?),
                                "indb" => indb = Some(meta.value()?.parse::<Path>()?),
                                "find" => find = Some(meta.value()?.parse::<Path>()?),
                                "fetch" => fetch = Some(meta.value()?.parse::<Path>()?),
                                "list" => listable = Some(meta.value()?.parse::<Path>()?),
                                "create" => create = Some(meta.value()?.parse::<Path>()?),
                                "update" => update = Some(meta.value()?.parse::<Path>()?),
                                "delete" => delete = Some(meta.value()?.parse::<Path>()?),
                                v => Err(meta.error(format!("unsupported attribute {}", v)))?,
                            }
                            Ok(())
                        })
                        .unwrap();
                    }
                    _ => panic!("Incorrect format for using the `model` attribute."),
                }
            }
        }
    }

    let name = &input.ident;
    let mut expanded = quote! {};

    if let Some(col) = coll {
        if let Some(p) = indb {
            expanded.extend(quote! {
                impl ModelInDb for #name {
                    const COLLECTION_NAME: &'static str = #col;
                    type InDb = #p;
                }
            });
        }
    } else if coll.is_some() || indb.is_some() {
        panic!("Both 'coll' and 'indb' must be specified together, or neither.");
    }

    if let Some(p) = find {
        expanded.extend(quote! {
            impl FindableInDb for #name {
                type FindInDb = #p;
            }
        });
    }

    if let Some(p) = fetch {
        expanded.extend(quote! {
            impl FetchableInDb for #name {
                type FetchInDb = #p;
            }
        });
    }

    if let Some(p) = listable {
        expanded.extend(quote! {
            impl ListableInDb for #name {
                type ListInDb = #p;
            }
        });
    }

    if let Some(p) = create {
        expanded.extend(quote! {
            impl CreatableInDb for #name {
                type CreateInDb = #p;
            }
        });
    }

    if let Some(p) = update {
        expanded.extend(quote! {
            impl UpdatableInDb for #name {
                type UpdateInDb = #p;
            }
        });
    }

    if let Some(p) = delete {
        expanded.extend(quote! {
            impl DeletableInDb for #name {
                type DeleteInDb = #p;
            }
        });
    }

    TokenStream::from(expanded)
}

// #[proc_macro_attribute]
// pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut method: Option<Ident> = None;
//     let mut path: Option<LitStr> = None;

//     let parser = meta::parser(|meta| {
//         if meta.path.is_ident("method") {
//             let ident: Ident = meta.value()?.parse()?;

//             let valid_methods = [
//                 "connect", "delete", "get", "head", "options", "patch", "post", "put", "trace",
//             ];

//             if !valid_methods.contains(&ident.to_string().as_str()) {
//                 return Err(meta.error("unsupported method; allowed methods are: connect, delete, get, head, options, patch, post, put, trace"));
//             }

//             method = Some(ident);
//             Ok(())
//         } else if meta.path.is_ident("path") {
//             path = Some(meta.value()?.parse()?);
//             Ok(())
//         } else {
//             Err(meta.error("unsupported attribute key, expected: method or path"))
//         }
//     });

//     parse_macro_input!(args with parser);

//     let _method = method.expect("expected #[route(method = ..., path = ...)]");
//     let _path = path.expect("expected #[route(method = ..., path = ...)]");

//     let input_fn = parse_macro_input!(input as ImplItemFn);

//     let sig = &input_fn.sig;
//     let body = input_fn.block;
//     let asyncness = &sig.asyncness;
//     let original_fn_name = &sig.ident;
//     let generics = &sig.generics;
//     let where_clause = &sig.generics.where_clause;

//     let mut wrapper_args = vec![];
//     let mut handler_call_args = vec![];
//     let mut arg_types = vec![];

//     for input in &sig.inputs {
//         if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
//             let wrapper = match attrs.len() {
//                 0 => {
//                     quote! { #pat: #ty }
//                 }
//                 1 => {
//                     let attr_name = attrs[0].path().to_token_stream().to_string();
//                     match attr_name.as_str() {
//                         "query" => quote! { axum::extract::Query(#pat): axum::extract::Query<#ty> },
//                         "json" => quote! { axum::extract::Json(#pat): axum::extract::Json<#ty> },
//                         other => {
//                             panic!("Unsupported attribute #[{}]", other);
//                         }
//                     }
//                 }
//                 _ => {
//                     quote! {
//                         compile_error!("Only one attribute is allowed per argument");
//                     }
//                 }
//             };

//             wrapper_args.push(wrapper);
//             handler_call_args.push(quote! { #pat });
//             arg_types.push(ty.clone());
//         }
//     }

//     let return_type = match &sig.output {
//         syn::ReturnType::Default => quote! { () },
//         syn::ReturnType::Type(_, ty) => quote! { #ty },
//     };

//     let expanded = quote! {
//         #asyncness fn
//             #original_fn_name #generics (#(#wrapper_args),*)
//             -> std::result::Result<axum::Json<#return_type>, axum::http::StatusCode> #where_clause {
//                 #body
//             }
//     };

//     println!("expanded: {}", expanded);
//     expanded.into()
// }

#[proc_macro_attribute]
pub fn routes(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut prefix: Option<LitStr> = None;
    let mut state: Option<Path> = None;

    let parser = meta::parser(|meta| {
        if meta.path.is_ident("prefix") {
            prefix = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("state") {
            state = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute key, expected: prefix, state"))
        }
    });

    parse_macro_input!(args with parser);
    let prefix = prefix.expect("expected #[routes(prefix = ...)]");
    let prefix_str = prefix.value();
    let state_ident = state.expect("expected #[routes(state = ...)]");

    let mut input_impl = parse_macro_input!(input as ItemImpl);
    let routes_name = input_impl.self_ty.to_token_stream().to_string();
    let routes_ident = Ident::new(
        format!("export_bindings_{}", routes_name.to_lowercase()).as_str(),
        Span::call_site(),
    );

    // TODO: check the routes for duplicate routes same method same path!!

    let mut route_entries = vec![];
    let mut ts_routes = vec![];
    let mut import_lines = HashSet::new();
    let mut insert_import = |i: &String| {
        let re = Regex::new(r"\b[A-Z][a-zA-Z0-9]*\b").unwrap();
        for cap in re.find_iter(i) {
            import_lines.insert(format!(
                "import type {{ {} }} from './{}.ts';",
                cap.as_str(),
                cap.as_str()
            ));
        }
    };

    for item in &mut input_impl.items {
        if let ImplItem::Fn(ImplItemFn {
            sig, attrs, block, ..
        }) = item
        {
            let original_block = block.clone();
            let mut route_attr_index = None;
            let mut route_info = None;

            // Find the route attribute
            for (i, attr) in attrs.iter().enumerate() {
                if attr.path().is_ident("route") {
                    route_attr_index = Some(i);

                    let mut method: Option<Ident> = None;
                    let mut path: Option<String> = None;
                    let mut route_type: Option<Ident> = None;
                    let mut res_type: Option<Type> = None;

                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("method") {
                            method = Some(meta.value()?.parse()?);
                        } else if meta.path.is_ident("path") {
                            let value: LitStr = meta.value()?.parse()?;
                            path = Some(value.value());
                        } else if meta.path.is_ident("type") {
                            route_type = Some(meta.value()?.parse()?);
                        } else if meta.path.is_ident("res") {
                            res_type = Some(meta.value()?.parse()?);
                        } else {
                            return Err(meta.error("expected #[route(method = ..., path = ..., type = ..., res = ...)]"));
                        }
                        Ok(())
                    });

                    let method = method.expect("expected method in #[route(...)]");
                    let path = path.expect("expected path in #[route(...)]");

                    // Validate type - only allow "json" for now
                    if let Some(ref t) = route_type {
                        if t != "json" {
                            panic!(
                                "unsupported type: {}, only 'json' is currently supported",
                                t
                            );
                        }
                    }

                    route_info = Some((method, path, route_type, res_type));
                    break;
                }
            }

            if let Some((method, path, _route_type, res_type)) = route_info {
                // Remove the route attribute
                if let Some(index) = route_attr_index {
                    attrs.remove(index);
                }

                let valid_methods = [
                    "connect", "delete", "get", "head", "options", "patch", "post", "put", "trace",
                ];

                if !valid_methods.contains(&method.to_string().as_str()) {
                    panic!("unsupported method: {}", method)
                }

                let method_uppercase = method.to_string().to_uppercase();
                let fn_name = &sig.ident;
                let method_ident = Ident::new(&method.to_string().to_lowercase(), method.span());

                // Transform function parameters to handle #[json], #[query] and #[path] attributes
                let mut wrapper_args = vec![];
                let mut handler_call_args = vec![];
                let mut json_type = None;
                let mut query_type = None;
                let mut path_types = Vec::new();

                for input in &mut sig.inputs {
                    if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
                        let ty_str = ty.to_token_stream().to_string().replace(' ', "");
                        let pat_str = pat.to_token_stream().to_string().replace(' ', "");

                        match attrs.len() {
                            0 => {
                                wrapper_args.push(quote! { #pat: #ty });
                                handler_call_args.push(quote! { #pat });
                            }
                            1 => {
                                let attr_name = attrs[0].path().to_token_stream().to_string();
                                match attr_name.as_str() {
                                    "query" => {
                                        if query_type.is_some() {
                                            panic!("Only one #[query] parameter allowed");
                                        }
                                        query_type = Some(ty_str);
                                        wrapper_args.push(quote! { Query(#pat): Query<#ty> });
                                        handler_call_args.push(quote! { #pat });
                                        attrs.clear();
                                    }
                                    "json" => {
                                        if json_type.is_some() {
                                            panic!("Only one #[json] parameter allowed");
                                        }
                                        json_type = Some(ty_str);
                                        wrapper_args.push(quote! { Json(#pat): Json<#ty> });
                                        handler_call_args.push(quote! { #pat });
                                        attrs.clear();
                                    }
                                    "path" => {
                                        path_types.push((pat_str, "any".to_string()));
                                        wrapper_args.push(quote! { Path(#pat): Path<#ty> });
                                        handler_call_args.push(quote! { #pat });
                                        attrs.clear();
                                    }
                                    other => {
                                        panic!("Unsupported attribute #[{}]", other);
                                    }
                                }
                            }
                            _ => {
                                panic!("Only one attribute is allowed per argument");
                            }
                        }
                    }
                }

                // Update the function signature with the transformed parameters
                let mut new_inputs = Punctuated::new();
                for arg in wrapper_args.iter() {
                    let parsed_arg: FnArg =
                        syn::parse2(arg.clone()).expect("Failed to parse transformed argument");
                    new_inputs.push(parsed_arg);
                }
                sig.inputs = new_inputs;

                // // Create the return type wrapper for JSON responses
                // let return_type = match &sig.output {
                //     syn::ReturnType::Default => quote! { () },
                //     syn::ReturnType::Type(_, ty) => quote! { #ty },
                // };

                // // Update the return type to be Result<Json<T>, StatusCode>
                // sig.output = syn::parse_quote! {
                //     -> std::result::Result<axum::Json<#return_type>, axum::http::StatusCode>
                // };

                route_entries.push(quote! {
                    .route(#path, axum::routing::#method_ident(Self::#fn_name))
                });

                let mut full_path = prefix_str.clone() + path.as_str();

                let should_export = res_type.as_ref().is_some();

                let ts_ret = if let Some(res) = res_type.as_ref() {
                    quote!(#res).to_string()
                } else {
                    "void".to_string()
                };

                let ts_json = json_type.clone().unwrap_or_else(|| "void".to_string());
                let ts_query = query_type.clone();

                // Add imports
                if ts_json != "void" {
                    insert_import(&ts_json);
                }
                if let Some(ref query) = ts_query {
                    if query != &ts_json {
                        insert_import(query);
                    }
                }
                if ts_ret != "void"
                    && ts_ret != ts_json
                    && ts_ret != ts_query.clone().unwrap_or_default()
                {
                    insert_import(&ts_ret);
                }

                let mut fn_args = match (&ts_json[..], &ts_query) {
                    ("void", None) => "".to_string(),
                    ("void", Some(query)) => format!("query: {}, ", query),
                    (json, None) => format!("body: {}, ", json),
                    (json, Some(query)) => format!("body: {}, query: {}, ", json, query),
                };

                let mut path_args = String::new();
                for (pat, ty) in &path_types {
                    full_path = full_path.replace(&format!("{{{}}}", pat), &format!("${{{}}}", pat));
                    path_args += &format!("{}: {}, ", pat, ty);
                }

                fn_args = path_args + &fn_args;

                let query_string = if ts_query.is_some() {
                    " + '?' + new URLSearchParams(Object.entries(query).filter(([_, value]) => value != null))"
                } else {
                    ""
                };

                let body_part = if ts_json != "void" {
                    concat!(
                        "headers: { 'Content-Type': 'application/json' },",
                        "body: JSON.stringify(body),"
                    )
                } else {
                    ""
                };
                let ret_part = if ts_ret != "void" {
                    "return await res.json();"
                } else {
                    "return;"
                };

                ts_routes.push(quote! {
                    bindings.push(format!(
"export async function {}({}): Promise<{}> {{
    const res = await fetch(`{}`{}, {{
        method: '{}',
        {}
    }});
    if (res.ok) {{
        {}
    }} else {{
        throw await res.text();
    }}
}}\n",
                        stringify!(#fn_name),
                        #fn_args,
                        #ts_ret,
                        #full_path,
                        #query_string,
                        #method_uppercase,
                        #body_part,
                        #ret_part
                    ));
                });

                if should_export {
                    *block = syn::parse_quote! {
                        {
                            let response = (async move ||
                                #original_block
                            )().await;
                            {
                                #[inline(always)]
                                fn __asert_type<t: crate::utils::macros::ContainsJson<#res_type>>(r: &t) {}
                                __asert_type(&response);
                            }
                            response
                        }
                    };
                } else {
                    *block = syn::parse_quote! {
                        #original_block
                    };
                }
            }
        }
    }

    let router_fn: ImplItem = syn::parse_quote! {
        pub fn make_router() -> (&'static str, axum::Router<#state_ident>) {
            let router = axum::Router::new()
                #(#route_entries)*;
            (#prefix_str, router)
        }
    };
    input_impl.items.push(router_fn);

    let imports = import_lines.into_iter().collect::<Vec<String>>().join("\n");
    let mut expanded_test = quote! {};
    if ts_routes.len() > 0 {
        expanded_test = quote! {
            #[test]
            fn #routes_ident() {
                let mut bindings = Vec::new();
                #(#ts_routes)*
                let output = bindings.join("\n");
                std::fs::write(format!("bindings/{}.ts", #routes_name),
                    format!("// this file is autogenerated DONT EDIT IT!\n\n{}\n\n{}\n", #imports, output)
                ).expect("failed to write TypeScript bindings");
            }
        };
    }
    let expanded = quote! {
        #input_impl
        #expanded_test
    };

    expanded.into()
}

// #[cfg(feature = "wasm")]
// #[proc_macro_attribute]
// pub fn routes_builder(args: TokenStream, input: TokenStream) -> TokenStream {
//     use syn::{Expr, ExprTuple, ImplItem, ItemImpl, Type};

//     let mut as_ident_opt: Option<Ident> = None;
//     let mut use_ident_opt: Option<Ident> = None;

//     let parser = meta::parser(|meta| {
//         if meta.path.is_ident("as") {
//             as_ident_opt = Some(meta.value()?.parse()?);
//             Ok(())
//         } else if meta.path.is_ident("use") {
//             use_ident_opt = Some(meta.value()?.parse()?);
//             Ok(())
//         } else {
//             Err(meta.error("unsupported attribute key, expected: as, use"))
//         }
//     });

//     parse_macro_input!(args with parser);
//     let as_ident = as_ident_opt.expect("expected #[routes(as = ...)]");
//     let use_ident = use_ident_opt.expect("expected #[routes(use = ...)]");

//     let input_impl = parse_macro_input!(input as ItemImpl);

//     let mut route_views = Vec::new();

//     for item in &input_impl.items {
//         if let ImplItem::Const(const_item) = item {
//             if let Type::Path(type_path) = &const_item.ty {
//                 if type_path.path.segments.last().map(|s| s.ident.to_string())
//                     == Some("Route".to_string())
//                 {
//                     if let Expr::Tuple(ExprTuple { elems, .. }) = &const_item.expr {
//                         if elems.len() == 2 {
//                             let path = &elems[0];
//                             let component = &elems[1];

//                             let route_view = quote! {
//                                 <leptos_router::components::Route path=path!(#path) view=*#component/>
//                             };

//                             route_views.push(route_view);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     let expanded = quote! {
//         #input_impl

//         #[component(transparent)]
//         pub fn #as_ident() -> impl leptos_router::MatchNestedRoutes + Clone {
//             use leptos::prelude::*;
//             use leptos_router::*;
//             use leptos_router::components::*;

//             let _ = #use_ident.with(|cell| cell.set(Box::new(use_navigate())));
//             view! {
//                 #(#route_views)*
//             }
//             .into_inner()
//         }
//     }
//     .into();

//     println!("expanded: {}", expanded);
//     expanded
// }
