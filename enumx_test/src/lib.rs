#![cfg(test)]

use enumx::*;
use enumx_derive::*;

#[test]
fn test_enumx_fn() {
    #[enumx]
    fn f2( variant_index: usize ) -> Enum2<String,i32> {
        match variant_index {
            0 => "runtime error".to_string().into(),
            1 => 0xdead.into(),
            _ => panic!( "variant index out of bounds" ),
        }
    }

    assert_eq!( f2(0), Enum2::_0( "runtime error".to_string() ));
    assert_eq!( f2(1), Enum2::_1( 0xdead ));

    #[enumx]
    fn f3( variant_index: usize ) -> Enum3<String,i32,bool> {
        match variant_index {
            0|1 => f2(variant_index).into(),
            2 => false.into(),
            _ => panic!( "variant index out of bounds" ),
        }
    }
    assert_eq!( f3(0), Enum3::_0( "runtime error".to_string() ));
    assert_eq!( f3(1), Enum3::_1( 0xdead ));
    assert_eq!( f3(2), Enum3::_2( false ));

    #[enumx]
    fn g3( variant_index: usize ) -> Enum3<i32,bool,String> {
        match variant_index {
            0|1 => f2( variant_index ).into(),
              _ => f3( variant_index ).into(),
        }
    }
    assert_eq!( g3(0), Enum3::_2( "runtime error".to_string() ));
    assert_eq!( g3(1), Enum3::_0( 0xdead ));
    assert_eq!( g3(2), Enum3::_1( false ));

    fn h( variant_index: usize ) -> String {
        match f3( variant_index ) {
            Enum3::_0( string ) => string.into(),
            Enum3::_1( errno  ) => format!( "errno: 0x{:x}", errno ),
            Enum3::_2( flag   ) => format!( "flag: {}", flag ),
        }
    }
    assert_eq!( h(0), "runtime error".to_string() );
    assert_eq!( h(1), "errno: 0xdead".to_string() );
    assert_eq!( h(2), "flag: false".to_string() );
}
