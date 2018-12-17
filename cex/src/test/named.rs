use super::*;

use enumx_derive::Exchange;

#[ derive( Exchange, Debug )]
enum ReadU32Error {
    IO( std::io::Error ),
    Parse( std::num::ParseIntError ),
}

fn read_u32( filename: &'static str )
    -> Result<u32, Cex<ReadU32Error>>
{
    use std::io::Read;

    let mut f = std::fs::File::open( filename ).may_throw()?;
    let mut s = String::new();
    f.read_to_string( &mut s ).may_throw_log( log!() )?;
    let number = s.trim().parse::<u32>()
                  .may_throw_log( log!( "fail in parsing {} to u32", s.trim() ))?;
    Ok( number )
}

#[derive( Debug, PartialEq, Eq )]
struct MulOverflow( u32, u32 );

#[ derive( Exchange, Debug )]
enum AMulBEqCError {
    IO( std::io::Error ),
    Parse( std::num::ParseIntError ),
    Overflow( MulOverflow ),
}

fn a_mul_b_eq_c( file_a: &'static str, file_b: &'static str, file_c : &'static str )
    -> Result<bool, Cex<AMulBEqCError>>
{
    let a = read_u32( file_a ).may_rethrow_ex()?;

    let b = match read_u32( file_b ) {
        Ok(  value ) => value,
        Err( cex   ) => {
            if a == 0 {
                0 // 0 * b == 0, no matter what b is.
            } else {
                rethrow_log_ex!( cex );
            }
        },
    };
 
    let c = match read_u32( file_c ) {
        Ok(  value ) => value,
        Err( cex   ) => match cex.error {
            ReadU32Error::IO(    _   ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( err ) => throw!( err ),
        },
    };

    a.checked_mul( b )
     .ok_or( MulOverflow(a,b) )
     .may_throw_log( log!( "u32 overflow: {} * {}", a, b ))
     .map( |result| result == c )
}

#[test]
fn test_mix_named_with_adhoc() {
    fn read_u32_throws_adhoc( filename: &'static str )
        -> Result<u32, Cex<Enum!( std::io::Error, std::num::ParseIntError )>>
    {
        read_u32( filename ).may_rethrow()
    }
    assert!( read_u32_throws_adhoc("src/test/no_file").is_err() );

    fn read_u32_throws_named( filename: &'static str )
        -> Result<u32, Cex<AMulBEqCError>>
    {
        read_u32_throws_adhoc( filename ).may_rethrow_named()
    }
    assert!( read_u32_throws_named("src/test/no_file").is_err() );
}

#[test]
fn test_read_u32() {
    assert!( read_u32("src/test/no_file").map_err( |cex|
        if let ReadU32Error::IO(_) = cex.error { true } else { false }
    ).unwrap_err() );

    assert!( read_u32("src/test/not_num").map_err( |cex| {
        if let ReadU32Error::Parse(_) = cex.error { true } else { false }
    }).unwrap_err() );

    assert_eq!( read_u32("src/test/3").ok().unwrap(), 3 );
}

#[test]
fn test_a_mul_b_eq_c() {
    assert!(
        a_mul_b_eq_c( "src/test/no_file", "src/test/7", "src/test/21"
        ).map_err( |cex|
            if let AMulBEqCError::IO(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/not_num", "src/test/7", "src/test/21"
        ).map_err( |cex|
            if let AMulBEqCError::Parse(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/no_file", "src/test/21"
        ).map_err( |cex|
            if let AMulBEqCError::IO(_) = cex.error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21"
        ).map_err( |cex|
            if let AMulBEqCError::Parse(_) = cex.error { true } else { false }
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
            module: "cex::test::named",
            file: "cex/src/test/named.rs",
            line: 20,
            column: 35,
            info: Some(
                "fail in parsing not-a-number to u32"
            )
        },
        Log {
            module: "cex::test::named",
            file: "cex/src/test/named.rs",
            line: 45,
            column: 17,
            info: None
        }
    ]
}"# ))
    );
}
