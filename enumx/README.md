# Purpose

This project provides ad-hoc enum types, and traits for user-defined enum exchange.

An `enum` can be exchanged into another `enum` if all variants in the formmer are in the latter.

# Type and API Naming

Ad-hoc enums composed of x variants are named as `Enum1`, `Enum2`, ... etc.

Variants are named as `_0`, `_1`, ... etc.

User-defined `enum`s are normal Rust `enum`s using any readable enum names or variant names, with `Exchange` derived using `#[derive(Exchang)]`.

An enum can be constructed `from_variant()`, while a variant can be converted `into_enum()`.

An ad-hoc enum can be converted from another one using `from_enumx()`, while one can be coverted into another one using `into_enumx()`.

**A user-defined `Exchange` enum can be converted `From`/`Into` its associated `Exchange::EnumX` type, which is an ad-hoc enum.**

A user-defined `Exchange` enum can be `exchange_from()` another user-defined `Exchange` enum, while one can be `exchange_into()` another.

# Examples

See [test code](https://github.com/oooutlk/enumx/blob/master/enumx/src/lib.rs#L512).

# Limitation

Current version supports up to 16 variants by default, and extended to 32 variants with `enum17_enum32` feature.
In otherwords, `Enum1`..=`Enum16` is available but `Enum17` and the succeeding enums are not, by default.
With `enum17_enum32` feature, `Enum32` is available but `Enum33` and the succeeding enums are not.
But notice that it may cost several minutes to compile this crate with `enum17_enum32` feature.

# License

Licensed under MIT.

# Acknowledgement

This version is inspired by [`frunk_core::coproduct`](https://docs.rs/frunk_core/0.2.2/frunk_core/coproduct/index.html)
