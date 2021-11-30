//! Saves an intermediate representation of the input.

use proc_macro2::Span;
use syn::{DeriveInput, Fields, Generics, Ident, Result};

use crate::{Data, Default, DeriveWhere, Error, ItemAttr, Trait, VariantAttr};

/// Parsed input.
pub struct Input<'a> {
    /// `derive_where` attributes on the item.
    pub derive_wheres: Vec<DeriveWhere>,
    /// Item [`Span`].
    pub span: Span,
    /// Generics necessary to define for an `impl`.
    pub generics: &'a Generics,
    /// Fields or variants of this item.
    pub item: Item<'a>,
}

/// Fields or variants of an item.
#[allow(clippy::large_enum_variant)]
pub enum Item<'a> {
    /// Enum.
    Enum {
        /// [`struct@Ident`] of this enum.
        ident: &'a Ident,
        /// Variants of this enum.
        variants: Vec<Data<'a>>,
    },
    /// Struct, tuple struct or union.
    Item(Data<'a>),
}

impl<'a> Input<'a> {
    /// Create [`Input`] from `proc_macro_derive` parameter.
    pub fn parse(
        span: Span,
        DeriveInput {
            attrs,
            ident,
            generics,
            data,
            ..
        }: &'a DeriveInput,
    ) -> Result<Self> {
        // Parse `Attribute`s on item.
        let ItemAttr {
            skip_inner,
            derive_wheres,
        } = ItemAttr::from_attrs(span, data, attrs)?;

        // Extract fields and variants of this item.
        // TODO: check for empty structs, tuple structs or enums.
        let item = match &data {
            syn::Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    Data::from_struct(skip_inner, ident, fields).map(Item::Item)
                }
                Fields::Unnamed(fields) => {
                    Data::from_tuple(skip_inner, ident, fields).map(Item::Item)
                }
                Fields::Unit => Err(Error::unit_struct(span)),
            }?,
            syn::Data::Enum(data) => {
                let mut accumulated_defaults = Default::default();

                let variants = data
                    .variants
                    .iter()
                    .map(|variant| {
                        // Parse `Attribute`s on variant.
                        let VariantAttr {
                            default,
                            skip_inner,
                        } = VariantAttr::from_attrs(
                            &variant.attrs,
                            &derive_wheres,
                            &mut accumulated_defaults,
                        )?;

                        match &variant.fields {
                            Fields::Named(fields) => Data::from_struct_variant(
                                ident,
                                skip_inner,
                                default,
                                &variant.ident,
                                fields,
                            ),
                            Fields::Unnamed(fields) => Data::from_tuple_variant(
                                ident,
                                skip_inner,
                                default,
                                &variant.ident,
                                fields,
                            ),
                            Fields::Unit => {
                                Data::from_unit_variant(ident, skip_inner, default, &variant.ident)
                            }
                        }
                    })
                    .collect::<Result<Vec<Data>>>()?;

                // Make sure a variant has the `option` attribute if `Default` is being implemented.
                if !accumulated_defaults.0
                    && derive_wheres.iter().any(|derive_where| {
                        derive_where
                            .traits
                            .iter()
                            .any(|trait_| **trait_ == Trait::Default)
                    })
                {
                    return Err(Error::default_missing(span));
                }

                Item::Enum { ident, variants }
            }
            syn::Data::Union(data) => {
                Data::from_union(skip_inner, ident, &data.fields).map(Item::Item)?
            }
        };

        #[cfg(feature = "zeroize")]
        {
            if !(
                // Any generics used.
                !generics.params.is_empty()
                // Any field is skipped.
                || item.any_skip()
                // `Default` is used on an enum.
                || item.any_default(&derive_wheres)
                // `Zeroize(fqs)` is used on any field.
                || item.any_fqs()
            ) {
                return Err(Error::item(span));
            }
        }

        #[cfg(not(feature = "zeroize"))]
        {
            if !(
                // Any generics used.
                !generics.params.is_empty()
                // Any field is skipped.
                || item.any_skip()
                // `Default` is used on an enum.
                || item.any_default(&derive_wheres)
            ) {
                return Err(Error::item(span));
            }
        }

        Ok(Self {
            derive_wheres,
            span,
            generics,
            item,
        })
    }
}

impl Item<'_> {
    /// Return [`struct@Ident`] of this [`Item`].
    pub fn ident(&self) -> &Ident {
        match self {
            Item::Item(data) => data.ident,
            Item::Enum { ident, .. } => ident,
        }
    }

    /// Returns `true` if any field is skipped.
    fn any_skip(&self) -> bool {
        match self {
            Item::Item(data) => data.any_skip(),
            Item::Enum { variants, .. } => variants.iter().any(|data| data.any_skip()),
        }
    }

    /// Returns `true` if any field uses `default`.
    // MSRV: `matches!` was added in 1.42.0.
    #[allow(clippy::match_like_matches_macro)]
    fn any_default(&self, derive_wheres: &[DeriveWhere]) -> bool {
        (match self {
            Item::Enum { .. } => true,
            _ => false,
        }) && derive_wheres.iter().any(|derive_where| {
            derive_where
                .traits
                .iter()
                .any(|trait_| **trait_ == Trait::Default)
        })
    }

    /// Returns `true` if any field uses `Zeroize(fqs)`.
    #[cfg(feature = "zeroize")]
    fn any_fqs(&self) -> bool {
        use crate::util::Either;

        match self {
            Item::Item(data) => match data.fields() {
                Either::Left(fields) => fields.fields.iter().any(|field| field.attr.zeroize_fqs.0),
                Either::Right(_) => false,
            },
            Item::Enum { variants, .. } => variants.iter().any(|data| match data.fields() {
                Either::Left(fields) => fields.fields.iter().any(|field| field.attr.zeroize_fqs.0),
                Either::Right(_) => false,
            }),
        }
    }
}
