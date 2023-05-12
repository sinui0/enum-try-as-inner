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

use enum_as_inner::EnumAsInner;

mod name_collisions {
    #![allow(dead_code, missing_copy_implementations, missing_docs)]
    struct Option;
    struct Some;
    struct None;
    struct Result;
    struct Ok;
    struct Err;
}
#[allow(unused_imports)]
use name_collisions::*;

#[derive(Debug, EnumAsInner)]
enum ManyVariants {
    One(u32),
    Two(u32, i32),
    Three(bool, u32, i64),
}

#[test]
fn test_one_unnamed() {
    let mut many = ManyVariants::One(1);

    assert!(many.is_one());
    assert!(!many.is_two());
    assert!(!many.is_three());

    assert!(many.as_one().is_some());
    assert!(many.as_two().is_none());
    assert!(many.as_three().is_none());

    assert!(many.as_one_mut().is_some());
    assert!(many.as_two_mut().is_none());
    assert!(many.as_three_mut().is_none());

    assert_eq!(*many.as_one().unwrap(), 1_u32);
    assert_eq!(*many.as_one_mut().unwrap(), 1_u32);
    assert_eq!(many.into_one().unwrap(), 1_u32);
}

#[test]
fn test_two_unnamed() {
    let mut many = ManyVariants::Two(1, 2);

    assert!(!many.is_one());
    assert!(many.is_two());
    assert!(!many.is_three());

    assert!(many.as_one().is_none());
    assert!(many.as_two().is_some());
    assert!(many.as_three().is_none());

    assert!(many.as_one_mut().is_none());
    assert!(many.as_two_mut().is_some());
    assert!(many.as_three_mut().is_none());

    assert_eq!(many.as_two().unwrap(), (&1_u32, &2_i32));
    assert_eq!(many.as_two_mut().unwrap(), (&mut 1_u32, &mut 2_i32));
    assert_eq!(many.into_two().unwrap(), (1_u32, 2_i32));
}

#[test]
fn test_three_unnamed() {
    let mut many = ManyVariants::Three(true, 1, 2);

    assert!(!many.is_one());
    assert!(!many.is_two());
    assert!(many.is_three());

    assert!(many.as_one().is_none());
    assert!(many.as_two().is_none());
    assert!(many.as_three().is_some());

    assert!(many.as_one_mut().is_none());
    assert!(many.as_two_mut().is_none());
    assert!(many.as_three_mut().is_some());

    assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
    assert_eq!(
        many.as_three_mut().unwrap(),
        (&mut true, &mut 1_u32, &mut 2_i64)
    );
    assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
}
