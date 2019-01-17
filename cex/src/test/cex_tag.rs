use super::*;

use cex_derive::Logger;
mod cex { pub use crate::{Frame,Logger}; }

#[derive( EnumX, Debug )]
enum ReadU32Error {
    IO( std::io::Error ),
    Parse( std::num::ParseIntError ),
}

#[cex]
fn read_u32( filename: &'static str )
    -> Result<u32, ReadU32Error>
{
    use std::io::Read;

    let mut f = std::fs::File::open( filename )?;
    let mut s = String::new();
    let read_op: Result<(),std::io::Error> = try {
        f.read_to_string( &mut s )?; // This `?` is not effected
    };
    match read_op {
        Ok(_) => return Ok( s.trim().parse::<u32>()? ),
        Err(err) => return err.error(),
    }
}

#[derive( Debug, PartialEq, Eq )]
struct MulOverflow( u32, u32 );

#[derive( EnumX, Debug )]
enum AMulBEqCError {
    IO( std::io::Error ),
    Parse( std::num::ParseIntError ),
    Overflow( MulOverflow ),
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
            ReadU32Error::IO(    _ ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( e ) => return e.error(),
        },
    };

    a.checked_mul( b )
     .ok_or( MulOverflow(a,b) )
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

use cex_derive::cex;

#[derive( EnumX, Debug, PartialEq, Eq )]
enum CexErr {
    Code(i32),
    Text(&'static str),
}

#[cex]
fn misc() -> Result<(),CexErr> {
    fn bar() -> Result<(),i32> { Err(42)? }
    let _bar = || -> Result<(),i32> { Ok( bar()? )};
    let _bar: Result<(),i32> = try { Err(42)? };

    #[cex] fn _baz() -> Result<(),CexErr> { Err(42)? }
    let _baz = #[cex] || -> Result<(),CexErr> { Ok( bar()? )};
    let _baz: Result<(),CexErr> = #[cex] try { Err(42)? };

    Err(42)?
}

#[test]
fn test_misc() {
    assert_eq!( misc(), Err( CexErr::Code( 42 )));
}

mod log_frame {
    use super::*;

    #[derive( EnumX, Logger, Debug, PartialEq, Eq )]
    enum CexErr {
        Code( Log<i32> ),
        Text( Log<&'static str> ),
    }
    
    #[cex(to_log)]
    fn _cex_to_log() -> Result<(),CexErr> { Err(42)? }
    
    #[cex(log)]
    fn _cex_log() -> Result<(),CexErr> { Ok( _cex_to_log()? )}
}

mod log_level {
    use super::*;

    type Log<E> = super::Log<E, Env<Vec<Frame>>>;

    #[derive( EnumX, Logger, Debug, PartialEq, Eq )]
    enum CexErr {
        Code( Log<i32> ),
        Text( Log<&'static str> ),
    }
    
    #[cex(to_log(( LogLevel::Debug, frame!() )))]
    fn _cex_to_log() -> Result<(),CexErr> { Err(42)? }
    
    #[cex(log(( LogLevel::Info, frame!() )))]
    fn _cex_log() -> Result<(),CexErr> { Ok( _cex_to_log()? )}
}
