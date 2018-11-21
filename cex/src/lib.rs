// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

pub use enumx::*;

/// Checked EXception.
pub trait CeX {
    type LR;
    fn from_lr( lr: Self::LR, backtrace: Backtrace ) -> Self;
    fn into_lr( self ) -> ( Self::LR, Backtrace );
}

#[derive( Debug,Default,PartialEq,Eq )]
pub struct Backtrace( pub Vec<ThrowPoint> );

#[derive( Debug,Default,PartialEq,Eq )]
pub struct ThrowPoint {
    pub line     : u32,
    pub column   : u32,
    pub function : &'static str,
    pub module   : &'static str,
    pub file     : &'static str,
}

impl ThrowPoint {
    pub fn new( line: u32, column: u32, function: &'static str, module: &'static str, file: &'static str ) -> Self {
        ThrowPoint{ line, column, function, module, file }
    }
}

use cex_derive::CeXDerives;
use std::cmp::Ordering;

macro_rules! cex_types {
    ( $($cex:ident( $enum:ident<$($err:ident),+> $($variant:ident)+ );)+) => {$(
        #[derive( CeXDerives )]
        #[derive( Debug )]
        pub struct $cex<$($err),+> {
            pub error     : $enum<$($err),+>,
            pub backtrace : Backtrace,
        }

        impl<$($err),+> $cex<$($err),+> {$(
            pub fn $variant( e: $err ) -> Self {
                $cex {
                    error     : $enum::$variant( e ),
                    backtrace : Backtrace::default(),
                }
            }
        )+}

        impl<$($err),+> PartialEq for $cex<$($err),+>
            where $enum<$($err),+> : PartialEq
        {
            fn eq( &self, other: &Self ) -> bool { self.error == other.error }
            fn ne( &self, other: &Self ) -> bool { self.error != other.error }
        }

        impl<$($err),+> Eq for $cex<$($err),+>
            where $enum<$($err),+> : Eq
        {
        }

        impl<$($err),+> PartialOrd for $cex<$($err),+>
            where $enum<$($err),+> : PartialOrd
        {
            fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
                self.error.partial_cmp( &other.error )
            }
        }
        
        impl<$($err),+> Ord for $cex<$($err),+>
            where $enum<$($err),+> : Ord
        {
            #[inline] fn cmp( &self, other: &Self ) -> Ordering {
                self.error.cmp( &other.error )
            }
        }
    )+}
}

cex_types! {
     Ce1( Enum1<E0> _0 );
     Ce2( Enum2<E0,E1> _0 _1 );
     Ce3( Enum3<E0,E1,E2> _0 _1 _2 );
     Ce4( Enum4<E0,E1,E2,E3> _0 _1 _2 _3 );
     Ce5( Enum5<E0,E1,E2,E3,E4> _0 _1 _2 _3 _4 );
     Ce6( Enum6<E0,E1,E2,E3,E4,E5> _0 _1 _2 _3 _4 _5 );
     Ce7( Enum7<E0,E1,E2,E3,E4,E5,E6> _0 _1 _2 _3 _4 _5 _6 );
     Ce8( Enum8<E0,E1,E2,E3,E4,E5,E6,E7> _0 _1 _2 _3 _4 _5 _6 _7 );
     Ce9( Enum9<E0,E1,E2,E3,E4,E5,E6,E7,E8> _0 _1 _2 _3 _4 _5 _6 _7 _8 );
    Ce10( Enum10<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 );
    Ce11( Enum11<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 );
    Ce12( Enum12<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 );
    Ce13( Enum13<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 );
    Ce14( Enum14<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 );
    Ce15( Enum15<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 );
    Ce16( Enum16<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 );
    Ce17( Enum17<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 );
    Ce18( Enum18<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 );
    Ce19( Enum19<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 );
    Ce20( Enum20<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 );
    Ce21( Enum21<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 );
    Ce22( Enum22<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 );
    Ce23( Enum23<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 );
    Ce24( Enum24<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 );
    Ce25( Enum25<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 );
    Ce26( Enum26<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 );
    Ce27( Enum27<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 );
    Ce28( Enum28<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26,E27> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 );
    Ce29( Enum29<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26,E27,E28> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 );
    Ce30( Enum30<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26,E27,E28,E29> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29 );
    Ce31( Enum31<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26,E27,E28,E29,E30> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29 _30 );
    Ce32( Enum32<E0,E1,E2,E3,E4,E5,E6,E7,E8,E9,E10,E11,E12,E13,E14,E15,E16,E17,E18,E19,E20,E21,E22,E23,E24,E25,E26,E27,E28,E29,E30,E31> _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _15 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29 _30 _31 );
}
