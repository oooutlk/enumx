// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! This project provides ad-hoc enum types, and traits for user-defined enum exchange.
//!
//! See [enumx README](https://github.com/oooutlk/enumx/blob/master/enumx/README.md) for more.

/// Constructs an enum from one of its variant type.
pub trait FromVariant<Variant,Index> {
    fn from_variant( variant: Variant ) -> Self;
}

/// Converts into an enum as its variant.
pub trait IntoEnum<Enum,Index> {
    fn into_enum( self ) -> Enum;
}

impl<Enum,Variant,Index> IntoEnum<Enum,Index> for Variant
    where Enum: FromVariant<Variant,Index>
{
    fn into_enum( self ) -> Enum { FromVariant::<Variant,Index>::from_variant( self )}
}

/// Converts an ad-hoc enum into another one.
pub trait IntoEnumX<Dest,Indices> {
    fn into_enumx( self ) -> Dest;
}

/// Constructs an ad-hoc enum from another one.
pub trait FromEnumX<Src,Indices> {
    fn from_enumx( src: Src ) -> Self;
}

impl<Src,Dest,Indices> FromEnumX<Src,Indices> for Dest
    where Src: IntoEnumX<Dest,Indices>
{
    fn from_enumx( src: Src ) -> Self { src.into_enumx() }
}

/// Indicates the prototype for an enum that participates in enum exchange.
pub trait Exchange {
    type EnumX;
}

/// Constructs a user-defined enum from another user-defined enum.
pub trait ExchangeFrom<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices> {
    fn exchange_from( src: SrcNamed ) -> Self;
}

impl<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices> ExchangeFrom<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices> for DestNamed
    where DestNamed   : Exchange<EnumX=DestUnnamed> + From<DestUnnamed>
        , SrcNamed    : Exchange<EnumX=SrcUnnamed>  + Into<SrcUnnamed>
        , DestUnnamed : FromEnumX<SrcUnnamed,Indices>
{
    fn exchange_from( src: SrcNamed ) -> Self {
        DestNamed::from( FromEnumX::<SrcUnnamed,Indices>::from_enumx( src.into() ))
    }
}

/// Converts a user-defined enum into another user-defined enum.
pub trait ExchangeInto<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices> {
    fn exchange_into( self ) -> DestNamed;
}

impl<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices> ExchangeInto<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices> for SrcNamed
    where DestNamed: ExchangeFrom<SrcNamed,SrcUnnamed,DestNamed,DestUnnamed,Indices>
{
    fn exchange_into( self ) -> DestNamed {
        DestNamed::exchange_from( self )
    }
}

/// Recursive descent indices for `IntoEnumX`
pub struct LR<L,R> {
    pub l : L,
    pub r : R,
}

/// Indicates an impossible index for `Enum0`, internal use only.
pub struct Nil;

/// Never type, placeholder in the last `R` variant of nested `LR`s.
pub enum Enum0 {}

macro_rules! variant_index_types {
    ($($index:ident)+) => {$(
        pub struct $index;
    )+}
}

variant_index_types! { V0 V1 V2 V3 V4 V5 V6 V7 V8 V9 V10 V11 V12 V13 V14 V15 V16 V17 V18 V19 V20 V21 V22 V23 V24 V25 V26 V27 V28 V29 V30 V31 V32 }

macro_rules! enum_types {
    ($( $enum:ident<$($generic:ident),+>{ $($variant_name:ident)+ } )+) => {$(
        #[derive( Debug,PartialEq,Eq,PartialOrd,Ord )]
        pub enum $enum<$($generic),+> {
            $( $variant_name( $generic )),+
        }

        impl<$($generic),+> $enum<$($generic),+> {$(
            pub fn $variant_name( e: $generic ) -> Self {
                $enum::$variant_name( e )
            }
        )+}
    )+}
}

