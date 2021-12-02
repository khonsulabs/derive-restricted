use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn struct_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
			struct Test<T> { field: core::marker::PhatomData<T> }
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test { field: ref __field } => Test { field: ::core::clone::Clone::clone(__field) },
					}
				}
			}

			impl<T> ::core::marker::Copy for Test<T>
			{ }

			impl<T> ::core::fmt::Debug for Test<T>
			{
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test { field: ref __field } => {
							let mut __builder = ::core::fmt::Formatter::debug_struct(__f, "Test");
							::core::fmt::DebugStruct::field(&mut __builder, "field", __field);
							::core::fmt::DebugStruct::finish(&mut __builder)
						}
					}
				}
			}

			impl<T> ::core::default::Default for Test<T>
			{
				fn default() -> Self {
					Test { field: ::core::default::Default::default() }
				}
			}

			impl<T> ::core::cmp::Eq for Test<T>
			{ }

			impl<T> ::core::hash::Hash for Test<T>
			{
				fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
					match self {
						Test { field: ref __field } => { ::core::hash::Hash::hash(__field, __state); }
					}
				}
			}

			impl<T> ::core::cmp::Ord for Test<T>
			{
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					match (self, __other) {
						(Test { field: ref __field }, Test { field: ref __other_field }) =>
							match ::core::cmp::Ord::cmp(__field, __other_field) {
								::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
								__cmp => __cmp,
							},
					}
				}
			}

			impl<T> ::core::cmp::PartialEq for Test<T>
			{
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					match (self, __other) {
						(Test { field: ref __field }, Test { field: ref __other_field }) =>
							true && ::core::cmp::PartialEq::eq(__field, __other_field),
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T>
			{
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test { field: ref __field }, Test { field: ref __other_field }) =>
							match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
								__cmp => __cmp,
							},
					}
				}
			}
		},
	)
}

#[test]
fn tuple() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
			struct Test<T>(core::marker::PhatomData<T>);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
					}
				}
			}

			impl<T> ::core::marker::Copy for Test<T>
			{ }

			impl<T> ::core::fmt::Debug for Test<T>
			{
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test(ref __0) => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "Test");
							::core::fmt::DebugTuple::field(&mut __builder, __0);
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
					}
				}
			}

			impl<T> ::core::default::Default for Test<T>
			{
				fn default() -> Self {
					Test(::core::default::Default::default())
				}
			}

			impl<T> ::core::cmp::Eq for Test<T>
			{ }

			impl<T> ::core::hash::Hash for Test<T>
			{
				fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
					match self {
						Test(ref __0) => { ::core::hash::Hash::hash(__0, __state); }
					}
				}
			}

			impl<T> ::core::cmp::Ord for Test<T>
			{
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					match (self, __other) {
						(Test(ref __0), Test(ref __other_0)) =>
							match ::core::cmp::Ord::cmp(__0, __other_0) {
								::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
								__cmp => __cmp,
							},
					}
				}
			}

			impl<T> ::core::cmp::PartialEq for Test<T>
			{
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					match (self, __other) {
						(Test(ref __0), Test(ref __other_0)) =>
							true && ::core::cmp::PartialEq::eq(__0, __other_0),
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T>
			{
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test(ref __0), Test(ref __other_0)) =>
							match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
								__cmp => __cmp,
							},
					}
				}
			}
		},
	)
}

