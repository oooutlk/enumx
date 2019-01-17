use super::*;

use cex_derive::cex;

#[derive( EnumX, Debug )]
enum ReadU32Error {
    Parse( std::num::ParseIntError ),
    Other( DynErr ),
}

#[cex]
fn read_u32( filename: &'static str )
    -> Result<u32,ReadU32Error>
{
    use std::io::Read;

    let mut f = std::fs::File::open( filename ).map_dyn_err()?;

    let mut s = String::new();
    f.read_to_string( &mut s ).map_dyn_err()?;
    let number = s.trim().parse::<u32>()?;
    Ok( number )
}

#[derive( Debug, PartialEq, Eq )]
struct MulOverflow( u32, u32 );

#[derive( EnumX, Debug )]
enum AMulBEqCError {
    Parse( std::num::ParseIntError ),
    Overflow( MulOverflow ),
    Other( DynErr ),
}

#[cex]
fn a_mul_b_eq_c( file_a: &'static str, file_b: &'static str, file_c: &'static str )
    -> Result<bool, AMulBEqCError>
{
    let a = read_u32( file_a )?;

    let b = match read_u32( file_b ) {
        Ok(  value ) => value,
        Err( err ) => {
            if a == 0 {
                0 // 0 * b == 0, no matter what b is.
            } else {
                return err.error();
            }
        },
    };
 
    let c = match read_u32( file_c ) {
        Ok(  value ) => value,
        Err( err   ) => match err {
            ReadU32Error::Other( _ ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( e ) => return e.error(),
        },
    };

    a.checked_mul( b )
     .ok_or( MulOverflow(a,b) )
     .map( |result| result == c )
     .map_error()
}

#[test]
fn test_read_u32() {
    assert!( read_u32("src/test/no_file").map_err( |err|
        if let ReadU32Error::Other(_) = err { true } else { false }
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
            if let AMulBEqCError::Other(_) = err { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/not_num", "src/test/7", "src/test/21"
        ).map_err( |err|
            if let AMulBEqCError::Parse(_) = err { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/no_file", "src/test/21"
        ).map_err( |err|
            if let AMulBEqCError::Other(_) = err { true } else { false }
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
