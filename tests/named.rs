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
enum ManyVariants {
    One { one: u32 },
    Two { one: u32, two: i32 },
    Three { one: bool, two: u32, three: i64 },
}

#[test]
fn test_one_named() {
    let mut many = ManyVariants::One { one: 1 };

    assert!(many.is_one());
    assert!(!many.is_two());
    assert!(!many.is_three());

    assert!(many.try_as_one().is_ok());
    assert!(many.try_as_two().is_err());
    assert!(many.try_as_three().is_err());

    assert!(many.try_as_one_mut().is_ok());
    assert!(many.try_as_two_mut().is_err());
    assert!(many.try_as_three_mut().is_err());

    assert_eq!(*many.try_as_one().unwrap(), 1_u32);
    assert_eq!(*many.try_as_one_mut().unwrap(), 1_u32);
}

#[test]
fn test_two_named() {
    let mut many = ManyVariants::Two { one: 1, two: 2 };

    assert!(!many.is_one());
    assert!(many.is_two());
    assert!(!many.is_three());
    assert!(many.try_as_one().is_err());
    assert!(many.try_as_two().is_ok());
    assert!(many.try_as_three().is_err());
    assert!(many.try_as_one_mut().is_err());
    assert!(many.try_as_two_mut().is_ok());
    assert!(many.try_as_three_mut().is_err());

    assert_eq!(many.try_as_two().unwrap(), (&1_u32, &2_i32));
    assert_eq!(many.try_as_two_mut().unwrap(), (&mut 1_u32, &mut 2_i32));
    assert_eq!(many.try_into_two().unwrap(), (1_u32, 2_i32));
}

#[test]
fn test_three_named() {
    let mut many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    assert!(!many.is_one());
    assert!(!many.is_two());
    assert!(many.is_three());
    assert!(many.try_as_one().is_err());
    assert!(many.try_as_two().is_err());
    assert!(many.try_as_three().is_ok());
    assert!(many.try_as_one_mut().is_err());
    assert!(many.try_as_two_mut().is_err());
    assert!(many.try_as_three_mut().is_ok());

    assert_eq!(many.try_as_three().unwrap(), (&true, &1_u32, &2_i64));
    assert_eq!(
        many.try_as_three_mut().unwrap(),
        (&mut true, &mut 1_u32, &mut 2_i64)
    );
    assert_eq!(many.try_into_three().unwrap(), (true, 1_u32, 2_i64));
}