#[test]
fn enum_() -> Result<()> {
	#[cfg(feature = "nightly")]
	let discriminant = quote! {
		let __self_disc = ::core::intrinsics::discriminant_value(&self);
		let __other_disc = ::core::intrinsics::discriminant_value(&__other);
	};
	#[cfg(not(feature = "nightly"))]
	let discriminant = quote! {
		let __self_disc = ::core::mem::discriminant(self);
		let __other_disc = ::core::mem::discriminant(__other);
	};
	#[cfg(feature = "nightly")]
	let ord = quote! {
		::core::cmp::Ord::cmp(&__self_disc, &__other_disc)
	};
	#[cfg(not(any(feature = "nightly", feature = "safe")))]
	let ord = quote! {
		::core::cmp::Ord::cmp(
			&unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
			&unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
		)
	};

	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let ord = quote! {
		match self {
			Test::A { field: ref __field } =>
				match __other {
					Test::B { } => ::core::cmp::Ordering::Less,
					Test::C(ref __other_0) => ::core::cmp::Ordering::Less,
					Test::D() => ::core::cmp::Ordering::Less,
					Test::E => ::core::cmp::Ordering::Less,
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::B { } =>
				match __other {
					Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
					Test::C(ref __other_0) => ::core::cmp::Ordering::Less,
					Test::D() => ::core::cmp::Ordering::Less,
					Test::E => ::core::cmp::Ordering::Less,
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::C(ref __0) =>
				match __other {
					Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
					Test::B { } => ::core::cmp::Ordering::Greater,
					Test::D() => ::core::cmp::Ordering::Less,
					Test::E => ::core::cmp::Ordering::Less,
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::D() =>
				match __other {
					Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
					Test::B { } => ::core::cmp::Ordering::Greater,
					Test::C(ref __other_0) => ::core::cmp::Ordering::Greater,
					Test::E => ::core::cmp::Ordering::Less,
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::E =>
				match __other {
					Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
					Test::B { } => ::core::cmp::Ordering::Greater,
					Test::C(ref __other_0) => ::core::cmp::Ordering::Greater,
					Test::D() => ::core::cmp::Ordering::Greater,
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
		}
	};
	#[cfg(feature = "nightly")]
	let partial_ord = quote! {
		::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
	};
	#[cfg(not(any(feature = "nightly", feature = "safe")))]
	let partial_ord = quote! {
		::core::cmp::PartialOrd::partial_cmp(
			&unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
			&unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
		)
	};
	#[cfg(all(not(feature = "nightly"), feature = "safe"))]
	let partial_ord = quote! {
		match self {
			Test::A { field: ref __field } =>
				match __other {
					Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::B { } =>
				match __other {
					Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::C(ref __0) =>
				match __other {
					Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::D() =>
				match __other {
					Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
			Test::E =>
				match __other {
					Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
					_ => ::core::unreachable!("comparing variants yielded unexpected results"),
				},
		}
	};

	test_derive(
		quote! {
			#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
			enum Test<T> {
				A { field: core::marker::PhatomData<T>},
				B { },
				C(core::marker::PhatomData<T>),
				D(),
				#[derive_where(default)]
				E,
			}
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			{
				#[inline]
				fn clone(&self) -> Self {
					match self {
						Test::A { field: ref __field } => Test::A { field: ::core::clone::Clone::clone(__field) },
						Test::B { } => Test::B { },
						Test::C(ref __0) => Test::C(::core::clone::Clone::clone(__0)),
						Test::D() => Test::D(),
						Test::E => Test::E,
					}
				}
			}

			impl<T> ::core::marker::Copy for Test<T>
			{ }

			impl<T> ::core::fmt::Debug for Test<T>
			{
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test::A { field: ref __field } => {
							let mut __builder = ::core::fmt::Formatter::debug_struct(__f, "A");
							::core::fmt::DebugStruct::field(&mut __builder, "field", __field);
							::core::fmt::DebugStruct::finish(&mut __builder)
						}
						Test::B { } => {
							let mut __builder = ::core::fmt::Formatter::debug_struct(__f, "B");
							::core::fmt::DebugStruct::finish(&mut __builder)
						}
						Test::C(ref __0) => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "C");
							::core::fmt::DebugTuple::field(&mut __builder, __0);
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
						Test::D() => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "D");
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
						Test::E => ::core::fmt::Formatter::write_str(__f, "E"),
					}
				}
			}

			impl<T> ::core::default::Default for Test<T>
			{
				fn default() -> Self {
					Test::E
				}
			}

			impl<T> ::core::cmp::Eq for Test<T>
			{ }

			impl<T> ::core::hash::Hash for Test<T>
			{
				fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
					match self {
						Test::A { field: ref __field } => {
							::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
							::core::hash::Hash::hash(__field, __state);
						}
						Test::B { } => {
							::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
						}
						Test::C(ref __0) => {
							::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
							::core::hash::Hash::hash(__0, __state);
						}
						Test::D() => {
							::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
						}
						Test::E => {
							::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
						}
					}
				}
			}

			impl<T> ::core::cmp::Ord for Test<T>
			{
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
								match ::core::cmp::Ord::cmp(__field, __other_field) {
									::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
									__cmp => __cmp,
								},
							(Test::C(ref __0), Test::C(ref __other_0)) =>
								match ::core::cmp::Ord::cmp(__0, __other_0) {
									::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
									__cmp => __cmp,
								},
							_ => ::core::cmp::Ordering::Equal,
						}
					} else {
						#ord
					}
				}
			}

			impl<T> ::core::cmp::PartialEq for Test<T>
			{
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
						match (self, __other) {
							(Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
								true && ::core::cmp::PartialEq::eq(__field, __other_field),
							(Test::C(ref __0), Test::C(ref __other_0)) =>
								true && ::core::cmp::PartialEq::eq(__0, __other_0),
							_ => true,
						}
					} else {
						false
					}
				}
			}

			impl<T> ::core::cmp::PartialOrd for Test<T>
			{
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					#discriminant

					if __self_disc == __other_disc {
						match (self, __other) {
							(Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
								match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
							(Test::C(ref __0), Test::C(ref __other_0)) =>
								match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
							_ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
						}
					} else {
						#partial_ord
					}
				}
			}
		},
	)
}

#[test]
fn union_() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone, Copy)]
			union Test<T> {
				a: core::marker::PhantomData<T>,
				b: u8,
			}
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			{
				#[inline]
				fn clone(&self) -> Self {
					struct __AssertCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
					let _: __AssertCopy<Self>;
					*self
				}
			}

			impl<T> ::core::marker::Copy for Test<T>
			{ }
		},
	)
}