enum_types! {
     Enum1<T0>{ _0 }
     Enum2<T0,T1>{ _0 _1 }
     Enum3<T0,T1,T2>{ _0 _1 _2 }
     Enum4<T0,T1,T2,T3>{ _0 _1 _2 _3 }
     Enum5<T0,T1,T2,T3,T4>{ _0 _1 _2 _3 _4 }
     Enum6<T0,T1,T2,T3,T4,T5>{ _0 _1 _2 _3 _4 _5 }
     Enum7<T0,T1,T2,T3,T4,T5,T6>{ _0 _1 _2 _3 _4 _5 _6 }
     Enum8<T0,T1,T2,T3,T4,T5,T6,T7>{ _0 _1 _2 _3 _4 _5 _6 _7 }
     Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 }
     Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 }
     Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 }
     Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 }
     Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 }
     Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 }
     Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 }
     Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>{ _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 }
}

macro_rules! from_variant {
    ( $enum:ident<$($generics:ident),+>, $variant_name:ident, $variant_ty:ident, $variant_idx:ident ) => {
        impl<$($generics),+> FromVariant<$variant_ty,$variant_idx> for $enum<$($generics),+> {
            fn from_variant( variant: $variant_ty ) -> Self {
                $enum::$variant_name( variant )
            }
        }
    }
}

from_variant!{ Enum1<T0>, _0, T0, V0 }

from_variant!{ Enum2<T0,T1>, _0, T0, V0 }
from_variant!{ Enum2<T0,T1>, _1, T1, V1 }

from_variant!{ Enum3<T0,T1,T2>, _0, T0, V0 }
from_variant!{ Enum3<T0,T1,T2>, _1, T1, V1 }
from_variant!{ Enum3<T0,T1,T2>, _2, T2, V2 }

from_variant!{ Enum4<T0,T1,T2,T3>, _0, T0, V0 }
from_variant!{ Enum4<T0,T1,T2,T3>, _1, T1, V1 }
from_variant!{ Enum4<T0,T1,T2,T3>, _2, T2, V2 }
from_variant!{ Enum4<T0,T1,T2,T3>, _3, T3, V3 }

from_variant!{ Enum5<T0,T1,T2,T3,T4>, _0, T0, V0 }
from_variant!{ Enum5<T0,T1,T2,T3,T4>, _1, T1, V1 }
from_variant!{ Enum5<T0,T1,T2,T3,T4>, _2, T2, V2 }
from_variant!{ Enum5<T0,T1,T2,T3,T4>, _3, T3, V3 }
from_variant!{ Enum5<T0,T1,T2,T3,T4>, _4, T4, V4 }

from_variant!{ Enum6<T0,T1,T2,T3,T4,T5>, _0, T0, V0 }
from_variant!{ Enum6<T0,T1,T2,T3,T4,T5>, _1, T1, V1 }
from_variant!{ Enum6<T0,T1,T2,T3,T4,T5>, _2, T2, V2 }
from_variant!{ Enum6<T0,T1,T2,T3,T4,T5>, _3, T3, V3 }
from_variant!{ Enum6<T0,T1,T2,T3,T4,T5>, _4, T4, V4 }
from_variant!{ Enum6<T0,T1,T2,T3,T4,T5>, _5, T5, V5 }

from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _0, T0, V0 }
from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _1, T1, V1 }
from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _2, T2, V2 }
from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _3, T3, V3 }
from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _4, T4, V4 }
from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _5, T5, V5 }
from_variant!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, _6, T6, V6 }

from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _0, T0, V0 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _1, T1, V1 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _2, T2, V2 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _3, T3, V3 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _4, T4, V4 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _5, T5, V5 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _6, T6, V6 }
from_variant!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, _7, T7, V7 }

from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _0, T0, V0 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _1, T1, V1 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _2, T2, V2 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _3, T3, V3 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _4, T4, V4 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _5, T5, V5 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _6, T6, V6 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _7, T7, V7 }
from_variant!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, _8, T8, V8 }

from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _0, T0, V0 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _1, T1, V1 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _2, T2, V2 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _3, T3, V3 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _4, T4, V4 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _5, T5, V5 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _6, T6, V6 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _7, T7, V7 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _8, T8, V8 }
from_variant!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, _9, T9, V9 }

from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _0, T0, V0 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _1, T1, V1 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _2, T2, V2 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _3, T3, V3 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _4, T4, V4 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _5, T5, V5 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _6, T6, V6 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _7, T7, V7 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _8, T8, V8 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _9, T9, V9 }
from_variant!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, _10, T10, V10 }

