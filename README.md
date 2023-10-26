# enum-try-as-inner

A deriving proc-macro for generating functions to automatically give access to the inner members of enum.

This is a fork of [`enum-as-inner`](https://crates.io/enum-as-inner), this crate focuses on returning `Result<Variant, EnumError>` instead.

## Basic unnamed field case

The basic case is meant for single item enums, like:

```rust
use enum_try_as_inner::EnumTryAsInner;

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum OneEnum {
    One(u32),
}

let one = OneEnum::One(1);

assert_eq!(*one.try_as_one().unwrap(), 1);
assert_eq!(one.try_into_one().unwrap(), 1);
```

where the result is either a reference for inner items or a tuple containing the inner items.

## Unit case

This will return true if enum's variant matches the expected type

```rust
use enum_try_as_inner::EnumTryAsInner;

#[derive(EnumTryAsInner)]
enum UnitVariants {
    Zero,
    One,
    Two,
}

let unit = UnitVariants::Two;

assert!(unit.is_two());
```

## Multiple, unnamed field case

This will return a tuple of the inner types:

```rust
use enum_try_as_inner::EnumTryAsInner;

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum ManyVariants {
    One(u32),
    Two(u32, i32),
    Three(bool, u32, i64),
}

let many = ManyVariants::Three(true, 1, 2);

assert!(many.is_three());
assert_eq!(many.try_as_three().unwrap(), (&true, &1_u32, &2_i64));
assert_eq!(many.try_into_three().unwrap(), (true, 1_u32, 2_i64));
```

## Multiple, named field case

This will return a tuple of the inner types, like the unnamed option:

```rust
use enum_try_as_inner::EnumTryAsInner;

#[derive(Debug, EnumTryAsInner)]
#[derive_err(Debug)]
enum ManyVariants {
    One { one: u32 },
    Two { one: u32, two: i32 },
    Three { one: bool, two: u32, three: i64 },
}

let many = ManyVariants::Three { one: true, two: 1, three: 2 };

assert!(many.is_three());
assert_eq!(many.try_as_three().unwrap(), (&true, &1_u32, &2_i64));
assert_eq!(many.try_into_three().unwrap(), (true, 1_u32, 2_i64));
```

# Error

This macro generates an error type for each enum which contains information about which variant was expected,
which variant was found at runtime, and if using the `try_into_*` functions, the actual value.

## Error derives

By default, the generated error does not implement any traits, including `std::error::Error`.

Derive macros can be forwarded to the error implementation using the `derive_err` attribute.

If the `Debug` derive is provided, an implementation of `Display` and `Error` will be automatically provided.

If you would like to implement your own `Display` format, you will need to also implement `Debug` and `Error` yourself.

```rust
use enum_try_as_inner::EnumTryAsInner;

#[derive(Debug, Clone, PartialEq, EnumTryAsInner)]
#[derive_err(Debug, Clone, PartialEq)]
enum Clonable {
    One(u32),
    Two(u32, i32),
}

let one = Clonable::One(1);
let err = one.try_into_two().unwrap_err();
let cloned = err.clone();

assert_eq!(err, cloned);

println!("expected {}, but got {}", err.expected(), err.actual());
println!("actual value: {:?}", err.into_value().unwrap());
```