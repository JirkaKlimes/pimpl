//! Procedural macros for the `pimpl` crate. Depend on the [`pimpl`] facade
//! crate instead of using this directly.
//!
//! [`pimpl`]: https://docs.rs/pimpl

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks a function as a *probabilistic implementation*.
///
/// At this stage the macro is a stub: it discards the body and substitutes
/// `todo!("pimpl: not yet observed")`. Later versions will resolve the
/// function against `pimpl.collapsed` and splice the cached body in place.
#[proc_macro_attribute]
pub fn pimpl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    func.block = syn::parse_quote!({ todo!("pimpl: not yet observed") });
    quote!(#func).into()
}