from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _0, T0, V0 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _1, T1, V1 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _2, T2, V2 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _3, T3, V3 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _4, T4, V4 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _5, T5, V5 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _6, T6, V6 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _7, T7, V7 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _8, T8, V8 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _9, T9, V9 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _10, T10, V10 }
from_variant!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, _11, T11, V11 }

from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _0, T0, V0 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _1, T1, V1 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _2, T2, V2 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _3, T3, V3 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _4, T4, V4 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _5, T5, V5 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _6, T6, V6 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _7, T7, V7 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _8, T8, V8 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _9, T9, V9 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _10, T10, V10 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _11, T11, V11 }
from_variant!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, _12, T12, V12 }

from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _0, T0, V0 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _1, T1, V1 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _2, T2, V2 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _3, T3, V3 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _4, T4, V4 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _5, T5, V5 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _6, T6, V6 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _7, T7, V7 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _8, T8, V8 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _9, T9, V9 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _10, T10, V10 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _11, T11, V11 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _12, T12, V12 }
from_variant!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, _13, T13, V13 }

from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _0, T0, V0 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _1, T1, V1 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _2, T2, V2 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _3, T3, V3 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _4, T4, V4 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _5, T5, V5 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _6, T6, V6 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _7, T7, V7 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _8, T8, V8 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _9, T9, V9 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _10, T10, V10 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _11, T11, V11 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _12, T12, V12 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _13, T13, V13 }
from_variant!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, _14, T14, V14 }
 
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _0, T0, V0 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _1, T1, V1 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _2, T2, V2 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _3, T3, V3 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _4, T4, V4 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _5, T5, V5 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _6, T6, V6 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _7, T7, V7 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _8, T8, V8 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _9, T9, V9 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _10, T10, V10 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _11, T11, V11 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _12, T12, V12 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _13, T13, V13 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _14, T14, V14 }
from_variant!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, _15, T15, V15 }

impl<U0> IntoEnumX<Enum1<U0>,Nil> for Enum0 {
    fn into_enumx( self ) -> Enum1<U0> { match self {} }
}

impl<L,T0,U0> IntoEnumX<Enum1<U0>,LR<L,Enum0>> for Enum1<T0>
    where Enum1<U0> : FromVariant<T0,L>
{
    fn into_enumx( self ) -> Enum1<U0> {
        match self {
            Enum1::_0(v) => Enum1::<U0>::from_variant( v ),
        }
    }
}

macro_rules! into_enum0 {
    ( $enum:ident<U0,$($generics:ident),+>, $descent_enum:ident ) => {
        impl<U0,$($generics),+> IntoEnumX<$enum<U0,$($generics),+>,Nil> for Enum0
            where Enum0 : IntoEnumX<$descent_enum<$($generics),+>,Nil>
        {
            fn into_enumx( self ) -> $enum<U0,$($generics),+> { match self {} }
        }
    }
}

macro_rules! into_enum1 {
    ( $enum:ident<$($generics:ident),+> ) => {
        impl<L,T0,$($generics),+> IntoEnumX<$enum<$($generics),+>,LR<L,Enum0>> for Enum1<T0>
            where $enum<$($generics),+> : FromVariant<T0,L>
        {
            fn into_enumx( self ) -> $enum<$($generics),+> {
                match self {
                    Enum1::_0(v) => $enum::<$($generics),+>::from_variant(v),
                }
            }
        }
    }
}

macro_rules! into_enumx {
    ( $src_enum:ident<T0,$($descent_generics:ident),+>, $dest_enum:ident<$($dest_generics:ident),+>, $descent_enum:ident{ $($dest_descent_variant_name:ident $src_descent_variant_name:ident)+ } ) => {
        impl<L,R,T0,$($descent_generics),+,$($dest_generics),+> IntoEnumX<$dest_enum<$($dest_generics),+>,LR<L,R>> for $src_enum<T0,$($descent_generics),+>
            where $dest_enum<$($dest_generics),+>       : FromVariant<T0,L>
                , $descent_enum<$($descent_generics),+> : IntoEnumX<$dest_enum<$($dest_generics),+>,R>
        {
            fn into_enumx( self ) -> $dest_enum<$($dest_generics),+> {
                match self {
                    $src_enum::_0(v) => $dest_enum::<$($dest_generics),+>::from_variant(v),
                    $( $src_enum::$src_descent_variant_name(v) => $descent_enum::$dest_descent_variant_name(v).into_enumx() ),+
                }
            }
        }
    }
}

