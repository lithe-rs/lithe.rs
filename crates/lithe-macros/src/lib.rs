use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    Block, Expr, ExprMethodCall, ExprPath, FnArg, ItemFn, Pat, ReturnType,
};

#[proc_macro_attribute]
pub fn client(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let expanded = quote! {
        #[allow(dead_code)]
        #input_fn
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn server(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let visibility = &input_fn.vis;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    // Extract argument names and types
    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();

    for arg in inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                arg_names.push(&pat_ident.ident);
                arg_types.push(&pat_type.ty);
            }
        }
    }

    let args_tuple_type = if arg_types.len() == 1 {
        let ty = &arg_types[0];
        quote! { #ty }
    } else {
        quote! { (#(#arg_types),*) }
    };

    let args_unpack = if arg_names.len() == 1 {
        let name = &arg_names[0];
        quote! { let #name = args; }
    } else {
        quote! { let (#(#arg_names),*) = args; }
    };

    let internal_fn_name = syn::Ident::new(
        &format!("__lithe_rpc_{}", fn_name),
        proc_macro2::Span::call_site(),
    );

    let fn_name_str = fn_name.to_string();

    let output_type = match output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let expanded = quote! {
        #[cfg(not(target_arch = "wasm32"))]
        #input_fn

        #[cfg(not(target_arch = "wasm32"))]
        #[allow(dead_code)]
        pub async fn #internal_fn_name(args: #args_tuple_type) -> ::lithe::serde_json::Value {
            // Compile-time check for Serialize
            fn _check_serialize<T: ::lithe::serde::Serialize>() {}
            _check_serialize::<#output_type>();

            #args_unpack
            let res = #fn_name(#(#arg_names),*).await;
            ::lithe::serde_json::to_value(res).expect("Failed to serialize RPC result")
        }

        #[cfg(target_arch = "wasm32")]
        #[allow(dead_code)]
        #visibility async fn #fn_name(#inputs) #output {
            ::lithe::browser::call_server(#fn_name_str, (#(#arg_names),*)).await
        }
    };

    TokenStream::from(expanded)
}

struct OnClickVisitor {
    anon_handlers: Vec<ItemFn>,
    base_name: String,
    count: u32,
}

impl VisitMut for OnClickVisitor {
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        visit_mut::visit_expr_method_call_mut(self, node);

        if node.method == "on_click" && node.args.len() == 1 {
            let arg = &node.args[0];

            match arg {
                // Case 1: on_click(handle_click) - Path to a function
                Expr::Path(ExprPath { path, .. }) => {
                    let mut filtered = Vec::new();
                    for seg in &path.segments {
                        let s = seg.ident.to_string();
                        if s != "crate" && s != "super" && s != "self" {
                            filtered.push(s);
                        }
                    }
                    let path_str = filtered.join("_");
                    let final_str = format!("Lithe.dispatch('{}')", path_str);
                    let new_arg: Expr = parse_quote!(#final_str);
                    node.args[0] = new_arg;
                }
                // Case 2: on_click(|| { ... }) - Closure
                Expr::Closure(closure) => {
                    self.count += 1;
                    let handler_name = format!("{}_anon_{}", self.base_name, self.count);
                    let handler_ident =
                        syn::Ident::new(&handler_name, proc_macro2::Span::call_site());

                    let body = &closure.body;
                    let block: Block = match &**body {
                        Expr::Block(b) => b.block.clone(),
                        Expr::Async(a) => a.block.clone(),
                        _ => parse_quote!({ #body }),
                    };

                    let anon_fn: ItemFn = parse_quote! {
                        #[cfg(target_arch = "wasm32")]
                        #[::lithe::wasm_bindgen::prelude::wasm_bindgen(js_name = #handler_name)]
                        pub fn #handler_ident() {
                            ::lithe::wasm_bindgen_futures::spawn_local(async move {
                                #block
                            });
                        }
                    };

                    self.anon_handlers.push(anon_fn);

                    let dispatch_str = format!("Lithe.dispatch('{}')", handler_name);
                    let new_arg: Expr = parse_quote!(#dispatch_str);
                    node.args[0] = new_arg;
                }
                _ => {}
            }
        }
    }
}

#[proc_macro_attribute]
pub fn page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let base_name = input_fn.sig.ident.to_string();

    let mut visitor = OnClickVisitor {
        anon_handlers: Vec::new(),
        base_name,
        count: 0,
    };

    visitor.visit_item_fn_mut(&mut input_fn);

    let anon_fns = visitor.anon_handlers;

    let expanded = quote! {
        #input_fn
        #(#anon_fns)*
    };
    TokenStream::from(expanded)
}
