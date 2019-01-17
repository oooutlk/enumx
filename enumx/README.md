# Summary

Add macros and traits, for minimal support of structural enums.

The new constructs are:

* Two traits `EnumxFrom`, `IntoEnumx`.

* A new proc-macro derive `EnumX` applicable for `enum`s, generating type
convertion `impl`s defined in these traits.

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

An enum with `#[derive(EnumX)]` is considered as an exchangeable enum.

```rust
#[derive(EnumX)]
enum Info {
    Code(i32),
    Text(&'static str),
}
```

An exchangeable enum can be constructed from one of its variants:

```rust
let info: Info = 42.into_enumx();
let info = Info::enumx_from(42);
```

An exchangeable enum can be exchanged from/into another exchangeable one, as
long as one has all the variant types appearing in the other one's definition.

```rust
#[derive(EnumX)]
enum Data {
    Num(i32),
    Text(&'static str),
    Flag(bool),
}

let info: Info = 42.into_enumx();
let data: Data = info.into_enumx();

let info = Info::enumx_from(42);
let data = Data::enumx_from(info);
```

## Enum methods

By now, we call `enumx_from()`, `into_enumx()` as enum exchange methods.

## Syntax limits of exchangeable enum

All variants must be in the form of "newtype".

```rust
#[derive(EnumX)]
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

## Convertion rules

The following 2 methods are the minimum support of structural enum:

*  `enumx_from()` and `into_enumx()`.

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
pub trait EnumxFrom<Src,Index> {
    fn enumx_from( src: Src ) -> Self;
}

pub trait IntoEnumx<Dest,Index> {
    fn into_enumx( self ) -> Dest;
}
```

Notice that the traits have phantom types `Index` in their generics to hold
positional information to help compiler accomplishing type inferences and avoid
overlapping `impl`s.

## Distinguish from `std::convert`

Since standard `From`/`Into` does not have such phantom types, it is not
feasible to implement enum exchange methods in `From`/`Into`. Trying to
implement in `From` will cause compile error because we need to impl multiple
`From<Variant>` but the generic `Variant` type in different `impl`s could be of
the same actual type, resulting in overlapping `impl`s.

# Drawbacks

- Abusing structural enums in cases that they are not suitable for.

- As a library mimicing structrual enums, enumx supports a limited count of
  variants. The default is 16, and can be twisted via environment variable
  `ENUMX_MAX_VARIANTS`.

# Rationale and alternatives

# Prior art

Concepts similar with structural enum exist in other language. One example is
union types in Typed Racket. However, it supports more powerful type inferences
such as uniquifing and flattening. All these seems to bring significant changes
to Rust internals.

The [`frunk_core`](https://docs.rs/frunk_core/) library provides `coproduct`
which is similar with structural enums, by which this library is inspired.
However it aims at generic programming and the coproduct is nested `enum`s, not
supporting pattern matching.

# License

Licensed under MIT.