into_enum0!{               Enum2<U0,U1>, Enum1 }
into_enum1!{               Enum2<U0,U1> }
into_enumx!{ Enum2<T0,T1>, Enum2<U0,U1>, Enum1{_0 _1} }

into_enum0!{                  Enum3<U0,U1,U2>, Enum2 }
into_enum1!{                  Enum3<U0,U1,U2> }
into_enumx!{ Enum2<T0,T1>,    Enum3<U0,U1,U2>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>, Enum3<U0,U1,U2>, Enum2{_0 _1 _1 _2} }

into_enum0!{                     Enum4<U0,U1,U2,U3>, Enum3 }
into_enum1!{                     Enum4<U0,U1,U2,U3> }
into_enumx!{ Enum2<T0,T1>,       Enum4<U0,U1,U2,U3>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,    Enum4<U0,U1,U2,U3>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>, Enum4<U0,U1,U2,U3>, Enum3{_0 _1 _1 _2 _2 _3} }

into_enum0!{                        Enum5<U0,U1,U2,U3,U4>, Enum4 }
into_enum1!{                        Enum5<U0,U1,U2,U3,U4> }
into_enumx!{ Enum2<T0,T1>,          Enum5<U0,U1,U2,U3,U4>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,       Enum5<U0,U1,U2,U3,U4>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,    Enum5<U0,U1,U2,U3,U4>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>, Enum5<U0,U1,U2,U3,U4>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }

into_enum0!{                           Enum6<U0,U1,U2,U3,U4,U5>, Enum5 }
into_enum1!{                           Enum6<U0,U1,U2,U3,U4,U5> }
into_enumx!{ Enum2<T0,T1>,             Enum6<U0,U1,U2,U3,U4,U5>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,          Enum6<U0,U1,U2,U3,U4,U5>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,       Enum6<U0,U1,U2,U3,U4,U5>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,    Enum6<U0,U1,U2,U3,U4,U5>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>, Enum6<U0,U1,U2,U3,U4,U5>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }

into_enum0!{                              Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum6 }
into_enum1!{                              Enum7<U0,U1,U2,U3,U4,U5,U6> }
into_enumx!{ Enum2<T0,T1>,                Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,             Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,          Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,       Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,    Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>, Enum7<U0,U1,U2,U3,U4,U5,U6>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }

into_enum0!{                                 Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum7 }
into_enum1!{                                 Enum8<U0,U1,U2,U3,U4,U5,U6,U7> }
into_enumx!{ Enum2<T0,T1>,                   Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,             Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,          Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,       Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,    Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>, Enum8<U0,U1,U2,U3,U4,U5,U6,U7>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }

into_enum0!{                                    Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum8 }
into_enum1!{                                    Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8> }
into_enumx!{ Enum2<T0,T1>,                      Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                   Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,             Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,          Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,       Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,    Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>, Enum9<U0,U1,U2,U3,U4,U5,U6,U7,U8>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }

into_enum0!{                                        Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum9 }
into_enum1!{                                        Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9> }
into_enumx!{ Enum2<T0,T1>,                          Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                       Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                    Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                 Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,              Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,           Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,        Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,     Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>, Enum10<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }

into_enum0!{                                            Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum10 }
into_enum1!{                                            Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10> }
into_enumx!{ Enum2<T0,T1>,                              Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                           Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                        Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                     Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,                  Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,               Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,            Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,         Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>,     Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }
into_enumx!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>, Enum11<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10>, Enum10{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10} }

into_enum0!{                                                Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum11 }
into_enum1!{                                                Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11> }
into_enumx!{ Enum2<T0,T1>,                                  Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                               Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                            Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                         Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,                      Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,                   Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,                Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,             Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>,         Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }
into_enumx!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>,     Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum10{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10} }
into_enumx!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>, Enum12<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11>, Enum11{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11} }

into_enum0!{                                                    Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum12 }
into_enum1!{                                                    Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12> }
into_enumx!{ Enum2<T0,T1>,                                      Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                                   Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                                Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                             Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,                          Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,                       Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,                    Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,                 Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>,             Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }
into_enumx!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>,         Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum10{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10} }
into_enumx!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>,     Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum11{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11} }
into_enumx!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>, Enum13<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12>, Enum12{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12} }

