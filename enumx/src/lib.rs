// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! enumx = ENUM eXtensions.
//!
//! See the [enumx book](https://oooutlk.github.io/enumx/) for more.

#![cfg_attr( feature="unstable", feature(
    fn_traits,
    generator_trait,
    generators,
    trusted_len,
    unboxed_closures
))]

pub mod macros;

/// Reorganize types, traits and macros to export to end users.
///
/// Two categories:
///
/// 1. enum exchange.
///
/// 2. enum trait implementations.
///
/// If users want to import utils in #1, just `use enumx::export::exchange::*;`.
/// If users want to import utils in #2, just `use enumx::export::impls::*;`.
/// If utils both in #1 and #2 are needed, just `use enumx::export::*;`.
pub mod export {
    pub mod exchange {
        pub use crate::{
            EnumToEnum,
            ExchangeFrom,
            ExchangeInto,
            FromVariant,
            IntoEnum,
            TyPat,
        };
        pub use enumx_derive::{
            Enum,
            Exchange,
            FromVariant as _,
            Proto,
            def_impls,
            enumx,
        };
    }
    pub mod impls {
        pub mod derives {
            pub use enumx_derive::{
                def_impls,
                sum,
                sum_err,
            };
        }
        pub use derives::*;
        pub use crate::{
            impl_trait,
            impl_super_traits,
            impl_all_traits,
        };
    }

    pub use exchange::*;
    pub use impls::*;
}

pub use enumx_derive::{
    Enum,
    Exchange,
    FromVariant,
    Proto,
    def_impls,
    enumx,
    sum,
    sum_err,
};

/// Constructs an enum from one of its variants.
pub trait FromVariant<Variant, Index> {
    fn from_variant( variant: Variant ) -> Self;
}

/// Wraps a variant into an enum.
pub trait IntoEnum<Enum, Index> {
    fn into_enum( self ) -> Enum;
}

impl<Enum,Variant,Index> IntoEnum<Enum, Index> for Variant
    where Enum: FromVariant<Variant, Index>
{
    fn into_enum( self ) -> Enum {
        <Enum as FromVariant::<Variant, Index>>::from_variant( self )
    }
}

/// Constructs an enum from one of its variants, or from an enum composed of a subset of its variants.
pub trait ExchangeFrom<Src, Index> {
    fn exchange_from( src: Src ) -> Self;
}

/// Wraps a variant into an enum, or converts an enum into another one, the variants of which is a superset of the converted enum's.
pub trait ExchangeInto<Dest, Index> {
    fn exchange_into( self ) -> Dest;
}

impl<Src, Dest, Index> ExchangeInto<Dest, Index> for Src
    where Dest: ExchangeFrom<Src, Index>,
{
    fn exchange_into( self ) -> Dest {
        Dest::exchange_from( self )
    }
}

/// Used in `ExchangeFrom`/`ExchangeInto` to distinguish conversions between enums from those between an enum and its variant.
pub struct EnumToEnum<Index>( Index );

/// Indicates the prototype for a user-defined `Exchange`-able enum.
pub trait Proto {
    type Type;
    fn from_proto( src: Self::Type ) -> Self;
    fn into_proto( self ) -> Self::Type;
}

/// # Predefined ad-hoc enums
///
/// This library has defined `Enum0`, `Enum1` .. up to `Enum16` by default.
///
/// The library user can `use enumx::predefined::*;` for convenience.
///
/// A feature named "enum32" increases the set of predefined enums up to `Enum32`.
///
/// `Cargo.toml`:
///
/// ```toml
/// [dependencies.enumx]
/// version = "0.4"
/// features = "enum32"
/// ```
///
/// The predefined enums can be disabled by opting out "Enum16" and "Enum32" features.
///
/// `Cargo.toml`:
///
/// ```toml
/// [dependencies.enumx]
/// version = "0.4"
/// default-features = false
/// ```
#[cfg( any( feature="enum16", feature="enum32" ))]
pub mod predefined {
    use crate as enumx;
    use crate::{Exchange, def_impls, impl_trait};

