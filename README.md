# from_int &emsp; [![Latest Version]][crates.io]

[crates.io]: https://crates.io/crates/from_int

## Motivation

This crate provides an easy way to convert a plain integer into an enum type, something that rust can currently do natively in the opposite direction.

## Usage

Usage of `from_int` is extremely simple. Just add it as a dependency to your crate, then:

```rust
extern crate from_int; // contains the trait
#[macro_use] extern crate from_int_derive; // contains the macro

use from_int::FromInt;

#[derive(FromInt, Debug, PartialEq)]
enum TestEnum {
    VariantOne = 1,
    VariantTwo = 2,
    VariantThree = 528,
    VariantX = 999
}

assert_eq!(TestEnum::VariantOne, TestEnum::from_int(1));
assert_eq!(TestEnum::VariantTwo, TestEnum::from_int(2));
assert_eq!(TestEnum::VariantThree, TestEnum::from_int(528));
assert_eq!(TestEnum::VariantX, TestEnum::from_int(999));

// This would panic:
assert_eq!(TestEnum::VariantOne, TestEnum::from_int(123));
```
