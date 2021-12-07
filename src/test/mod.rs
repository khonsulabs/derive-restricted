mod basic;
mod bound;
mod enum_;
mod misc;
mod skip;
mod use_case;
#[cfg(feature = "zeroize")]
mod zeroize;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

fn test_derive(input: TokenStream, expected: TokenStream) -> Result<()> {
	let left = crate::derive_where_internal(input)?.to_string();
	let right = quote! { #expected }.to_string();

	assert_eq!(left, right);
	Ok(())
}

fn compiles(input: TokenStream) -> Result<()> {
	crate::derive_where_internal(input).map(|_| ())
}