    #[cfg( feature="enum16" )]
    def_impls! {
        #[derive( Exchange, Clone, Debug, PartialEq, Eq, PartialOrd, Ord )]
        pub enum Enum![ 0..=16 ];
    }

    #[cfg( feature="enum32" )]
    def_impls! {
        #[derive( Exchange, Clone, Debug, PartialEq, Eq, PartialOrd, Ord )]
        pub enum Enum![ 17..=32 ];
    }

    #[cfg( feature="enum16" )]
    pub mod enum1_16 {
        use super::*;

        impl_trait!{ _impl!(T) AsRef<T> _for!( Enum![1..=16] )}
        impl_trait!{ _impl!(T) AsMut<T> _for!( Enum![1..=16] )}
        impl_trait!{ DoubleEndedIterator _for!( Enum![1..=16] )}
        impl_trait!{ ExactSizeIterator _for!( Enum![1..=16] )}
        impl_trait!{ _impl!(A) Extend<A> _for!( Enum![1..=16] )}
        impl_trait!{ Iterator _for!( Enum![1..=16] )}
        impl_trait!{ std::error::Error _for!( Enum![1..=16] )}
        impl_trait!{ std::fmt::Display _for!( Enum![1..=16] )}
        impl_trait!{ std::iter::FusedIterator _for!( Enum![1..=16] )}
        impl_trait!{ std::ops::Deref _for!( Enum![1..=16] )}
        impl_trait!{ std::ops::DerefMut _for!( Enum![1..=16] )}

        #[cfg( feature="unstable" )]
        crate::impl_all_traits!{ _impl!(Args) Fn<Args> _for!( Enum![1..=16] )}

        #[cfg( feature="unstable" )]
        impl_trait!{ std::iter::TrustedLen _for!( Enum![1..=16] )}

        #[cfg( feature="unstable" )]
        impl_trait!{ _impl!(R) std::ops::Generator<R> _for!( Enum![1..=16] )}
    }

    #[cfg( feature="enum32" )]
    pub mod enum17_32 {
        use super::*;

        impl_trait!{ _impl!(T) AsRef<T> _for!( Enum![17..=32] )}
        impl_trait!{ _impl!(T) AsMut<T> _for!( Enum![17..=32] )}
        impl_trait!{ DoubleEndedIterator _for!( Enum![17..=32] )}
        impl_trait!{ ExactSizeIterator _for!( Enum![17..=32] )}
        impl_trait!{ _impl!(A) Extend<A> _for!( Enum![17..=32] )}
        impl_trait!{ Iterator _for!( Enum![17..=32] )}
        impl_trait!{ std::error::Error _for!( Enum![17..=32] )}
        impl_trait!{ std::fmt::Display _for!( Enum![17..=32] )}
        impl_trait!{ std::iter::FusedIterator _for!( Enum![17..=32] )}
        impl_trait!{ std::ops::Deref _for!( Enum![17..=32] )}
        impl_trait!{ std::ops::DerefMut _for!( Enum![17..=32] )}

        #[cfg( feature="unstable" )]
        crate::impl_all_traits!{ _impl!(Args) Fn<Args> _for!( Enum![17..=32] )}

        #[cfg( feature="unstable" )]
        impl_trait!{ std::iter::TrustedLen _for!( Enum![17..=32] )}

        #[cfg( feature="unstable" )]
        impl_trait!{ _impl!(R) std::ops::Generator<R> _for!( Enum![17..=32] )}
    }
}

/// Since `enum`s in Rust do not have prototypes, this mod does the work.
pub mod proto {
    use crate as enumx;
    use crate::def_impls;

    def_impls! {
        pub enum __![ 0..=16 ];
    }

