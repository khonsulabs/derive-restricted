//! [`Hash`](core::hash::Hash) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DataType, DeriveTrait, Item, SimpleType, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Hash`](core::hash::Hash).
pub struct Hash;

impl TraitImpl for Hash {
    fn as_str(&self) -> &'static str {
        "Hash"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Hash
    }

    fn supports_skip(&self) -> bool {
        true
    }

    fn build_signature(
        &self,
        _item: &Item,
        _trait_: &DeriveTrait,
        body: &TokenStream,
    ) -> TokenStream {
        quote! {
            fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                match self {
                    #body
                }
            }
        }
    }

    fn build_body(&self, trait_: &DeriveTrait, data: &Data) -> TokenStream {
        let self_pattern = data.self_pattern();
        let trait_path = trait_.path();

        // Add hashing the variant if this is an enum.
        let discriminant = if let DataType::Variant { .. } = data.type_ {
            Some(quote! { #trait_path::hash(&::core::mem::discriminant(self), __state); })
        } else {
            None
        };

        match data.simple_type() {
            SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
                let self_ident = fields.iter_self_ident(trait_);

                quote! {
                    #self_pattern => {
                        #discriminant
                        #(#trait_path::hash(#self_ident, __state);)*
                    }
                }
            }
            SimpleType::Unit(_) => {
                quote! {
                    #self_pattern => {
                        #discriminant
                    }
                }
            }
            SimpleType::Union(_) => unreachable!("unexpected trait for union"),
        }
    }
}
