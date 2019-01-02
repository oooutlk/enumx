// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! Structural enums implemented in enum exchange.

// Kind
pub struct AA; // Anonymous to Anonymous
pub struct AN; // Anonymous to Named
pub struct NA; // Named to Anonymous
pub struct NN; // Named to Named
pub struct VA; // Variant to Anonymous
pub struct VN; // Variant to Named

/// Constructs an enum from one of its variant type.
pub trait FromVariant<Variant,Index,Kind> {
    fn from_variant( variant: Variant ) -> Self;
}

/// Converts an exchangeable enum into another one.
pub trait IntoEnum<Enum,Index,Kind> {
    fn into_enum( self ) -> Enum;
}

impl<Enum,Variant,Index,Kind> IntoEnum<Enum,Index,Kind> for Variant
    where Enum: FromVariant<Variant,Index,Kind>
{
    fn into_enum( self ) -> Enum {
        FromVariant::<Variant,Index,Kind>::from_variant( self )
    }
}

/// Constructs an exchangeable enum from another one.
pub trait ExchangeFrom<Src,Indices,Kind> {
    fn exchange_from( src: Src ) -> Self;
}

impl<Src,Dest,Proto,Indices> ExchangeFrom<Src,Indices,NA> for Dest
    where Src  : Exchange<Proto=Proto>
        , Dest : ExchangeFrom<Proto,Indices,AA>
{
    fn exchange_from( src: Src ) -> Self {
        Dest::exchange_from( src.into_proto() )
    }
}

/// Converts an exchangeable enum into another one.
pub trait ExchangeInto<Dest,Indices,Kind> {
    fn exchange_into( self ) -> Dest;
}

impl<Src,Dest,Indices> ExchangeInto<Dest,Indices,AA> for Src
    where Dest: ExchangeFrom<Src,Indices,AA>
{
    fn exchange_into( self ) -> Dest {
        ExchangeFrom::<Src,Indices,AA>::exchange_from( self )
    }
}

impl<Src,Dest,Proto,Indices> ExchangeInto<Dest,Indices,AN> for Src
    where Dest : Exchange<Proto=Proto>
        , Src  : ExchangeInto<Proto,Indices,AA>
{
    fn exchange_into( self ) -> Dest {
        Dest::from_proto( self.exchange_into() )
    }
}

/// Recursive descent indices
pub struct LR<L,R> ( pub L, pub R );

/// Index of the first variant
pub struct V0;

/// Indicates an impossible index for `Enum0`, internal use only.
pub struct Nil;

/// Never type
pub enum Enum0 {}

#[derive( Debug, PartialEq, Eq, PartialOrd, Ord )]
pub enum Enum1<T0> { _0(T0) }

impl<T0> FromVariant<T0,V0,VA> for Enum1<T0> {
    fn from_variant( variant: T0 ) -> Self {
        Enum1::_0( variant )
    }
}

impl<T0> ExchangeFrom<Enum0,Nil,AA> for Enum1<T0> {
    fn exchange_from( src: Enum0 ) -> Self { match src {} }
}

impl<T0> ExchangeFrom<Enum1<T0>,V0,AA> for Enum1<T0> {
    fn exchange_from( src: Enum1<T0> ) -> Self {
        match src {
            Enum1::_0(v) => Enum1::_0(v)
        }
    }
}

include!( concat!( env!( "OUT_DIR" ), "/predefined.rs" ));

/// Indicates the prototype for a user-defined exchangeable enum.
pub trait Exchange {
    type Proto;
    fn from_proto( src: Self::Proto ) -> Self;
    fn into_proto( self ) -> Self::Proto;
}

impl<Variant,Enum,Proto,Index> FromVariant<Variant,Index,VN> for Enum
    where Self  : Exchange<Proto=Proto>
        , Proto : FromVariant<Variant,Index,VA>
{
    fn from_variant( variant: Variant ) -> Self {
        Enum::from_proto( Proto::from_variant( variant ))
    }
}

impl<Src,Dest,Proto,Indices> ExchangeFrom<Src,Indices,AN> for Dest
    where Self  : Exchange<Proto=Proto>
        , Proto : ExchangeFrom<Src,Indices,AA>
{
    fn exchange_from( src: Src ) -> Self {
        Dest::from_proto( Proto::exchange_from( src ))
    }
}

impl<Src,Dest,Proto,Indices> ExchangeInto<Dest,Indices,NA> for Src
    where Self  : Exchange<Proto=Proto>
        , Proto : ExchangeInto<Dest,Indices,AA>
{
    fn exchange_into( self ) -> Dest {
        self.into_proto().exchange_into()
    }
}

