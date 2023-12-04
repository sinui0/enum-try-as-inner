# enum-try-as-inner

A deriving proc-macro for generating functions to automatically give access to the inner members of enum.

This is a fork of [`enum-as-inner`](https://crates.io/crates/enum-as-inner), this crate focuses on returning `Result<Variant, EnumError>` instead.

# Accessors

The macro automatically generates functions for accessing fields for every variant.

```rust
use enum_try_as_inner::EnumTryAsInner;

#[derive(EnumTryAsInner)]
enum MyEnum {
    Foo(u32),
    Bar(String)
}
```

Expanded it looks something like this (omitting some details):

```rust, ignore
enum MyEnum {
    Foo(u32),
    Bar(String)
}

impl MyEnum {
    /// Returns true if this is a `MyEnum::Foo`, otherwise false
    pub fn is_foo(&self) -> bool {
        match self {
            Self::Foo(_) => true,
            _ => false,
        }
    }

    /// Returns references to the inner fields if this is a `MyEnum::Foo`,
    /// otherwise an `MyEnumError`
    pub fn try_as_foo(&self) -> Result<&u32, MyEnumError> {
        match self {
            Self::Foo(inner) => Ok((inner)),
            _ => {
                Err(
                    MyEnumError::new(
                        "Foo",
                        self.variant_name(),
                        None,
                    ),
                )
            }
        }
    }

    /// Returns mutable references to the inner fields if this is a `MyEnum::Foo`, 
    /// otherwise an `MyEnumError`
    pub fn try_as_foo_mut(&mut self) -> Result<&mut u32, MyEnumError> {
        match self {
            Self::Foo(inner) => Ok((inner)),
            _ => {
                Err(
                    MyEnumError::new(
                        "Foo",
                        self.variant_name(),
                        None,
                    ),
                )
            }
        }
    }

    /// Returns the inner fields if this is a `MyEnum::Foo`, otherwise returns
    /// back the enum in the `Err` case of the result
    #[inline]
    pub fn try_into_foo(self) -> Result<u32, MyEnumError> {
        match self {
            Self::Foo(inner) => Ok((inner)),
            _ => {
                Err(
                    MyEnumError::new(
                        "Foo",
                        self.variant_name(),
                        Some(self),
                    ),
                )
            }
        }
    }
    
    /// Returns the name of the variant.
    fn variant_name(&self) -> &'static str {
        match self {
            Self::Foo(..) => "Foo",
            Self::Bar(..) => "Bar",
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }

    // .. Omitted methods for `Bar` variant
}
```

# Error

The macro generates an error type which provides information about which variant was expected,
which variant was found at runtime, and if using the `try_into_*` functions, the actual value.

Expanded it looks like this (omitting some details):

```rust
enum MyEnum {
    Foo(u32),
    Bar(String)
}

struct MyEnumError {
    expected: &'static str,
    actual: &'static str,
    value: Option<MyEnum>,
}

impl MyEnumError {
    /// Creates a new error indicating the expected variant and the actual variant.
    fn new(
        expected: &'static str,
        actual: &'static str,
        value: Option<MyEnum>,
    ) -> Self {
        Self { expected, actual, value }
    }

    /// Returns the name of the variant that was expected.
    pub fn expected(&self) -> &'static str {
        self.expected
    }

    /// Returns the name of the actual variant.
    pub fn actual(&self) -> &'static str {
        self.actual
    }

    /// Returns a reference to the actual value, if present.
    pub fn value(&self) -> Option<&MyEnum> {
        self.value.as_ref()
    }

    /// Returns the actual value, if present.
    pub fn into_value(self) -> Option<MyEnum> {
        self.value
    }
}
```

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

# Examples

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
assert!(unit.try_as_two().is_ok());
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

## State Machine

This example demonstrates how to construct a state machine for a traffic light with the help of the derive macro.

```rust
// This is similar to the typestate pattern, but here the state is wrapped in an enum so that
// the internal state is not part of the type signature of `TrafficLight`.
#[derive(Debug, Default)]
pub struct TrafficLight {
    state: state::State,
}

#[derive(Debug)]
pub enum TrafficLightError {
    InvalidState(String),
    NotEnoughCarsPassed(usize),
    NotEnoughCarsWaiting(usize),
}

// This lets us propagate state errors easily using the `?` operator.
impl From<state::StateError> for TrafficLightError {
    fn from(err: state::StateError) -> Self {
        TrafficLightError::InvalidState(err.to_string())
    }
}

// Every state transition/update is guarded by a runtime check
impl TrafficLight {
    pub fn turn_red(&mut self) -> Result<(), TrafficLightError> {
        // Only a yellow light can turn red.
        self.state.try_as_yellow()?;

        self.state = state::State::Red(state::Red::default());

        Ok(())
    }

    pub fn turn_yellow(&mut self) -> Result<(), TrafficLightError> {
        // Only a green light can turn yellow.
        // This serves as both a runtime state-guard as well as an accessor
        // for the state variables.
        let &state::Green { cars_passed } = self.state.try_as_green()?;

        if cars_passed > 10 {
            self.state = state::State::Yellow;
        } else {
            return Err(TrafficLightError::NotEnoughCarsPassed(cars_passed));
        }

        Ok(())
    }

    pub fn turn_green(&mut self) -> Result<(), TrafficLightError> {
        // Only a red light can turn green.
        let &state::Red { cars_waiting } = self.state.try_as_red()?;

        if cars_waiting > 0 {
            self.state = state::State::Green(state::Green::default());
        } else {
            return Err(TrafficLightError::NotEnoughCarsWaiting(cars_waiting));
        }

        Ok(())
    }

    pub fn record_passed_car(&mut self) -> Result<(), TrafficLightError> {
        // Passing cars are only recorded when the light is green.
        let state::Green { cars_passed } = self.state.try_as_green_mut()?;

        *cars_passed += 1;

        Ok(())
    }

    pub fn record_waiting_car(&mut self) -> Result<(), TrafficLightError> {
        // Waiting cars are only recorded when the light is red.
        let state::Red { cars_waiting } = self.state.try_as_red_mut()?;

        *cars_waiting += 1;

        Ok(())
    }
}

mod state {
    use enum_try_as_inner::EnumTryAsInner;

    #[derive(Debug, EnumTryAsInner)]
    #[derive_err(Debug)]
    pub enum State {
        Red(Red),
        Yellow,
        Green(Green),
    }

    impl Default for State {
        fn default() -> Self {
            State::Red(Red::default())
        }
    }

    #[derive(Debug, Default)]
    pub struct Red {
        pub cars_waiting: usize,
    }

    #[derive(Debug, Default)]
    pub struct Green {
        pub cars_passed: usize,
    }
}

let mut light = TrafficLight::default();

light.record_waiting_car().unwrap();

// Can not transition from red to yellow.
assert!(light.turn_yellow().is_err());

light.turn_green().unwrap();

for _ in 0..=9 {
    light.record_passed_car().unwrap();
}

// At least 10 cars must have passed to turn yellow.
assert!(light.turn_yellow().is_err());

light.record_passed_car().unwrap();

light.turn_yellow().unwrap();
light.turn_red().unwrap();
```
