use super::*;
type Log<E> = super::Log<E, Env<Vec<Frame>>>;

use cex_derive::Logger;
mod cex { pub use crate::{Frame,Logger}; }

#[derive( EnumX, Logger, Debug )]
enum ReadU32Error {
    IO(    Log<std::io::Error> ),
    Parse( Log<std::num::ParseIntError> ),
}

fn read_u32( filename: &'static str ) -> Result<u32,ReadU32Error> {
    use std::io::Read;

    let mut f = std::fs::File::open( filename )
        .map_err_to_log(( LogLevel::Debug, frame!() ))
        .map_error()?;

    let mut s = String::new();
    f.read_to_string( &mut s )
        .map_err_to_log(( LogLevel::Debug, frame!() ))
        .map_error()?;

    let number = s.trim().parse::<u32>()
        .map_err_to_log(( LogLevel::Debug, frame!() ))
        .map_error()?;

    Ok( number )
}

#[derive( Debug, PartialEq, Eq )]
struct MulOverflow( u32, u32 );

#[derive( EnumX, Logger, Debug )]
enum AMulBEqCError {
    IO(       Log<std::io::Error> ),
    Parse(    Log<std::num::ParseIntError> ),
    Overflow( Log<MulOverflow> ),
}

fn a_mul_b_eq_c( file_a: &'static str, file_b: &'static str, file_c: &'static str ) -> Result<bool, AMulBEqCError> {
    let a = read_u32( file_a ).map_err_log(( LogLevel::Debug, frame!() )).map_error()?;

    let b = match read_u32( file_b ) {
        Ok(  value ) => value,
        Err( err ) => {
            if a == 0 {
                0 // 0 * b == 0, no matter what b is.
            } else {
                return err.log(( LogLevel::Debug, frame!() )).error();
            }
        },
    };
 
    let c = match read_u32( file_c ) {
        Ok(  value ) => value,
        Err( err   ) => match err {
            ReadU32Error::IO(    _ ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( e ) => return e.log(( LogLevel::Debug, frame!() )).error(),
        },
    };

    a.checked_mul( b )
     .ok_or( MulOverflow(a,b) )
     .map_err_to_log(( LogLevel::Debug, frame!() ))
     .map_error()
     .map( |result| result == c )
}

#[test]
fn test_read_u32() {
    assert!( read_u32("src/test/no_file").map_err( |err|
        if let ReadU32Error::IO(_) = err { true } else { false }
    ).unwrap_err() );

    assert!( read_u32("src/test/not_num").map_err( |err| {
        if let ReadU32Error::Parse(_) = err { true } else { false }
    }).unwrap_err() );

    assert_eq!( read_u32("src/test/3").ok().unwrap(), 3 );
}

#[test]
fn test_a_mul_b_eq_c() {
    assert!(
        a_mul_b_eq_c( "src/test/no_file", "src/test/7", "src/test/21"
        ).map_err( |err|
            if let AMulBEqCError::IO(_) = err { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/not_num", "src/test/7", "src/test/21"
        ).map_err( |err|
            if let AMulBEqCError::Parse(_) = err { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/no_file", "src/test/21"
        ).map_err( |err|
            if let AMulBEqCError::IO(_) = err { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21"
        ).map_err( |err|
            if let AMulBEqCError::Parse(_) = err { true } else { false }
        ).unwrap_err() );

    assert!( a_mul_b_eq_c( "src/test/3", "src/test/0",       "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/0", "src/test/no_file", "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/0", "src/test/not_num", "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/3", "src/test/7",       "src/test/21" ).ok().unwrap() );
}

#[test]
fn test_backtrace() {
    assert_eq!( a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21" ).map_err( |err| format!( "{:#?}", err )),
                Err( String::from( r#"Parse(
    Log {
        inner: ParseIntError {
            kind: InvalidDigit
        },
        agent: Env(
            []
        )
    }
)"#
                ))
    );
}