into_enum0!{                                                        Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum13 }
into_enum1!{                                                        Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13> }
into_enumx!{ Enum2<T0,T1>,                                          Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                                       Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                                    Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                                 Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,                              Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,                           Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,                        Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,                     Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>,                 Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }
into_enumx!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>,             Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum10{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10} }
into_enumx!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>,         Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum11{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11} }
into_enumx!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>,     Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum12{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12} }
into_enumx!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>, Enum14<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13>, Enum13{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12 _12 _13} }

into_enum0!{                                                            Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum14 }
into_enum1!{                                                            Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14> }
into_enumx!{ Enum2<T0,T1>,                                              Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                                           Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                                        Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                                     Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,                                  Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,                               Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,                            Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,                         Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>,                     Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }
into_enumx!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>,                 Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum10{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10} }
into_enumx!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>,             Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum11{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11} }
into_enumx!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>,         Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum12{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12} }
into_enumx!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>,     Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum13{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12 _12 _13} }
into_enumx!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>, Enum15<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14>, Enum14{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12 _12 _13 _13 _14} }

into_enum0!{                                                                Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum15 }
into_enum1!{                                                                Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15> }
into_enumx!{ Enum2<T0,T1>,                                                  Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum1{_0 _1} }
into_enumx!{ Enum3<T0,T1,T2>,                                               Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum2{_0 _1 _1 _2} }
into_enumx!{ Enum4<T0,T1,T2,T3>,                                            Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum3{_0 _1 _1 _2 _2 _3} }
into_enumx!{ Enum5<T0,T1,T2,T3,T4>,                                         Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum4{_0 _1 _1 _2 _2 _3 _3 _4} }
into_enumx!{ Enum6<T0,T1,T2,T3,T4,T5>,                                      Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum5{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5} }
into_enumx!{ Enum7<T0,T1,T2,T3,T4,T5,T6>,                                   Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum6{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6} }
into_enumx!{ Enum8<T0,T1,T2,T3,T4,T5,T6,T7>,                                Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum7{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7} }
into_enumx!{ Enum9<T0,T1,T2,T3,T4,T5,T6,T7,T8>,                             Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum8{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8} }
into_enumx!{ Enum10<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9>,                         Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum9{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9} }
into_enumx!{ Enum11<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10>,                     Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum10{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10} }
into_enumx!{ Enum12<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11>,                 Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum11{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11} }
into_enumx!{ Enum13<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12>,             Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum12{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12} }
into_enumx!{ Enum14<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13>,         Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum13{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12 _12 _13} }
into_enumx!{ Enum15<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14>,     Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum14{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12 _12 _13 _13 _14} }
into_enumx!{ Enum16<T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15>, Enum16<U0,U1,U2,U3,U4,U5,U6,U7,U8,U9,U10,U11,U12,U13,U14,U15>, Enum15{_0 _1 _1 _2 _2 _3 _3 _4 _4 _5 _5 _6 _6 _7 _7 _8 _8 _9 _9 _10 _10 _11 _11 _12 _12 _13 _13 _14 _14 _15} }

#[cfg(feature="enum17_enum32")]
include!("enum17_enum32.rs");

#[cfg( test )]
mod test {
    use super::*;

    mod test_unamed {
        use super::*;

