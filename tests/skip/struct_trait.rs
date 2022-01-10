use std::cmp::Ordering;

use derive_where::derive_where;

use crate::util::{
	self, AssertDebug, AssertHash, AssertOrd, AssertPartialEq, AssertPartialOrd, Wrapper,
};

#[test]
fn debug() {
	#[derive_where(Debug)]
	#[derive_where(skip_inner(Debug))]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());

	let _ = AssertDebug(&test_1);

	assert_eq!(format!("{:?}", test_1), "Test");
}

#[test]
fn hash() {
	#[derive_where(Hash)]
	#[derive_where(skip_inner(Hash))]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_ge = Test(43.into());

	let _ = AssertHash(&test_1);

	util::hash_eq(&test_1, &test_2);
	util::hash_eq(test_1, test_ge);
}

#[test]
fn ord() {
	#[derive_where(Eq, Ord, PartialEq, PartialOrd)]
	#[derive_where(skip_inner(Ord))]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(43.into());
	let test_ge = Test(43.into());

	let _ = AssertOrd(&test_1);

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_ge), Ordering::Equal);
}

#[test]
fn partial_eq() {
	#[derive_where(PartialEq)]
	struct Test<T>(
		#[derive_where(skip(PartialEq))]
		#[allow(dead_code)]
		Wrapper<T>,
	);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_ge = Test(43.into());

	let _ = AssertPartialEq(&test_1);

	assert!(test_1 == test_2);
	assert!(test_1 == test_ge);
}

#[test]
fn partial_ord() {
	#[derive_where(PartialEq, PartialOrd)]
	#[derive_where(skip_inner(PartialOrd))]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(41.into());
	let test_ge = Test(43.into());

	let _ = AssertPartialOrd(&test_1);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_le), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_ge), Some(Ordering::Equal));
}

#[test]
fn all() {
	#[derive_where(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
	#[derive_where(skip_inner(Debug, Hash, Ord, PartialEq, PartialOrd))]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(41.into());
	let test_ge = Test(43.into());

	let _ = AssertDebug(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	assert_eq!(format!("{:?}", test_1), "Test");

	util::hash_eq(&test_1, &test_2);
	util::hash_eq(&test_1, &test_ge);

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_ge), Ordering::Equal);

	assert!(test_1 == test_2);
	assert!(test_1 == test_ge);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_le), Some(Ordering::Equal));
	assert_eq!(test_1.partial_cmp(&test_ge), Some(Ordering::Equal));
}
