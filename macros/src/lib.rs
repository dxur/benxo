extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    FnArg, Ident, ItemTrait, LitStr, PatType, TraitItem, TraitItemFn, meta, parse_macro_input,
};

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

    let method = method.expect("expected #[route(method = ..., path = ...)]");
    let path = path.expect("expected #[route(method = ..., path = ...)]");
    let path_str = path.value();

    let input_fn = parse_macro_input!(input as TraitItemFn);
    if input_fn.default.is_some() {
        panic!("Expected a function without a default implementation");
    }

    let sig = &input_fn.sig;
    let asyncness = &sig.asyncness;
    let original_fn_name = &sig.ident;
    let generics = &sig.generics;
    let where_clause = &sig.generics.where_clause;

    #[cfg(not(feature = "wasm"))]
    {
        let mut wrapper_args = vec![];
        let mut handler_call_args = vec![];
        let mut arg_types = vec![];

        for input in &sig.inputs {
            if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
                for attr in attrs {
                    let attr_name = attr.path().to_token_stream().to_string();
                    let wrapper = match attr_name.as_str() {
                        "query" => quote! { #pat: axum::extract::Query<#ty> },
                        "body" => quote! { #pat: axum::extract::Json<#ty> },
                        "ignore" => quote! { #pat: #ty },
                        other => quote! {
                            compile_error!(concat!("Unsupported attribute #[", #other, "]"));
                        },
                    };

                    wrapper_args.push(wrapper);
                    handler_call_args.push(quote! { #pat });
                    arg_types.push(ty.clone());
                }
            }
        }

        let return_type = match &sig.output {
            syn::ReturnType::Default => quote! { () },
            syn::ReturnType::Type(_, ty) => quote! { #ty },
        };

        let expanded = quote! {
            #asyncness fn #original_fn_name #generics (state: axum::extract::State<S>, #(#wrapper_args),*) -> std::result::Result<axum::Json<#return_type>, axum::http::StatusCode> #where_clause;
        };

        expanded.into()
    }

    #[cfg(feature = "wasm")]
    {
        let mut query_args = vec![];
        let mut body_args = vec![];
        let mut args = vec![];

        for input in &sig.inputs {
            if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
                for attr in attrs {
                    let attr_name = attr.path().to_token_stream().to_string();
                    if attr_name == "ignore" {
                        continue;
                    }
                    match attr_name.as_str() {
                        "query" => {
                            if query_args.len() > 0 {
                                panic!("Only one query parameter is allowed");
                            }
                            query_args.push((pat.clone(), ty.clone()))
                        }
                        "body" => {
                            if body_args.len() > 0 {
                                panic!("Only one body parameter is allowed");
                            }
                            body_args.push((pat.clone(), ty.clone()))
                        }
                        other => panic!("Unsupported attribute #[{}]", other),
                    };
                    args.push(quote! { #pat: #ty });
                }
            }
        }

        let return_type = match &sig.output {
            syn::ReturnType::Default => quote! { () },
            syn::ReturnType::Type(_, ty) => quote! { #ty },
        };

        let json_body = {
            if body_args.len() > 0 {
                let name = &body_args[0].0;
                quote! { .json(&#name)? }
            } else {
                quote! {}
            }
        };

        let req_head = {
            if query_args.len() > 0 {
                let name = &query_args[0].0;
                quote! {
                    gloo_net::http::Request::#method(format!("{}{}?{}", Self::PREFIX.to_owned(), #path_str, serde_urlencoded::to_string(#name).map_err(|e| gloo_net::Error::GlooError(format!("{}", e)))?).as_str())
                }
            } else {
                quote! {
                    gloo_net::http::Request::#method(format!("{}{}", Self::PREFIX.to_owned(), #path_str).as_str())
                }
            }
        };

        let expanded = quote! {
            #asyncness fn #original_fn_name #generics (#(#args),*) -> std::result::Result<#return_type, gloo_net::Error> #where_clause {
                #req_head
                #json_body
                .send().await?.json().await
            }
        };
        println!("expanded: {}", expanded);
        expanded.into()
    }
}

#[proc_macro_attribute]
pub fn routes(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut prefix: Option<LitStr> = None;
    let mut for_ident_opt: Option<Ident> = None;

    let parser = meta::parser(|meta| {
        if meta.path.is_ident("prefix") {
            prefix = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("for") {
            for_ident_opt = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute key, expected: prefix, for"))
        }
    });

    parse_macro_input!(args with parser);
    let prefix = prefix.expect("expected #[routes(prefix = ..., for = ...)]");
    let prefix_str = prefix.value();
    let for_ident = for_ident_opt.expect("expected #[routes(prefix = ..., for = ...)]");

    let mut input_trait = parse_macro_input!(input as ItemTrait);

    // TODO: check the routes for duplicate routes same method same path!!

    #[cfg(not(feature = "wasm"))]
    {
        let mut route_entries = vec![];
        let mut router_args = vec![];
        let mut router_generics = vec![];
        let mut router_generics_constraints = vec![];
        let mut i: i32 = 0;
        for item in &input_trait.items {
            if let TraitItem::Fn(TraitItemFn { sig, attrs, .. }) = item {
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
                                return Err(
                                    meta.error("expected #[route(method = ..., path = ...)]")
                                );
                            }
                            Ok(())
                        });
                        let method = method.expect("expected method in #[route(...)]");
                        let path = path.expect("expected path in #[route(...)]");
                        let fn_name = &sig.ident;
                        let method_ident =
                            Ident::new(&method.to_string().to_lowercase(), method.span());
                        route_entries.push(quote! {
                            .route(#path, axum::routing::#method_ident(#fn_name))
                        });
                        i += 1;
                        let generic_h_ident = Ident::new(&format!("H{}", i), Span::call_site());
                        let generic_t_ident = Ident::new(&format!("T{}", i), Span::call_site());
                        router_args.push(quote! {
                            #fn_name: #generic_h_ident
                        });
                        router_generics.push(quote! {
                            #generic_h_ident, #generic_t_ident
                        });
                        router_generics_constraints.push(quote! {
                            #generic_h_ident: axum::handler::Handler<#generic_t_ident, S>,
                            #generic_t_ident: 'static,
                        });
                    }
                }
            }
        }

        let router_fn: TraitItem = syn::parse_quote! {
            fn make_router<#(#router_generics),*>(#(#router_args),*) -> (&'static str, axum::Router<S>)
            where
                #(#router_generics_constraints)*
            {
                let router = axum::Router::new()
                    #(#route_entries)*;
                (#prefix_str, router)
            }
        };
        input_trait.items.push(router_fn);
        input_trait
            .generics
            .params
            .push(syn::parse_quote! { S: 'static + Send + Sync + Clone + 'static });
        let expanded = quote! {
            #input_trait
        };
        println!("expanded: {}", expanded);
        expanded.into()
    }

    #[cfg(feature = "wasm")]
    {
        let trait_name = &input_trait.ident;
        let routes_prefix: TraitItem = syn::parse_quote! {
            const PREFIX: &str = #prefix_str;
        };
        input_trait.items.push(routes_prefix);
        quote! {
            #input_trait
            impl #trait_name for #for_ident {}
        }
        .into()
    }
}

#[cfg(feature = "wasm")]
#[proc_macro_attribute]
pub fn routes_builder(args: TokenStream, input: TokenStream) -> TokenStream {
    use syn::{Expr, ExprTuple, ImplItem, ItemImpl, Type};

    let mut as_ident_opt: Option<Ident> = None;
    let mut use_ident_opt: Option<Ident> = None;

    let parser = meta::parser(|meta| {
        if meta.path.is_ident("as") {
            as_ident_opt = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("use") {
            use_ident_opt = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute key, expected: as, use"))
        }
    });

    parse_macro_input!(args with parser);
    let as_ident = as_ident_opt.expect("expected #[routes(as = ...)]");
    let use_ident = use_ident_opt.expect("expected #[routes(use = ...)]");

    let input_impl = parse_macro_input!(input as ItemImpl);

    let mut route_views = Vec::new();

    for item in &input_impl.items {
        if let ImplItem::Const(const_item) = item {
            if let Type::Path(type_path) = &const_item.ty {
                if type_path.path.segments.last().map(|s| s.ident.to_string())
                    == Some("Route".to_string())
                {
                    if let Expr::Tuple(ExprTuple { elems, .. }) = &const_item.expr {
                        if elems.len() == 2 {
                            let path = &elems[0];
                            let component = &elems[1];

                            let route_view = quote! {
                                <leptos_router::components::Route path=path!(#path) view=*#component/>
                            };

                            route_views.push(route_view);
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        #input_impl

        #[component(transparent)]
        pub fn #as_ident() -> impl leptos_router::MatchNestedRoutes + Clone {
            use leptos::prelude::*;
            use leptos_router::*;
            use leptos_router::components::*;

            let _ = #use_ident.with(|cell| cell.set(Box::new(use_navigate())));
            view! {
                #(#route_views)*
            }
            .into_inner()
        }
    }
    .into();

    println!("expanded: {}", expanded);
    expanded
}
