// extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

#[proc_macro_derive(ListNodeDebug)]
pub fn list_node_debug(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let stream = impl_debug(&input.ident);
    TokenStream::from(stream)
}

#[proc_macro_derive(PreOrder)]
pub fn pre_order(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let stream = impl_pre_order(&input.ident);
    TokenStream::from(stream)
}

#[proc_macro_derive(InfixOrder)]
pub fn infix_order(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let stream = impl_infix_order(&input.ident);
    TokenStream::from(stream)
}

fn impl_debug(ident: &Ident) -> proc_macro2::TokenStream {
    quote!(
        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut node = &mut self.clone();
                let mut str = "".to_string();
                loop {
                    str.push_str(&node.val.to_string());
                    str.push_str("->");
                    node = match node.next.as_mut() {
                        Some(n) => n.as_mut(),
                        None => break,
                    };
                }
                str.remove(str.len() - 1);
                str.remove(str.len() - 1);
                write!(f, "{}", str)
            }
        }
    )
}

fn impl_infix_order(ident: &Ident) -> proc_macro2::TokenStream {
    quote!(
        impl #ident {
            fn infix_order(&self) {
                if let Some(left) = &self.left {
                    left.borrow().infix_order();
                }
                println!("{}", self.val);
                if let Some(right) = &self.right {
                    right.borrow().infix_order();
                }
            }
        }
    )
}

fn impl_pre_order(ident: &Ident) -> proc_macro2::TokenStream {
    quote!(
        impl #ident {
            fn pre_order(&self) {
                println!("{}", self.val);
                if let Some(left) = &self.left {
                    left.borrow().pre_order();
                }
                if let Some(right) = &self.right {
                    right.borrow().pre_order();
                }
            }
        }
    )
}
