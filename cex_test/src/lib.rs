#![cfg(test)]

use cex::*;
use cex_derive::*;

#[test]
fn test_cex_fn() {
    #[cex]
    fn f2( quit: usize ) -> Result<(), Ce2<String,i32>> {
        match quit {
            0 => Err("runtime error".to_string())?,
            1 => return Err(0xdead),
            _ => Ok(()),
        }
    }
    assert_eq!( f2(0), Err( Ce2::_0( "runtime error".to_string() ))); 
    assert_eq!( f2(1), Err( Ce2::_1( 0xdead )));
    assert_eq!( f2(2), Ok(()) );

    #[cex]
    fn f3( quit: usize ) -> Result<(), Ce3<String,i32,bool>> {
        match quit {
            0 | 1 => Ok(f2(quit)?),
            2 => Err(false)?,
            _ => Ok(()),
        }
    }
    assert_eq!( f3(0), Err( Ce3::_0( "runtime error".to_string() )));
    assert_eq!( f3(1), Err( Ce3::_1( 0xdead )));
    assert_eq!( f3(2), Err( Ce3::_2( false )));
    assert_eq!( f3(3), Ok(()) );

    fn g( quit: usize ) -> Result<(), String> {
        match f3( quit ) {
            Ok(_) => Ok(()),
            Err( err ) => {
                match err.error {
                    Enum3::_0( string ) => Err( string ),
                    Enum3::_1( errno  ) => Err( format!( "errno: 0x{:x}", errno )),
                    Enum3::_2( flag   ) => Err( format!( "flag: {}", flag )),
                }
            },
        }
    }
    assert_eq!( g(0), Err( "runtime error".to_string() ));
    assert_eq!( g(1), Err( "errno: 0xdead".to_string() ));
    assert_eq!( g(2), Err( "flag: false".to_string() ));
    assert_eq!( g(3), Ok(()));
}

#[test]
fn test_enum_fn() {
    #[cex]
    fn f2( quit: usize ) -> Result<(), Enum2<String,i32>> {
        match quit {
            0 => Err("runtime error".to_string())?,
            1 => return Err(0xdead),
            _ => Ok(()),
        }
    }

    assert_eq!( f2(0), Err( Enum2::_0( "runtime error".to_string() ))); 
    assert_eq!( f2(1), Err( Enum2::_1( 0xdead )));
    assert_eq!( f2(2), Ok(()) );

    #[cex]
    fn f3( quit: usize ) -> Result<(), Enum3<String,i32,bool>> {
        match quit {
            0 | 1 => Ok(f2(quit)?),
            2 => Err(false)?,
            _ => Ok(()),
        }
    }
    assert_eq!( f3(0), Err( Enum3::_0( "runtime error".to_string() )));
    assert_eq!( f3(1), Err( Enum3::_1( 0xdead )));
    assert_eq!( f3(2), Err( Enum3::_2( false )));
    assert_eq!( f3(3), Ok(()) );

    #[cex]
    fn g3( variant_index: usize ) -> Result<(), Enum3<i32,bool,String>> {
        match variant_index {
            0|1 => Ok( f2( variant_index )? ),
              _ => return f3( variant_index ),
        }
    }
    assert_eq!( g3(0), Err( Enum3::_2( "runtime error".to_string() )));
    assert_eq!( g3(1), Err( Enum3::_0( 0xdead )));
    assert_eq!( g3(2), Err( Enum3::_1( false )));

    fn h( quit: usize ) -> Result<(), String> {
        match f3( quit ) {
            Ok(_) => Ok(()),
            Err( err ) => {
                match err {
                    Enum3::_0( string ) => Err( string ),
                    Enum3::_1( errno  ) => Err( format!( "errno: 0x{:x}", errno )),
                    Enum3::_2( flag   ) => Err( format!( "flag: {}", flag )),
                }
            },
        }
    }
    assert_eq!( h(0), Err( "runtime error".to_string() ));
    assert_eq!( h(1), Err( "errno: 0xdead".to_string() ));
    assert_eq!( h(2), Err( "flag: false".to_string() ));
    assert_eq!( h(3), Ok(()));
}

#[test]
fn print_backtrace() {
    #[cex]
    fn foo() -> Result<(),Ce1<()>> {
        Err(())?
    }

    #[cex]
    fn bar() -> Result<(),Ce1<()>> {
        return foo();
    }

    if let Err(err) = bar() {
        eprintln!( "{:#?}", err.backtrace );
    }
}
