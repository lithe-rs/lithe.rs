use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    Block, Expr, ExprMethodCall, ExprPath, ItemFn,
};

#[proc_macro_attribute]
pub fn client(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = quote! {
        #[allow(dead_code)]
        #input
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

                    let block: Block = if let Expr::Block(b) = &*closure.body {
                        b.block.clone()
                    } else {
                        let body = &closure.body;
                        parse_quote!({ #body })
                    };

                    let anon_fn: ItemFn = parse_quote! {
                        #[cfg(target_arch = "wasm32")]
                        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #handler_name)]
                        pub fn #handler_ident() {
                            #block
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
    let mut input = parse_macro_input!(item as ItemFn);
    let base_name = input.sig.ident.to_string();

    let mut visitor = OnClickVisitor {
        anon_handlers: Vec::new(),
        base_name,
        count: 0,
    };

    visitor.visit_item_fn_mut(&mut input);

    let anon_fns = visitor.anon_handlers;

    let expanded = quote! {
        #input
        #(#anon_fns)*
    };
    TokenStream::from(expanded)
}
