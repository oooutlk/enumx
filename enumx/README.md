# Summary

Add types, traits, macros for providing structural enum types in Rust.

The new constructs are:

* An `Enum!(T0,T1..)` macro to define a structrual enum type composed of variant types T0,T1.. etc.

* A set of predefined `enum` types with the names `Enum0`, `Enum1`,.. etc to notate the structural enums of different amount of variants.

* An `Exchange` trait for deriving user-defined sructural `enum` types.

# Motivation

By definition, structural enum types can be converted to each ohter if their variant types are compatible. If provided, they will help programmers to directly express a sum type concept and keep them from hand-writing or macro-generating boilerplate code for convertion `impl`s.

One of an important use cases, putting error types in function signatures, aka `throws`, will be discussed in its own thread.

# Overview

## Definition of structrual enum type

A structural enum type of n variant types is notated as `Enum!( T0, T1, .., T(n-1) )`, the actual type name of which is `Enum$n<T0,T1,..,T(n-1)>`, while `$n` means the digit number n appended to `Enum` as a complete identity. Its variants are named as `_0`, `_1`, .. `_(n-1)`. The first variant can be pattern matched as `<Enum!( T0, T1, .., T(n-1) )>::_0` or `Enum$n::_0`, and so forth.

For example, `Enum!( i32, &'static str )` is a structural enum type of 2 variants, which is essentially `Enum2<i32,&'static str>`. Two of its variants are `Enum2::_0` and `Enum2::_1`.

`Enum0` is also defined as a never type.

## Construct a structrual enum

It is obvious that any variant can be converted to the structral enum type containing it.

* Type annotation
```rust
let enum2: Enum!(i32,&'static str) = 42.into_enum();
```

* Type inference
```rust
let enum2 = <Enum!(i32,&'static str)>::from_variant( 42 );
```

## Convertion between structrual enums

A structural enum could be converted to another one, if all its variants are in the latter.

For example, `Enum!(i32,&'static str)` could be converted to `Enum!(&'static str,i32)`, `Enum!(i32,bool,&'static str)`, but not `Enum!(u32,&'static str)` nor `Enum!(i32,String)`.

* Type annotation
```rust
let enum2: Enum!(i32,&static str) = 42.into_enum();
let enum2: Enum!(&'static str,i32) = enum2.into_enumx();
let enum3: Enum!(i32,bool,&'static str) = enum2.into_enumx();
```

* Type inference
```rust
let enum2 = <Enum!(i32,&static str)>::from_variant( 42 );
let enum2 = <Enum!(&'static str,i32)::from_enumx( enum2 );
let enum3 = <Enum!(i32,bool,&'static str)::from_enumx( enum2 );
```

## Define a user-defined structrual enum 

A user-defined structrual enum should be tagged with `#[derive(Exchange)]`, and is able to be exchanged with others of compatible variant types.

To distinguish a user-defined structrual enum from `Enum!()`, , by now, we call the former "exchangeable enum", and the latter "ad-hoc enum".

```rust
#[derive(Exchange)]
enum Info {
    Code(i32),
    Text(&'static str),
}

#[derive(Exchange)]
enum Data {
    Code(i32),
    Text(&'static str),
    Flag(bool),
}

#[derive(Exchange)]
enum Datum {
    Code(u32),
    Text(String),
    Flag(bool),
}
```

## Construct a exchangeable enum

It is obvious that any variant can be converted to the exchangeable enum containing it.

* Type annotation
```rust
let info: Info = 0xdeadbeef.into_enum();
```

* Type inference
```rust
let info = Info::from_variant( 0xdeadbeef );
```

## Convertion between exchangeable enums

A exchangeable enum could be converted to another one, if all its variants are in the latter.

For example, `Info` could be converted to `Data`, but not `Datum`.

* Type annotation
```rust
let info: Info = 0xdeadbeaf.into_enum();
let data: Data = info.exchange_into();
```

* Type inference
```rust
let info = Info::from_variant( 0xdeadbeaf );
let data = Data::exchange_from( info );
```

# Detailed Design

## Predefined ad-hoc enum types