    #[cfg( feature="enum32" )]
    def_impls! {
        pub enum __![ 17..=32 ];
    }
}

macro_rules! impl_exchange_from {
    ( $($index:tt)+ ) => {
        $(
            impl<Enum,Variant> ExchangeFrom<Variant,[(); $index]> for Enum
                where Enum: FromVariant<Variant,[(); $index]>
            {
                fn exchange_from( variant: Variant ) -> Self {
                    Enum::from_variant( variant )
                }
            }
        )+
    };
}

impl_exchange_from!( 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 );

/// Wrapper for non-path types in type pattern matching using `#[ty_pat]` match
/// ```rust,no_run
/// use enumx::export::*;
/// use enumx::predefined::*;
///
/// #[enumx] fn bar( input: Enum!(&'static str,i32) ) {
///     #[ty_pat] match input {
///         TyPat::<&'static str>(s) => println!( "it's static str:{}", s ),
///         i32(i) => println!( "it's i32:{}", i ),
///     }
/// }
/// ```
pub type TyPat<T> = T;

#[cfg( test )]
mod tests {
    mod test_unnamed {
        use crate::*;
        use crate::predefined::*;

        #[test]
        fn test_from_variant() {
            let enum1 = Enum1::<i32>::from_variant( 2018 );
            assert_eq!( enum1, Enum1::_0( 2018 ));

            let enum2 = Enum2::<i32, String>::from_variant( "rust".to_string() );
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));

