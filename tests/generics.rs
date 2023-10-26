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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, EnumTryAsInner)]
#[derive_err(Debug)]
enum WithGenerics<T: Clone + Copy> {
    A(T),
    B(T),
}

#[test]
fn with_generics() {
    let mut with_generics = WithGenerics::A(100);

    assert!(with_generics.is_a());
    assert!(!with_generics.is_b());

    assert!(with_generics.try_as_a().is_ok());
    assert!(with_generics.try_as_b().is_err());

    assert_eq!(with_generics.try_into_a().unwrap(), 100);
    assert_eq!(*with_generics.try_as_a().unwrap(), 100);
    assert_eq!(*with_generics.try_as_a_mut().unwrap(), 100);

    assert!(with_generics.try_into_b().is_err());
    assert!(with_generics.try_as_b().is_err());
    assert!(with_generics.try_as_b_mut().is_err());
}