A set of predefined `enum` types with the names `Enum0`, `Enum1`,.. are defined in the form of:

```rust
pub enum Enum0 {}
pub enum Enum1<T0> { _0(T0) }
pub enum Enum2<T0,T1> { _0(T0), _1(T1) }
/* omitted */
```

## Translating `Enum!()` to the actual ad-hoc enum type

The purpose for `Enum!()` is to keep programmers from manually counting variants to pick the right number as a suffix to `Enum`.

It could be implemented as a declarative macro, counting its arguments to find the corresponding predefined enum type:

```rust
macro_rules! Enum {
    ( $t0:ty ) => { Enum1<$t0> };
    ( $t0:ty, $t1:ty ) => { Enum2<$t0,$t1> };
    ( $t0:ty, $t1:ty, $t2:ty ) => { Enum3<$t0,$t1,$t2> };
    /* omitted */
}
```

## Type convertion implementations NOT to use `From`

This is the most interesting part in implementation. In general, we cannot generate such implementations by implementing `std::convert::From` trait, due to the potential overlapping of `impl`s, which is not allowed in Rust. Simple demonstration of convertion between two `Enum2`s:

```rust
impl<T0,T1,U0,U1> From<Enum2<T0,T1>> for Enum2<U0,U1> { /* omitted */ }
```

If T0 equals to U0 and T1 equals to U1, we are doing `impl From<T> for T` now, which will result in compiler error. Further more, we are going to meet this again and again no matter what tricks we play as long as sticking in `impl From` for generic types.

A sound method is developing our own traits rather than using standard `From` trait to do the convertion. These traits should be able to encode structural information to map variant(s) to its(their) proper positions.

What we need to do is to check the equality of types in trait bounds, which is not directly supported in Rust. We should do some transformations to make rustc happy.

## The phantom index

We will introduce a ZST named `Nil`, a set of index types which are ZSTs and named as `V0`,`V1`,.., `V(n-1)` to reflect the position of the type list `T0,T1,..,T(n-1)`, and a recursive struct `pub struct LR<L,R>( pub L, pub R );`, to transform the positions as a "cons of car/cdr": `LR(V0, LR(V1, .. LR(V(n-1),Nil))..)`.

## Construct a structrual enum

We will introduce two traits: `FromVariant<Variant,Index>` and `IntoEnum<Enum,Index>`:
```rust
pub trait FromVariant<Variant,Index> {
    fn from_variant( variant: Variant ) -> Self;
}

pub trait IntoEnum<Enum,Index> {
    fn into_enum( self ) -> Enum;
}
```

And a blanket `impl`:
```rust
impl<Enum,Variant,Index> IntoEnum<Enum,Index> for Variant
    where Enum: FromVariant<Variant,Index>
{
    fn into_enum( self ) -> Enum { FromVariant::<Variant,Index>::from_variant( self )}
}
```

