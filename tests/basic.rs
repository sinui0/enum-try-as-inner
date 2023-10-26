#![warn(
    clippy::default_trait_access,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::unimplemented,
    clippy::use_self,
    missing_copy_implementations,
    missing_docs,
    non_snake_case,
    non_upper_case_globals,
    rust_2018_idioms,
    unreachable_pub
)]

use enum_try_as_inner::EnumTryAsInner;

pub mod name_collisions {
    #![allow(dead_code, missing_copy_implementations, missing_docs)]
    pub struct Option;
    pub struct Some;
    pub struct None;
    pub struct Result;
    pub struct Ok;
    pub struct Err;
}
#[allow(unused_imports)]
use name_collisions::*;

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum EmptyTest {}

#[test]
fn test_empty() {
    let empty = std::option::Option::None::<EmptyTest>;

    assert!(empty.is_none());
}

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum EmptyParendsTest {
    Empty(),
}

#[test]
fn test_empty_parends() {
    let mut empty = EmptyParendsTest::Empty();

    assert!(empty.is_empty());

    empty
        .try_as_empty()
        .expect("should have been something and a unit");
    empty
        .try_as_empty_mut()
        .expect("should have been something and a unit");

    empty
        .try_into_empty()
        .expect("should have been something and a unit");
}

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum OneTest {
    One(u32),
}

#[test]
fn test_one() {
    let mut empty = OneTest::One(1);

    assert!(empty.is_one());
    assert_eq!(*empty.try_as_one().unwrap(), 1);
    assert_eq!(*empty.try_as_one_mut().unwrap(), 1);
    assert_eq!(empty.try_into_one().unwrap(), 1);
}

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum MultiTest {
    Multi(u32, u32),
}

#[test]
fn test_multi() {
    let mut multi = MultiTest::Multi(1, 1);

    assert!(multi.is_multi());
    assert_eq!(multi.try_as_multi().unwrap(), (&1_u32, &1_u32));
    assert_eq!(multi.try_as_multi_mut().unwrap(), (&mut 1_u32, &mut 1_u32));
    assert_eq!(multi.try_into_multi().unwrap(), (1_u32, 1_u32));
}
