use std::{ error, fmt, num, io };
use std::io::Read;

type Result<T> = std::result::Result<T, Error>;

#[derive( Debug )]
enum Error {
    IO( io::Error ),
    Parse( num::ParseIntError ),
    Calc( u32, u32 ),
}

impl fmt::Display for Error {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            Error::IO( ref e ) => e.fmt( f ),
            Error::Parse( ref e ) => e.fmt( f ),
            Error::Calc( a, b ) => {
                write!( f, "u32 overflow: {} * {}", a, b )
            },
        }
    }
}

impl error::Error for Error {
    fn description( &self ) -> &str {
        match *self {
            Error::IO( ref e ) => e.description(),
            Error::Parse( ref e ) => e.description(),
            Error::Calc( _, _ ) => "multiplication overflow",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO( ref e ) => Some( e ),
            Error::Parse( ref e ) => Some( e ),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from( io_error: io::Error ) -> Error {
        Error::IO( io_error )
    }
}

impl From<num::ParseIntError> for Error {
    fn from( err: num::ParseIntError ) -> Error {
        Error::Parse( err )
    }
}

impl From<(u32,u32)> for Error {
    fn from( (a,b): (u32,u32) ) -> Error {
        Error::Calc( a,b )
    }
}

fn read_u32( filename: &'static str ) -> Result<u32> {
    let mut f = std::fs::File::open( filename )?;
    let mut s = String::new();
    f.read_to_string( &mut s )?;
    let number = s.trim().parse::<u32>()?;
    Ok( number )
}

fn a_mul_b_eq_c(
    file_a: &'static str,
    file_b: &'static str,
    file_c: &'static str )
    -> Result<bool>
{
    let a = read_u32( file_a )?;

    let b = match read_u32( file_b ) {
        Ok(  value ) => value,
        Err( error ) => {
            if a == 0 {
                0 // 0 * b == 0, no matter what b is.
            } else {
                return Err( error );
            }
        },
    };

    let c = match read_u32( file_c ) {
        Ok(  value ) => value,
        Err( error ) => match error {
            Error::IO(     _ ) => 0, // default to 0 if file is missing.
            Error::Parse(  _ ) => return Err( error ),
            Error::Calc( _,_ ) => {
                unreachable!(); // read_u32 does not do calculating at all!
            },
        },
    };

    a.checked_mul( b )
     .ok_or( Error::Calc(a,b) )
     .map( |result| result == c )
}

#[test]
fn test_read_u32() {
    assert!( read_u32("src/test/no_file").map_err( |error|
        if let Error::IO(_) = error { true } else { false }
    ).unwrap_err() );

    assert!( read_u32("src/test/not_num").map_err( |error| {
        if let Error::Parse(_) = error { true } else { false }
    }).unwrap_err() );

    assert_eq!( read_u32("src/test/3").ok().unwrap(), 3 );
}

#[test]
fn test_a_mul_b_eq_c() {
    assert!(
        a_mul_b_eq_c( "src/test/no_file", "src/test/7", "src/test/21"
        ).map_err( |error|
            if let Error::IO(_) = error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/not_num", "src/test/7", "src/test/21"
        ).map_err( |error|
            if let Error::Parse(_) = error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/no_file", "src/test/21"
        ).map_err( |error|
            if let Error::IO(_) = error { true } else { false }
        ).unwrap_err() );

    assert!(
        a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21"
        ).map_err( |error|
            if let Error::Parse(_) = error { true } else { false }
        ).unwrap_err() );

    assert!( a_mul_b_eq_c( "src/test/3", "src/test/0",       "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/0", "src/test/no_file", "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/0", "src/test/not_num", "src/test/0"  ).ok().unwrap() );
    assert!( a_mul_b_eq_c( "src/test/3", "src/test/7",       "src/test/21" ).ok().unwrap() );
}
