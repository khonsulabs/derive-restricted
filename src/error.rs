//! Error type.

use proc_macro2::Span;

/// Easy API to create all [`syn::Error`] messages in this crate.
pub struct Error;

impl Error {
	/// Unsupported item.
	pub fn item(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			"derive-where doesn't support items without generics, `skip` attributes or `enum`s \
			 implementing `Default`, as this can already be handled by standard `#[derive(..)]`",
		)
	}

	/// Unsupported unit struct.
	pub fn unit_struct(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			"derive-where doesn't support unit structs, as this can already be handled by \
			 standard `#[derive(..)]`",
		)
	}

	/// Unsupported trait for union.
	pub fn union(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			"traits other then `Clone` and `Copy` aren't supported by unions",
		)
	}

	/// Invalid syntax for attribute.
	pub fn attribute_syntax(span: Span, parse_error: syn::Error) -> syn::Error {
		syn::Error::new(
			span,
			format!("unexpected attribute syntax, {}", parse_error),
		)
	}

	/// Unsupported option in attribute.
	#[cfg(feature = "zeroize")]
	pub fn option_trait(span: Span, attribute: &str) -> syn::Error {
		syn::Error::new(span, format!("`{}` doesn't support this option", attribute))
	}

	/// Unsupported option in attribute.
	pub fn option(span: Span) -> syn::Error {
		syn::Error::new(span, "unknown option")
	}

	/// Unsupported options in attribute.
	pub fn options(span: Span, trait_: &str) -> syn::Error {
		syn::Error::new(span, format!("`{}` doesn't support any options", trait_))
	}

	/// Invalid syntax for an option in attribute.
	pub fn option_syntax(span: Span) -> syn::Error {
		syn::Error::new(span, "unexpected option syntax")
	}

	/// Missing sub-option for an option.
	#[cfg(feature = "zeroize")]
	pub fn option_required(span: Span, option: &str) -> syn::Error {
		syn::Error::new(span, format!("`{}` requires an option", option))
	}

	/// Duplicate option in attribute.
	pub fn option_duplicate(span: Span, option: &str) -> syn::Error {
		syn::Error::new(span, format!("duplicate `{}` option", option))
	}

	/// Unexpected constrained field skipping when configured to skip all fields
	/// anyway.
	pub fn option_skip_all(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			"unexpected constraint on `skip` when unconstrained `skip` already used",
		)
	}

	/// Duplicate trait constraint on `skip`.
	pub fn option_skip_duplicate(span: Span, trait_: &str) -> syn::Error {
		syn::Error::new(span, format!("duplicate `{}` constraint on `skip`", trait_))
	}

	/// Unsupported trait in `skip` constraint.
	pub fn option_skip_support(span: Span, trait_: &str) -> syn::Error {
		syn::Error::new(
			span,
			format!("unsupported `{}` constraint on `skip`", trait_),
		)
	}

	/// Invalid value for the `Zeroize` `crate` option.
	#[cfg(feature = "zeroize")]
	pub fn path(span: Span, parse_error: syn::Error) -> syn::Error {
		syn::Error::new(span, format!("expected path, {}", parse_error))
	}

	/// Unsupported [`Trait`](crate::Trait).
	pub fn trait_(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			format!(
				"unsupported trait, expected one of expected one of {}",
				Self::trait_list()
			),
		)
	}

	/// Invalid syntax for a [`Trait`](crate::Trait).
	pub fn trait_syntax(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			format!(
				"unsupported trait syntax, expected one of expected one of {}",
				Self::trait_list()
			),
		)
	}

	/// Invalid delimiter in `derive_where` attribute for
	/// [`Trait`](crate::Trait)s.
	pub fn derive_where_delimiter(span: Span) -> syn::Error {
		syn::Error::new(span, "expected `;` or `,")
	}

	/// Unsupported predicate type in `derive_where` attribute for where clause.
	pub fn generic(span: Span) -> syn::Error {
		syn::Error::new(span, "only type predicates are supported")
	}

	/// Invalid syntax in `derive_where` attribute for generics.
	pub fn generic_syntax(span: Span, parse_error: syn::Error) -> syn::Error {
		syn::Error::new(span, format!("expected type to bind to, {}", parse_error))
	}

	/// Unsupported default option if [`Default`] isn't implemented.
	pub fn default(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			"`default` is only supported if `Default` is being implemented",
		)
	}

	/// Missing `default` option on a variant when [`Default`] is implemented
	/// for an enum.
	pub fn default_missing(span: Span) -> syn::Error {
		syn::Error::new(
			span,
			"required `default` option on a variant if `Default` is being implemented",
		)
	}

	/// Missing `default` option on a variant when [`Default`] is implemented
	/// for an enum.
	pub fn default_duplicate(span: Span) -> syn::Error {
		syn::Error::new(span, "multiple `default` options in enum")
	}

	/// List of available [`Trait`](crate::Trait).
	fn trait_list() -> String {
		[
			"Clone",
			"Copy",
			"Debug",
			"Default",
			"Eq",
			"Hash",
			"Ord",
			"PartialEq",
			"PartialOrd",
			#[cfg(feature = "zeroize")]
			"Zeroize",
		]
		.join(", ")
	}
}