        #[test]
        fn test_from_variant() {
            let enum1 = Enum1::<i32>::from_variant( 2018 );
            assert_eq!( enum1, Enum1::_0( 2018 ));
            let enum2 = Enum2::<i32,String>::from_variant( "rust".to_string() );
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));
            let enum3 = Enum3::<i32,String,bool>::from_variant( true );
            assert_eq!( enum3, Enum3::_2( true ));
        }

        #[test]
        fn test_into_enum() {
            let enum1: Enum1<i32> = 2018.into_enum();
            assert_eq!( enum1, Enum1::_0( 2018 ));

            let enum2: Enum2<i32,String> = "rust".to_string().into_enum();
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));

            let enum3: Enum3<i32,String,bool> = true.into_enum();
            assert_eq!( enum3, Enum3::_2( true ));
        }

        #[test]
        fn test_exchange_into() {
            let enum1 = Enum1::<i32>::from_variant( 2018 );

            let enum1: Enum1<i32> = enum1.into_enumx();
            assert_eq!( enum1, Enum1::_0( 2018 ));

            let enum2: Enum2<String,i32> = enum1.into_enumx();
            assert_eq!( enum2, Enum2::_1( 2018 ));

            let enum2: Enum2<i32,String> = enum2.into_enumx();
            assert_eq!( enum2, Enum2::_0( 2018 ));

            let enum3: Enum3<bool,String,i32> = enum2.into_enumx();
            assert_eq!( enum3, Enum3::_2( 2018 ));

            let enum3: Enum3<i32,String,bool> = enum3.into_enumx();
            assert_eq!( enum3, Enum3::_0( 2018 ));
        }

        #[test]
        fn test_exchange_from() {
            let enum1 = Enum1::<String>::from_variant( "rust".to_string() );

            let enum1 = Enum1::<String>::from_enumx( enum1 );
            assert_eq!( enum1, Enum1::_0( "rust".to_string() ));

            let enum2 = Enum2::<i32,String>::from_enumx( enum1 );
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));

            let enum2 = Enum2::<String,i32>::from_enumx( enum2 );
            assert_eq!( enum2, Enum2::_0( "rust".to_string() ));

            let enum3 = Enum3::<bool,i32,String>::from_enumx( enum2 );
            assert_eq!( enum3, Enum3::_2( "rust".to_string() ));

            let enum3 = Enum3::<String,i32,bool>::from_enumx( enum3 );
            assert_eq!( enum3, Enum3::_0( "rust".to_string() ));
        }
    }

    mod test_named {
        use super::*;
        use enumx_derive::Exchange;

        #[derive(Exchange,Debug,PartialEq,Eq)]
        enum One<T0> {
            The(T0),
        }

        #[derive(Exchange,Debug,PartialEq,Eq)]
        enum Two<T0,T1> {
            Formmer(T0),
            Latter(T1),
        }

        #[derive(Exchange,Debug,PartialEq,Eq)]
        enum Three<T0,T1,T2> {
            First(T0),
            Second(T1),
            Third(T2),
        }

        #[test]
        fn test_from_variant() {
            let one = One::<i32>::from_variant( 2018 );
            assert_eq!( one, One::The( 2018 ));
            let two = Two::<i32,String>::from_variant( "rust".to_string() );
            assert_eq!( two, Two::Latter( "rust".to_string() ));
            let three = Three::<i32,String,bool>::from_variant( true );
            assert_eq!( three, Three::Third( true ));
        }

        #[test]
        fn test_into_enum() {
            let one: One<i32> = 2018.into_enum();
            assert_eq!( one, One::The( 2018 ));

            let two: Two<i32,String> = "rust".to_string().into_enum();
            assert_eq!( two, Two::Latter( "rust".to_string() ));

            let three: Three<i32,String,bool> = true.into_enum();
            assert_eq!( three, Three::Third( true ));
        }

        #[test]
        fn test_exchange_into() {
            let one = One::<String>::from_variant( "rust".to_string() );

            let one: One<String> = one.exchange_into();
            assert_eq!( one, One::The( "rust".to_string() ));

            let two: Two<i32,String> = one.exchange_into();
            assert_eq!( two, Two::Latter( "rust".to_string() ));

            let two: Two<String,i32> = two.exchange_into();
            assert_eq!( two, Two::Formmer( "rust".to_string() ));

            let three: Three<bool,i32,String> = two.exchange_into();
            assert_eq!( three, Three::Third( "rust".to_string() ));

            let three: Three<String,i32,bool> = three.exchange_into();
            assert_eq!( three, Three::First( "rust".to_string() ));
        }

        #[test]
        fn test_exchange_from() {
            let one = One::<i32>::from_variant( 2018 );

            let one = One::<i32>::exchange_from( one );
            assert_eq!( one, One::The( 2018 ));

            let two = Two::<String,i32>::exchange_from( one );
            assert_eq!( two, Two::Latter( 2018 ));

            let two = Two::<i32,String>::exchange_from( two );
            assert_eq!( two, Two::Formmer( 2018 ));

            let three = Three::<bool,String,i32>::exchange_from( two );
            assert_eq!( three, Three::Third( 2018 ));

            let three = Three::<i32,String,bool>::exchange_from( three );
            assert_eq!( three, Three::First( 2018 ));
        }
    }
}
