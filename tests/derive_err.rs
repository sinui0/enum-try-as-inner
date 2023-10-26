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
#[derive(Debug, PartialEq, EnumTryAsInner)]
#[derive_err(Debug, PartialEq)]
enum DeriveErr {
    Zero,
    One(u32),
    Two(u32, i32),
}

#[test]
fn test_derive_err() {
    let one = DeriveErr::One(1);
    let two = DeriveErr::Two(1, 2);

    assert_ne!(
        one.try_into_two().unwrap_err(),
        two.try_into_one().unwrap_err()
    );
}
