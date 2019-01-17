// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>

#![cfg_attr( test,
    feature( try_blocks, stmt_expr_attributes, proc_macro_hygiene ))]

//! Combinators for EnumX in general, or
//! Combinators for Error eXchange in error-handling.
//!
//! # Example
//!
//! ```rust
//! use enumx_derive::EnumX;
//! use enumx::prelude::*;
//!
//! use cex_derive::{cex,Logger};
//! use cex::*;
//!
//! #[derive( EnumX, Logger, Debug )]
//! enum ReadU32Error {
//!     IO(    Log<std::io::Error> ),
//!     Parse( Log<std::num::ParseIntError> ),
//! }
//! 
//! #[cex(to_log)]
//! fn read_u32( filename: &'static str )
//!     -> Result<u32, ReadU32Error>
//! {
//!     use std::io::Read;
//! 
//!     let mut f = std::fs::File::open( filename )?;
//!     let mut s = String::new();
//!     f.read_to_string( &mut s )?;
//!     let number = s.trim().parse::<u32>()?;
//!     Ok( number )
//! }
//! 
//! #[derive( Debug, PartialEq, Eq )]
//! struct MulOverflow( u32, u32 );
//! 
//! #[derive( EnumX, Logger, Debug )]
//! enum AMulBEqCError {
//!     IO(       Log<std::io::Error> ),
//!     Parse(    Log<std::num::ParseIntError> ),
//!     Overflow( Log<MulOverflow> ),
//! }
//! 
//! #[cex(log)]
//! fn a_mul_b_eq_c( file_a: &'static str, file_b: &'static str, file_c: &'static str )
//!     -> Result<bool, AMulBEqCError>
//! {
//!     let a = read_u32( file_a )?;
//! 
//!     let b = match read_u32( file_b ) {
//!         Ok(  value ) => value,
//!         Err( err ) => {
//!             if a == 0 {
//!                 0 // 0 * b == 0, no matter what b is.
//!             } else {
//!                 return err.error();
//!             }
//!         },
//!     };
//!  
//!     let c = match read_u32( file_c ) {
//!         Ok(  value ) => value,
//!         Err( err   ) => match err {
//!             ReadU32Error::IO(    _ ) => 0, // default to 0 if file is missing.
//!             ReadU32Error::Parse( e ) => return e.error(),
//!         },
//!     };
//! 
//!     Ok( a.checked_mul( b )
//!         .ok_or( MulOverflow(a,b) )
//!         .map( |result| result == c )
//!         .map_err_to_log( frame!() )
//!     ? )
//! }
//! ```

pub use enumx::prelude::*;

/// Enum exchange to wrap an `Ok`.
/// ```rust
/// use cex::*;
/// use enumx::Enum;
///
/// let ok: Result<Enum!(i32,bool),()> = 42.okey();
/// assert_eq!( ok, Ok( Enum2::_0(42) ));
/// ```
pub trait Okey {
    fn okey<E,Dest,Index>( self ) -> Result<Dest,E>
        where Self: Sized + IntoEnumx<Dest,Index>
    {
        Ok( self.into_enumx() )
    }
}

impl<Enum> Okey for Enum {}

/// Enum exchange to wrap an `Err`.
/// ```rust
/// use cex::*;
/// use enumx::Enum;
///
/// let error: Result<(),Enum!(i32,bool)> = 42.error();
/// assert_eq!( error, Err( Enum2::_0(42) ));
/// ```
pub trait Error {
    fn error<T,Dest,Index>( self ) -> Result<T,Dest>
        where Self: Sized + IntoEnumx<Dest,Index>
    {
        Err( self.into_enumx() )
    }
}

impl<Enum> Error for Enum {}

/// Enum exchange for `Ok` combinator.
/// ```rust
/// use cex::*;
/// use enumx::Enum;
///
/// let ok: Result<i32,()> = Ok( 42 );
/// let ok: Result<Enum!(i32,bool),()> = ok.map_okey();
/// assert_eq!( ok, Ok( Enum2::_0(42) ));
/// ```
pub trait MapOkey<Src,E>
    where Self : Into<Result<Src,E>>
{
    fn map_okey<Dest,Indices>( self ) -> Result<Dest,E>
        where Src : Sized + IntoEnumx<Dest,Indices>
    {
        self.into().map( |src| src.into_enumx() )
    }
}

impl<Res,T,Src> MapOkey<T,Src> for Res where Res: Into<Result<T,Src>> {}

/// Enum exchange for `Err` combinator.
/// ```rust
/// use cex::*;
/// use enumx::Enum;
///
/// let error: Result<(),i32> = Err( 42 );
/// let error: Result<(),Enum!(i32,bool)> = error.map_error();
/// assert_eq!( error, Err( Enum2::_0(42) ));
/// ```
pub trait MapError<T,Src>
    where Self : Into<Result<T,Src>>
{
    fn map_error<Dest,Indices>( self ) -> Result<T,Dest>
        where Src : Sized + IntoEnumx<Dest,Indices>
    {
        self.into().map_err( |src| src.into_enumx() )
    }
}

impl<Res,T,Src> MapError<T,Src> for Res where Res: Into<Result<T,Src>> {}

pub mod log;

pub use self::log::*;

#[cfg( feature = "dyn_err" )]
pub mod dyn_err;

#[cfg( feature = "dyn_err" )]
pub use self::dyn_err::*;

#[cfg( test )]
mod test;
