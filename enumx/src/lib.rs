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
pub trait ExchangeFrom<Src,Indices> {
    fn exchange_from( src: Src ) -> Self;
}

impl<Src,SrcAdhoc,Dest,DestAdhoc,Indices> ExchangeFrom<Src,Indices> for Dest
    where Dest      : Exchange<EnumX=DestAdhoc> + From<DestAdhoc>
        , Src       : Exchange<EnumX=SrcAdhoc>  + Into<SrcAdhoc>
        , DestAdhoc : FromEnumX<SrcAdhoc,Indices>
{
    fn exchange_from( src: Src ) -> Self {
        Dest::from( FromEnumX::<SrcAdhoc,Indices>::from_enumx( src.into() ))
    }
}

/// Converts a user-defined enum into another user-defined enum.
pub trait ExchangeInto<Dest,Indices> {
    fn exchange_into( self ) -> Dest;
}

impl<Src,Dest,Indices> ExchangeInto<Dest,Indices> for Src
    where Dest: ExchangeFrom<Src,Indices>
{
    fn exchange_into( self ) -> Dest {
        Dest::exchange_from( self )
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

variant_index_types! { V0 V1 V2 V3 V4 V5 V6 V7 V8 V9 V10 V11 V12 V13 V14 V15 V16 }

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

include!( "enum1_16.rs" );
#[cfg( feature="enum32" )] include!("enum17_32.rs");

macro_rules! enum_variant {
    ( $enum:ident<$($generics:ident),+>, $variant_name:ident, $variant_ty:ident, $variant_idx:ident ) => {
        impl<$($generics),+> FromVariant<$variant_ty,$variant_idx> for $enum<$($generics),+> {
            fn from_variant( variant: $variant_ty ) -> Self {
                $enum::$variant_name( variant )
            }
        }
    }
}

include!( "enum_variant1_16.rs" );
#[cfg( feature="enum32" )] include!("enum_variant17_32.rs");

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

include!("into_enum2_16.rs");
#[cfg( feature="enum32" )] include!("into_enum17_32.rs");

include!("sum1_16.rs");
#[cfg( feature="enum32" )] include!("sum17_32.rs");

include!("deref1_16.rs");
#[cfg( feature="enum32" )] include!("deref17_32.rs");

#[cfg(not( feature="enum32" ))] include!("enum_macro_1_16.rs");
#[cfg(     feature="enum32"  )] include!("enum_macro_1_32.rs");

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
        extern crate enumx_derive;
        use enumx_derive::Exchange;

        #[derive(Exchange,Debug,PartialEq,Eq)]
        enum One<T0> {
            The(T0),
        }

        #[derive(Exchange,Debug,PartialEq,Eq)]
        enum Two<T0,T1> {
            Former(T0),
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
            assert_eq!( two, Two::Former( "rust".to_string() ));

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
            assert_eq!( two, Two::Former( 2018 ));

            let three = Three::<bool,String,i32>::exchange_from( two );
            assert_eq!( three, Three::Third( 2018 ));

            let three = Three::<i32,String,bool>::exchange_from( three );
            assert_eq!( three, Three::First( 2018 ));
        }

        #[test]
        fn test_adhoc_from_enumx_named() {
            let three = Three::<bool,String,i32>::from_variant( 2018 );
            let enum3 = Enum3::<String,i32,bool>::from_enumx( three );
            assert_eq!( enum3, Enum3::_1( 2018 ));

            #[derive( Exchange, Debug, PartialEq, Eq )]
            enum IU { I(i32), U(u32) }

            #[derive( Exchange, Debug, PartialEq, Eq )]
            enum BIU { B(bool), I(i32), U(u32) }
        }

        #[test]
        fn test_named_into_enumx_adhoc() {
            let three = Three::<bool,String,i32>::from_variant( 2018 );
            let enum3: Enum3<String,i32,bool> = three.into_enumx();
            assert_eq!( enum3, Enum3::_1( 2018 ));
        }

        #[test]
        fn test_deref() {
            fn foo<'a>( data: &'a [u32], f: bool ) -> Sum2!( impl Iterator<Item=u32> + 'a ) {
                if f {
                    Enum2::_0( data.iter().map( |x| 2 * x ))
                } else {
                    Enum2::_1( data.iter().map( |x| x + 2 ))
                }
            }

            let data = [ 1, 2, 3 ];

            let mut iter = foo( &data[..], true );
            assert_eq!( deref2!( iter,size_hint() ), (3,Some(3)) );
            assert_eq!( deref_mut2!( iter,next() ), Some(2) );
            assert_eq!( deref_mut2!( iter,next() ), Some(4) );
            assert_eq!( deref_mut2!( iter,next() ), Some(6) );
            assert_eq!( deref_mut2!( iter,next() ), None    );

            let mut iter = foo( &data[..], false );
            assert_eq!( deref2!( iter,size_hint() ), (3,Some(3)) );
            assert_eq!( deref_mut2!( iter,next() ), Some(3) );
            assert_eq!( deref_mut2!( iter,next() ), Some(4) );
            assert_eq!( deref_mut2!( iter,next() ), Some(5) );
            assert_eq!( deref_mut2!( iter,next() ), None    );
        }
    }
}
