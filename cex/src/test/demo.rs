use super::*;

use cex_derive::{cex,Logger};
mod cex { pub use crate::{Frame,Logger}; }

#[derive( EnumX, Logger, Debug )]
enum ReadU32Error {
    IO(    Log<std::io::Error> ),
    Parse( Log<std::num::ParseIntError> ),
}

#[cex(to_log)]
fn read_u32( filename: &'static str )
    -> Result<u32, ReadU32Error>
{
    use std::io::Read;

    let mut f = std::fs::File::open( filename )?;
    let mut s = String::new();
    f.read_to_string( &mut s )?;
    let number = s.trim().parse::<u32>()?;
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

#[cex(log)]
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
            ReadU32Error::IO(    _ ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( e ) => return e.error(),
        },
    };

    Ok( a.checked_mul( b )
        .ok_or( MulOverflow(a,b) )
        .map( |result| result == c )
        .map_err_to_log( frame!() )
    ? )
}

#[test]
fn test_backtrace() {
    assert_eq!( a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21" ).map_err( |err| format!( "{:#?}", err )),
                Err( String::from( r#"Parse(
    Log {
        inner: ParseIntError {
            kind: InvalidDigit
        },
        agent: [
            Frame {
                module: "cex::test::demo",
                file: "cex/src/test/demo.rs",
                line: 12,
                column: 1,
                info: None
            }
        ]
    }
)"#
                ))
    );
}
