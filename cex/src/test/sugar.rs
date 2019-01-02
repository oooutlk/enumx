use super::*;

use cex_derive::cex;

cex! {
    fn read_u32( filename: &'static str ) -> u32
        throws IO(    std::io::Error )
             , Parse( std::num::ParseIntError )
    {
        use std::io::Read;

        let mut f = std::fs::File::open( filename )~?;
        let mut s = String::new();
        f.read_to_string( &mut s )~?;
        let number = s.trim().parse::<u32>()
                      .may_throw_log( log!( "fail in parsing {} to u32", s.trim() ))?;
        Ok( number )
    }
}

#[derive( Debug, PartialEq, Eq )]
pub struct MulOverflow( pub u32, pub u32 );

cex!{
    fn a_mul_b_eq_c(
        file_a: &'static str,
        file_b: &'static str,
        file_c: &'static str )
        -> bool
        throws IO(    std::io::Error )
             , Parse( std::num::ParseIntError )
             , Calc(  MulOverflow )
    {
        let a = read_u32( file_a )~~?;

        let b = match read_u32( file_b ) {
            Ok(  value ) => value,
            Err( cex   ) => {
                if a == 0 {
                    0 // 0 * b == 0, no matter what b is.
                } else {
                    rethrow_log!( cex );
                }
            },
        };
 
        let c = match read_u32( file_c ) {
            Ok(  value ) => value,
            Err( cex   ) => match cex.error {
                read_u32::Err::IO( _      ) => 0, // default to 0 if file is missing.
                read_u32::Err::Parse( err ) => throw!( err ),
            },
        };

        a.checked_mul( b )
         .ok_or( MulOverflow(a,b) )
         .may_throw_log( log!( "u32 overflow: {} * {}", a, b ))
         .map( |result| result == c )
    }
}

#[test]
fn test_read_u32() {
    assert!( read_u32("src/test/no_file").map_err( |cex|
        if let read_u32::Err::IO(_) = cex.error { true } else { false }
    ).unwrap_err() );

    assert!( read_u32("src/test/not_num").map_err( |cex| {
        if let read_u32::Err::Parse(_) = cex.error { true } else { false }
    }).unwrap_err() );

    assert_eq!( read_u32("src/test/3").ok().unwrap(), 3 );
}

#[test]
fn test_a_mul_b_eq_c() {
    assert!(
        a_mul_b_eq_c( "src/test/no_file", "src/test/7", "src/test/21"
        ).map_err( |cex|
            if let a_mul_b_eq_c::Err::IO(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/not_num", "src/test/7", "src/test/21"
        ).map_err( |cex|
            if let a_mul_b_eq_c::Err::Parse(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/no_file", "src/test/21"
        ).map_err( |cex|
            if let a_mul_b_eq_c::Err::IO(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21"
        ).map_err( |cex|
            if let a_mul_b_eq_c::Err::Parse(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!( a_mul_b_eq_c( "src/test/3", "src/test/0",       "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/0", "src/test/no_file", "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/0", "src/test/not_num", "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/3", "src/test/7",       "src/test/21" ).ok().unwrap() );
}

#[test]
fn test_backtrace() {
    assert_eq!( a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21" ).map_err( |cex| format!( "{:#?}", cex )),
                Err( String::from( r#"Cex {
    error: Parse(
        ParseIntError {
            kind: InvalidDigit
        }
    ),
    logs: [
        Log {
            module: "cex::test::sugar",
            file: "cex/src/test/sugar.rs",
            line: 5,
            column: 1,
            info: Some(
                "fail in parsing not-a-number to u32"
            )
        },
        Log {
            module: "cex::test::sugar",
            file: "cex/src/test/sugar.rs",
            line: 24,
            column: 1,
            info: None
        }
    ]
}"#
                ))
    );
}
