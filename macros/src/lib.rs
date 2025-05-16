extern crate proc_macro;

use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use regex::Regex;
use syn::{FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, LitStr, PatType, meta, parse_macro_input};

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut method: Option<Ident> = None;
    let mut path: Option<LitStr> = None;

    let parser = meta::parser(|meta| {
        if meta.path.is_ident("method") {
            let ident: Ident = meta.value()?.parse()?;

            let valid_methods = [
                "connect", "delete", "get", "head", "options", "patch", "post", "put", "trace",
            ];

            if !valid_methods.contains(&ident.to_string().as_str()) {
                return Err(meta.error("unsupported method; allowed methods are: connect, delete, get, head, options, patch, post, put, trace"));
            }

            method = Some(ident);
            Ok(())
        } else if meta.path.is_ident("path") {
            path = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute key, expected: method or path"))
        }
    });

    parse_macro_input!(args with parser);

    let _method = method.expect("expected #[route(method = ..., path = ...)]");
    let _path = path.expect("expected #[route(method = ..., path = ...)]");

    let input_fn = parse_macro_input!(input as ImplItemFn);

    let sig = &input_fn.sig;
    let body = input_fn.block;
    let asyncness = &sig.asyncness;
    let original_fn_name = &sig.ident;
    let generics = &sig.generics;
    let where_clause = &sig.generics.where_clause;

    let mut wrapper_args = vec![];
    let mut handler_call_args = vec![];
    let mut arg_types = vec![];

    for input in &sig.inputs {
        if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
            let wrapper = match attrs.len() {
                0 => {
                    quote! { #pat: #ty }
                }
                1 => {
                    let attr_name = attrs[0].path().to_token_stream().to_string();
                    match attr_name.as_str() {
                        "query" => quote! { axum::extract::Query(#pat): axum::extract::Query<#ty> },
                        "json" => quote! { axum::extract::Json(#pat): axum::extract::Json<#ty> },
                        other => {
                            panic!("Unsupported attribute #[{}]", other);
                        }
                    }
                }
                _ => {
                    quote! {
                        compile_error!("Only one attribute is allowed per argument");
                    }
                }
            };

            wrapper_args.push(wrapper);
            handler_call_args.push(quote! { #pat });
            arg_types.push(ty.clone());
        }
    }

    let return_type = match &sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    let expanded = quote! {
        #asyncness fn
            #original_fn_name #generics (#(#wrapper_args),*)
            -> std::result::Result<axum::Json<#return_type>, axum::http::StatusCode> #where_clause {
                #body
            }
    };

    println!("expanded: {}", expanded);
    expanded.into()
}

#[proc_macro_attribute]
pub fn routes(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut prefix: Option<LitStr> = None;
    let mut state: Option<LitStr> = None;

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
    let state = state.expect("expected #[routes(prefix = ...)]");
    let state_ident = Ident::new(&state.value().as_str(), state.span());

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
    for item in &input_impl.items {
        if let ImplItem::Fn(ImplItemFn { sig, attrs, .. }) = item {
            for attr in attrs {
                if attr.path().is_ident("route") {
                    let mut method: Option<Ident> = None;
                    let mut path: Option<String> = None;
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("method") {
                            method = Some(meta.value()?.parse()?);
                        } else if meta.path.is_ident("path") {
                            let value: LitStr = meta.value()?.parse()?;
                            path = Some(value.value());
                        } else {
                            return Err(meta.error("expected #[route(method = ..., path = ...)]"));
                        }
                        Ok(())
                    });
                    let method = method.expect("expected method in #[route(...)]");
                    let method_uppercase = method.to_string().to_uppercase();
                    let mut path = path.expect("expected path in #[route(...)]");
                    let fn_name = &sig.ident;
                    let method_ident =
                        Ident::new(&method.to_string().to_lowercase(), method.span());
                    route_entries.push(quote! {
                        .route(#path, axum::routing::#method_ident(Self::#fn_name))
                    });
                    path = prefix_str.clone() + path.as_str();
                    let mut ret_type = None;
                    let mut json_type = None;
                    let mut query_type = None;

                    for input in sig.inputs.iter() {
                        if let syn::FnArg::Typed(pat_type) = input {
                            let attrs = &pat_type.attrs;
                            let ty = &*pat_type.ty;
                            let ty_str = ty.to_token_stream().to_string().replace(' ', "");

                            let mut has_json = false;
                            let mut has_query = false;

                            for attr in attrs {
                                if attr.path().is_ident("json") {
                                    has_json = true;
                                } else if attr.path().is_ident("query") {
                                    has_query = true;
                                }
                            }

                            if has_json && has_query {
                                panic!("Only one of #[json] or #[query] allowed per parameter");
                            } else if has_json {
                                if json_type.is_some() {
                                    panic!("Only one #[json] parameter allowed");
                                }
                                json_type = Some(ty_str);
                            } else if has_query {
                                if query_type.is_some() {
                                    panic!("Only one #[query] parameter allowed");
                                }
                                query_type = Some(ty_str);
                            }
                        }
                    }
                    // Handle return type
                    if let syn::ReturnType::Type(_, ty) = &sig.output {
                        let ty_str = ty.to_token_stream().to_string().replace(' ', "");
                        ret_type = Some(ty_str);
                    }

                    // Prepare type names for TS
                    let ts_json = json_type.clone().unwrap_or_else(|| "void".to_string());
                    let ts_query = query_type.clone();
                    let ts_ret = ret_type.clone().unwrap_or_else(|| "void".to_string());

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

                    let fn_args = match (&ts_json[..], &ts_query) {
                        ("void", None) => "".to_string(),
                        ("void", Some(query)) => format!("query: {}", query),
                        (json, None) => format!("body: {}", json),
                        (json, Some(query)) => format!("body: {}, query: {}", json, query),
                    };

                    let query_string = if ts_query.is_some() {
                        " + '?' + new URLSearchParams(Object.entries(query).filter(([_, value]) => value != null))"
                    } else {
                        ""
                    };

                    let body_part = if ts_json != "void" {
                        "body: JSON.stringify(body),"
                    } else {
                        ""
                    };
                    ts_routes.push(quote! {
                        bindings.push(format!(
"export async function {}({}): Promise<{}> {{
    const res = await fetch(`{}`{}, {{
        method: '{}',
        headers: {{ 'Content-Type': 'application/json' }},
        {}
    }});
    return await res.json();
}}\n",
                            stringify!(#fn_name),
                            #fn_args,
                            #ts_ret,
                            #path,
                            #query_string,
                            #method_uppercase,
                            #body_part
                        ));
                    });
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
    let expanded = quote! {
        #input_impl
        #[test]
        fn #routes_ident() {
            let mut bindings = Vec::new();
            #(#ts_routes)*

            let output = bindings.join("\n");
            std::fs::write(format!("bindings/{}.ts", #routes_name), format!("// this file is autogenerated DONT EDIT IT!\n\n{}\n\nexport namespace {} {{\n{}\n}}\nexport default {};\n", #imports, #routes_name, output, #routes_name)).expect("failed to write TypeScript bindings");
        }
    };
    println!("expanded: {}", expanded);
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