            let enum3 = Enum3::<i32, String, bool>::from_variant( true );
            assert_eq!( enum3, Enum3::_2( true ));
        }

        #[test]
        fn test_into_enum() {
            let enum1: Enum1<i32> = 2018.exchange_into();
            assert_eq!( enum1, Enum1::_0( 2018 ));

            let enum2: Enum2<i32, String> = "rust".to_string().exchange_into();
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));

            let enum3: Enum3<i32, String, bool> = true.exchange_into();
            assert_eq!( enum3, Enum3::_2( true ));
        }

        #[test]
        fn test_exchange_from() {
            let enum1 = Enum1::<String>::exchange_from( "rust".to_string() );

            let enum1 = Enum1::<String>::exchange_from( enum1 );
            assert_eq!( enum1, Enum1::_0( "rust".to_string() ));

            let enum2 = Enum2::<i32, String>::exchange_from( enum1 );
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));

            let enum2 = Enum2::<String, i32>::exchange_from( enum2 );
            assert_eq!( enum2, Enum2::_0("rust".to_string() ));

            let enum3 = Enum3::<bool, i32, String>::exchange_from( enum2 );
            assert_eq!( enum3, Enum3::_2( "rust".to_string() ));

            let enum3 = Enum3::<String, i32, bool>::exchange_from( enum3 );
            assert_eq!( enum3, Enum3::_0( "rust".to_string() ));
        }

        #[test]
        fn test_exchange_into() {
            let enum1 = Enum1::<i32>::exchange_from( 2018 );

            let enum1: Enum1<i32> = enum1.exchange_into();
            assert_eq!( enum1, Enum1::_0( 2018 ));

            let enum2: Enum2<String, i32> = enum1.exchange_into();
            assert_eq!( enum2, Enum2::_1( 2018 ));

            let enum2: Enum2<i32, String> = enum2.exchange_into();
            assert_eq!( enum2, Enum2::_0( 2018 ));

            let enum3: Enum3<bool, String, i32> = enum2.exchange_into();
            assert_eq!( enum3, Enum3::_2( 2018 ));

            let enum3: Enum3<i32, String, bool> = enum3.exchange_into();
            assert_eq!( enum3, Enum3::_0( 2018 ));
        }
    }

    mod test_named {
        use crate::*;
        use crate::predefined::*;
        use crate as enumx;

        #[derive( Exchange, Clone, Debug, PartialEq, Eq, PartialOrd, Ord )]
        enum One<T> {
            The(T),
        }

        #[derive( Exchange, Clone, Debug, PartialEq, Eq, PartialOrd, Ord )]
        enum Two<A, B> {
            Former(A),
            Latter(B),
        }

        #[derive( Exchange, Clone, Debug, PartialEq, Eq, PartialOrd, Ord )]
        enum Three<A, B, C> {
            First(A),
            Second(B),
            Third(C),
        }

        #[test]
        fn test_from_variant() {
            let one = One::<i32>::from_variant( 2018 );
            assert_eq!( one, One::The( 2018 ));

            let two = Two::<i32, String>::from_variant( "rust".to_string() );
            assert_eq!( two, Two::Latter( "rust".to_string() ));

            let three = Three::<i32, String, bool>::from_variant( true );
            assert_eq!( three, Three::Third( true ));
        }

        #[test]
        fn test_into_enum() {
            let one: One<i32> = 2018.into_enum();
            assert_eq!( one, One::The( 2018 ));

            let two: Two<i32, String> = "rust".to_string().into_enum();
            assert_eq!( two, Two::Latter( "rust".to_string() ));

            let three: Three<i32, String, bool> = true.into_enum();
            assert_eq!( three, Three::Third( true ));
        }

        #[test]
        fn test_exchange_from() {
            let one = One::<i32>::exchange_from( 2018 );
            let enum1 = Enum1::<i32>::exchange_from( one );
            let one = One::<i32>::exchange_from( enum1 );

            let one = One::<i32>::exchange_from( one );
            assert_eq!( one, One::The( 2018 ));

            let two = Two::<String, i32>::exchange_from( one );
            assert_eq!( two, Two::Latter( 2018 ));

            let two = Two::<i32, String>::exchange_from( two );
            assert_eq!( two, Two::Former( 2018 ));

            let three = Three::<bool, String, i32>::exchange_from( two );
            assert_eq!( three, Three::Third( 2018 ));

            let three = Three::<i32, String, bool>::exchange_from( three );
            assert_eq!( three, Three::First( 2018 ));
        }

        #[test]
        fn test_exchange_into() {
            let one = One::<String>::exchange_from( "rust".to_string() );

            let one: One<String> = one.exchange_into();
            assert_eq!( one, One::The( "rust".to_string() ));

            let two: Two<i32, String> = one.exchange_into();
            assert_eq!( two, Two::Latter( "rust".to_string() ));

            let two: Two<String, i32> = two.exchange_into();
            assert_eq!( two, Two::Former( "rust".to_string() ));

            let three: Three<bool, i32, String> = two.exchange_into();
            assert_eq!( three, Three::Third( "rust".to_string() ));

            let three: Three<String, i32, bool> = three.exchange_into();
            assert_eq!( three, Three::First( "rust".to_string() ));
        }

        #[test]
        fn test_adhoc_from_named() {
            let three = Three::<bool, String, i32>::exchange_from( 2018 );
            let enum3 = Enum3::<String, i32, bool>::exchange_from( three );
            assert_eq!( enum3, Enum3::_1( 2018 ));
        }

        #[test]
        fn test_adhoc_into_named() {
            let enum3 = Enum3::<String, i32, bool>::exchange_from( 2018 );
            let three: Three<bool, String, i32> = enum3.exchange_into();
            assert_eq!( three, Three::Third( 2018 ));
        }

        #[test]
        fn test_named_into_adhoc() {
            let three = Three::<bool, String, i32>::exchange_from( 2018 );
            let enum3: Enum3<String, i32, bool> = three.exchange_into();
            assert_eq!( enum3, Enum3::_1( 2018 ));
        }

        #[test]
        fn test_named_from_adhoc() {
            let enum3 = Enum3::<String, i32, bool>::exchange_from( 2018 );
            let three = Three::<bool, String, i32>::exchange_from( enum3 );
            assert_eq!( three, Three::Third( 2018 ));
        }
    }
}
