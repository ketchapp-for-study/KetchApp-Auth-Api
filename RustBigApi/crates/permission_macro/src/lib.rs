extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn permission(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments as a Vec<Lit>
    let args = syn::punctuated::Punctuated::<Lit, syn::Token![,]>::parse_terminated
        .parse(args)
        .expect("Failed to parse permission macro arguments");
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract all permission strings from the attribute
    let required_perms: Vec<String> = args.iter().filter_map(|lit| {
        if let Lit::Str(lit_str) = lit {
            Some(lit_str.value())
        } else {
            None
        }
    }).collect();
    if required_perms.is_empty() {
        panic!("permission macro requires at least one string argument");
    }

    // Only support a single permission string for now (can be extended for multiple)
    if required_perms.len() != 1 {
        panic!("permission macro requires exactly one string argument for this usage");
    }
    let perm = &required_perms[0];

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    let expanded = quote! {
        #(#attrs)*
        const REQUIRED_PERMISSION: &str = #perm;
        #vis #sig #block
    };
    TokenStream::from(expanded)
}