Mapping a variant to the enum can be done in a [declarative macro](https://github.com/oooutlk/enumx/blob/master/enumx/src/lib.rs#L93).

## Convertion between structrual enums

We will introduce two traits: `IntoEnumX<Dest,Indices>` and `FromEnumX<Src,Indices>`:
```rust
pub trait IntoEnumX<Dest,Indices> {
    fn into_enumx( self ) -> Dest;
}

pub trait FromEnumX<Src,Indices> {
    fn from_enumx( src: Src ) -> Self;
}
```

And a blanket `impl`:
```rust
impl<Src,Dest,Indices> FromEnumX<Src,Indices> for Dest
    where Src: IntoEnumX<Dest,Indices>
{
    fn from_enumx( src: Src ) -> Self { src.into_enumx() }
}
```

For demonstrating the key idea, the following code snippet is quoted from [EnumX](https://github.com/oooutlk/enumx/blob/master/enumx/src/lib.rs#L162):

```rust
impl<L,R,T0,$($descent_generics),+,$($dest_generics),+> IntoEnumX<$dest_enum<$($dest_generics),+>,LR<L,R>> for $src_enum<T0,$($descent_generics),+>
    where $dest_enum<$($dest_generics),+>       : FromVariant<T0,L>
        , $descent_enum<$($descent_generics),+> : IntoEnumX<$dest_enum<$($dest_generics),+>,R>
```

`T0` is the first variant type of the source enum.

What we are doing is essentially check if the dest enum can be constructed from `T0`, and if not, try converting the rest variant types in source enum into the dest.
The `L` is the first index and the `R` is the rest indices. Notice: the two where clauses are not possible to be true at the same time.

## Define a user-defined structrual enum 

We will introduce an `Exchange` trait to reflect the prototype of an exchangeable enum, that is, an ad-hoc enum of the same variant types but renaming the variant names as `_0`,`_1`,.. accordingly.

```rust
pub trait Exchange {
    type EnumX;
}
```

For example, the `Exchange::EnumX` of the `Info` defined in previous section is `Enum2<i32,&'static str>`.

## Construct an exchangeable enum

It is obvious for `#[derive(Exchange)]` to generate `impl Exchange`, `impl From` EnumX, `impl Into` EnumX. All that we need to do is naming/renaming.

To `impl FromVariant<Variant,Index> for ExchangeableEnum`, first convert the variant to `Exchange::EnumX`, then convert it `Into` ExchangeableEnum.

To `impl IntoEnumX<AdhocEnum,Indices> for ExchangeableEnum`, first convert the ExchangeableEnum `Into` `Exchange::EnumX`, then convert it `IntoEnumX` AdhocEnum.

## Convertion between exchangeable enums

We will introduce two traits: `ExchangeFrom<Src,Indices>` and `ExchangeInto<Dest,Indices>`:

```rust
pub trait ExchangeFrom<Src,Indices> {
    fn exchange_from( src: Src ) -> Self;
}

pub trait ExchangeInto<Dest,Indices> {
    fn exchange_into( self ) -> Dest;
}
```

We convert the source enum to its ad-hoc enum, then to the dest's ad-hoc enum, then to the dest enum.

```rust
impl<Src,SrcAdhoc,Dest,DestAdhoc,Indices> ExchangeFrom<Src,Indices> for Dest
    where Dest      : Exchange<EnumX=DestAdhoc> + From<DestAdhoc>
        , Src       : Exchange<EnumX=SrcAdhoc>  + Into<SrcAdhoc>
        , DestAdhoc : FromEnumX<SrcAdhoc,Indices>
{
    fn exchange_from( src: Src ) -> Self {
        Dest::from( FromEnumX::<SrcAdhoc,Indices>::from_enumx( src.into() ))
    }
}
```

And a blanket trait implementation.

```rust
impl<Src,Dest,Indices> ExchangeInto<Dest,Indices> for Src
    where Dest: ExchangeFrom<Src,Indices>
{
    fn exchange_into( self ) -> Dest {
        Dest::exchange_from( self )
    }
}
```

# Drawbacks

1. The various kind of `From`/`Into`-alike traits may confuse users, and losing the chance in the situation that accepts standard `From`/`Into` only.

2. Predefined ad-hoc enums are a subset of possible ad-hoc enums. What if the programmer want an ad-hoc enum composed of 65535 variants?

# Rationale and alternatives

1. The EnumX v0.2 is inspired by [`frunk_core::coproduct`](https://docs.rs/frunk_core/0.2.2/frunk_core/coproduct/index.html), which provides another ad-hoc enum implementation. It uses the recursive enum as a public interface, getting rid of the variants count limit, at the cost of not supporting native pattern matching syntax on `enum`s. And the enum in recursive form may occupy more spaces than the flattern one proposed by this article. 

2. The EnumX v0.1 generates the convertion in standard `From`/`Into` trait, at the cost of not supporting generics, and heavy hacks in proc-macro attribute `#[enumx]`.

# Prior art

`frunk_core::coproduct`, and `EnumX` v0.1 just mentioned.

# Unresolved questions

Exchangeable enum is missing `FromEnumX` in the derive. As a result, there is no way to convert an ad-hoc enum to an exchangeable one automatically.

# Future possibilities

Make exchangeable enum a first-class structrual enum type.

# License

Licensed under MIT.