impl<Src,Dest,Proto,Indices> ExchangeFrom<Src,Indices,NN> for Dest
    where Self  : Exchange<Proto=Proto>
        , Proto : ExchangeFrom<Src,Indices,NA>
{
    fn exchange_from( src: Src ) -> Self {
        Dest::from_proto( Proto::exchange_from( src ))
    }
}

impl<Src,Dest,Proto,Indices> ExchangeInto<Dest,Indices,NN> for Src
    where Self  : Exchange<Proto=Proto>
        , Proto : ExchangeInto<Dest,Indices,AN>
{
    fn exchange_into( self ) -> Dest {
        self.into_proto().exchange_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_unnamed {
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
        fn test_exchange_from() {
            let enum1 = Enum1::<String>::from_variant( "rust".to_string() );

            let enum1 = Enum1::<String>::exchange_from( enum1 );
            assert_eq!( enum1, Enum1::_0( "rust".to_string() ));

            let enum2 = Enum2::<i32,String>::exchange_from( enum1 );
            assert_eq!( enum2, Enum2::_1( "rust".to_string() ));

            let enum2 = Enum2::<String,i32>::exchange_from( enum2 );
            assert_eq!( enum2, Enum2::_0( "rust".to_string() ));

            let enum3 = Enum3::<bool,i32,String>::exchange_from( enum2 );
            assert_eq!( enum3, Enum3::_2( "rust".to_string() ));

            let enum3 = Enum3::<String,i32,bool>::exchange_from( enum3 );
            assert_eq!( enum3, Enum3::_0( "rust".to_string() ));
        }

        #[test]
        fn test_exchange_into() {
            let enum1 = Enum1::<i32>::from_variant( 2018 );

            let enum1: Enum1<i32> = enum1.exchange_into();
            assert_eq!( enum1, Enum1::_0( 2018 ));

            let enum2: Enum2<String,i32> = enum1.exchange_into();
            assert_eq!( enum2, Enum2::_1( 2018 ));

            let enum2: Enum2<i32,String> = enum2.exchange_into();
            assert_eq!( enum2, Enum2::_0( 2018 ));

            let enum3: Enum3<bool,String,i32> = enum2.exchange_into();
            assert_eq!( enum3, Enum3::_2( 2018 ));

            let enum3: Enum3<i32,String,bool> = enum3.exchange_into();
            assert_eq!( enum3, Enum3::_0( 2018 ));
        }
    }

    mod test_named {
        mod enumx { pub use crate::Exchange; }

        use super::*;
        use enumx_derive::Exchange;

        #[derive( Exchange, Debug, PartialEq, Eq, PartialOrd, Ord )]
        enum One<T> { The(T) }

        #[derive( Exchange, Debug, PartialEq, Eq, PartialOrd, Ord )]
        enum Two<A,B> { Former(A), Latter(B) }
   
        #[derive( Exchange, Debug, PartialEq, Eq, PartialOrd, Ord )]
        enum Three<A,B,C> { First(A), Second(B), Third(C), }

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
        fn test_exchange_from() {
            let one = One::<i32>::from_variant( 2018 );
            let enum1 = Enum1::<i32>::exchange_from( one );
            let one = One::<i32>::exchange_from( enum1 );

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
        fn test_adhoc_from_named() {
            let three = Three::<bool,String,i32>::from_variant( 2018 );
            let enum3 = Enum3::<String,i32,bool>::exchange_from( three );
            assert_eq!( enum3, Enum3::_1( 2018 ));
        }

        #[test]
        fn test_adhoc_into_named() {
            let enum3 = Enum3::<String,i32,bool>::from_variant( 2018 );
            let three: Three<bool,String,i32> = enum3.exchange_into();
            assert_eq!( three, Three::Third( 2018 ));
        }

        #[test]
        fn test_named_into_adhoc() {
            let three = Three::<bool,String,i32>::from_variant( 2018 );
            let enum3: Enum3<String,i32,bool> = three.exchange_into();
            assert_eq!( enum3, Enum3::_1( 2018 ));
        }

        #[test]
        fn test_named_from_adhoc() {
            let enum3 = Enum3::<String,i32,bool>::from_variant( 2018 );
            let three = Three::<bool,String,i32>::exchange_from( enum3 );
            assert_eq!( three, Three::Third( 2018 ));
        }
    }
}
