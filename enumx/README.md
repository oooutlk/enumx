# Summary

Add macros and traits, for minimal support of structural enums.

The new constructs are:

* Four traits `FromVariant`, `IntoEnum`, `ExchangeFrom`, `ExchangeInto`.

* A new proc-macro derive `Exchange` applicable for `enum`s, generating type
convertion `impl`s defined in these four traits.

* A declarative macro `Enum!` accessing to predefined `Exchange`able enum.

In client code, write the following

```rust
use enumx::Enum;
use enumx::prelude::*;
```

to use the constructs mentioned above.

# Motivation

An enum can be utilized to express a set of finite, known elements. Enums
composed of variants that do not refer to domain knowledge are considered as
structural enums. They serve as a mechanism of code organisations.

Full-fledged structural enums provide mechanisms not only for gathering values
of variants into enums, but also for gathering variants of an structural enum,
into another one.

Such a mechanism is not available in Rust, requiring Rustaceans to implement it
themselves when needed.

While it is easy to write maros for "variants => enum" gathering, a general
"enum => enum" convertion is non-trival to implement.

These facts have caused some unfavorable results:

- Programmers are developing such non-trival equivalents in specific domains. A
  notable example is ["error-chain"](https://crates.io/crates/error-chain). It
  had reinvented the wheel for some kind of structural enum, aka `ErrorKind`.

- Tempting to use `trait` object instead, in cases which structural `enum` is
  most suitable for.

This library addresses these issues by inroducing a minimum support of
structural enums, aka exchangeable enums.

# Overview

An enum with `#[derive(Exchange)]` is considered as an exchangeable enum.

```rust
#[derive(Exchange)]
enum Info {
    Code(i32),
    Text(&'static str),
}
```

An exchangeable enum can be constructed from one of its variants:

```rust
let info: Info = 42.into_enum();
let info = Info::from_variant(42);
```

An exchangeable enum can be exchanged from/into another exchangeable one, as
long as one has all the variant types appearing in the other one's definition.

```rust
#[derive(Exchange)]
enum Data {
    Num(i32),
    Text(&'static str),
    Flag(bool),
}

let info: Info = 42.into_enum();
let data: Data = info.exchange_into();

let info = Info::from_variant(42);
let data = Data::exchange_from(info);
```

## Enum methods

By now, we call `from_variant()`, `into_enum()`, `exchange_from()`,
`exchange_into()`as enum exchange methods.

## Syntax limits of exchangeable enum

All variants must be in the form of "newtype".

```rust
#[derive(Exchange)]
enum Info {
    Text(String),  // ok, it is newtype
    Code(i32,u32), // compile error
}
```

should cause an error:

```text
1926 | Code(i32,u32),
     | ^^^^^^^^^^^^^ all variants of an exchangeable enum must be newtype
```

## Predefined exchangeable enums

The `Enum!( T0, T1, .. )` macro defines an predefined exchangeable enum composed
of variant types T0, T1, .. etc.

```rust
let info = <Enum!(i32,&'static str)>::from_variant(42);
```

is essentially equvalent to the following:

```rust
let info = __Enum2::from_variant(42);
```

while `__Enum2` is predefined but **may not exposed to programmers**:

```rust
#[derive(Exchange)]
enum __Enum2 {
    _0(i32),
    _1(&'static str),
}
```

Two `Enum!()`s with identical variant type list are identical types.

`<Enum!( T0, T1, .. )>::_0` for the first variant in pattern matching, and so
forth.

```rust
match info {
    <Enum!(i32,&'static str)>::_0(_i) => (),
    <Enum!(i32,&'static str)>::_1(_s) => (),
}
```

Predefined enums are also `Exchange`able enums. They are considered as unnamed
exchangeable enums, while user-defined ones are called named exchangeable enums.

An unnamed exchangeable enum is suitable in such usescases that an structural
enum does not worth a name, while a named exchangeable enum serves for those do
worth naming. 

From now on, we will prefer using predefined enums in examples, since they are
superior as notations, comparing to user-defined enums.

## Convertion rules

The following 2 rules are considered as the minimum support of structural enum:

1. variant <=> exchangeable enum

  `from_variant()`/`into_enum()` serve for it.

2. exchangeable enum => exchangeable enum composed of equal or more variants

  `exchange_from()`/`exchange_into()` serve for it.

The following rules are considered perculiar to exchangeable enums, which
distinguish them from "union types" in Typed Racket.

1. An exchangeable enum composed of duplicated variant types is a valid enum,
but it is nonsense because acual uses of its enum exchange methods will cause
compile errors.

```text
9 | let a = <Enum!(i32,i32)>::_0( 3722 );
  |         ^^^^^^^^^^^^^^^^ variants of an exchangeable enum must be unique.
```

2. No automatic flattening

  For example, `Enum!(A,Enum!(B,C))` can not be converted to `Enum!(A,B,C)` via
  enum exchange methods. Further more, making these two equal types will need
  changes in type systems, which is not possible for a proc-macro derive.

# Detailed design

The definition of enum exchange traits are as following:

```rust
pub trait FromVariant<Variant,Index,Kind> {
    fn from_variant(v: Variant) -> Self;
}

pub trait IntoEnum<Enum,Index,Kind> {
    fn into_enum(self) -> Enum;
}

pub trait ExchangeFrom<Src,Indices,Kind> {
    fn exchange_from(src: Src) -> Self;
}

pub trait ExchangeInto<Dest,Indices,Kind> {
    fn exchange_into(self) -> Dest;
}
```

Notice that all traits have phantom types `Index`/`Indices` and `Kind` in
their generics to hold positional information to help compiler accomplishing
type inferences and avoid overlapping `impl`s.

## Distinguish from `std::convert`

Since standard `From`/`Into` does not have such phantom types, it is not
feasible to implement enum exchange methods in `From`/`Into`. Trying to
implement in `From` will cause compile error because we need to impl multiple
`From<Variant>` but the generic `Variant` type in different `impl`s could be of
the same actual type, resulting in overlapping `impl`s.

## Distinguish between `Index` and `Indices`

Consider the convertion from `Enum!(A,B)` to `Enum!(A,B,Enum!(A,B))`.

Since these two enum types are not equal due to lacking of flattening , there
are two possible ways for this convertion:

1. making the former as the third variant of the latter.

2. matching the former to get an `A` or `B`, then making it as the first or
  second variant of the latter.

This is the root cause we distinguish between `FromVariant` and `ExchangeFrom`.

# Drawbacks

- Abusing structural enums in cases that they are not suitable for.

- Distinguish between `FromVariant` and `ExchangeFrom` will cause extra
annotations, which may be unnecessary in some usecases.

- As a library mimicing structrual enums, enumx supports a limited count of
  variants. The default is 16, and can be twisted via environment variable
  `ENUMX_MAX_VARIANTS`.

# Rationale and alternatives

## Misuse: always defining types and their convertions explicitly

People misusing this believe it is in such a case that:

1. All types, including structural enums, should have readable names, either
  handwritten or generated from user-defined macro.

2. Convertions between types should be defined explictly, either handwritten or
  generated from user-defined macro. They work in the way analogy to `friend`
  keyword in C++.

People using exchangeable enums believe it is in such a case that:

1. An structural enum may or may not worth a readable name.

2. Convertions between structural enums should be derived from `Exchange`, which
  works in the way analogy to `pub` keyword.

Always naming an structural enum and generating convertions for it will mislead
readers considering it being deliberate unless they finish reading all the code.

Take two analogies:

1. What if we are not allowed to use closures and local defined functions, but
have to use structural structs and functions defined far away from their only
invokings, for mimicing closures?

2. What if we are not allowed to use `pub`/`pub(crate)`/`pub(super)`, but have
to explicitly authorize all the possible `friend`s of a certain field?

## Misuse: trait objects

People misusing trait objects believe it is in such a case that:

1. All types, including structural enums, should implement some trait and may be
  categorized to a hierarchy of traits.

2. All types should be erased, and accesses should be done via public interface.

It is reasonable to use trait objects to express a set of unpredicable elements
having a set of related methods in a trait. Using it to express a set of
predicable elements having unrelated functions is possible, but is a concept
mismatch, which may potentially cause pitfalls:

1. Useless methods in trait.

2. Unnecessary boxing and `'static` lifetime bounds.

3. non-straightforward down-casting.

These are all non-issues for structural enums to express a set of predicable
elements having unrelated functions.

# Prior art

Concepts similar with structural enum exist in other language. One example is
union types in Typed Racket. However, it supports more powerful type inferences
such as `Enum!(A)` => `A`, `Enum!(A,A)` => `Enum!(A)`, `Enum!(A,Enum!(B,C))` =>
`Enum!(A,B,C)`. All these seems to bring significant changes to Rust internals.

The [`frunk_core`](https://docs.rs/frunk_core/) library provides `coproduct`
which is similar with ad-hic enums, by which this library is inspired. However
it aims at generic programming and the coproduct is nested `enum`s, not
supporting pattern matching.

# License

Licensed under MIT.
