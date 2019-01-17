//! Dynamic error support.
//! # Example
//!
//! ```rust
//! use enumx_derive::EnumX;
//! use cex_derive::cex;
//! use cex::*;
//!
//! #[derive( EnumX, Debug )]
//! enum ReadU32Error {
//!     Parse( std::num::ParseIntError ),
//!     Other( cex::DynErr ),
//! }
//! 
//! #[cex]
//! fn read_u32( filename: &'static str )
//!     -> Result<u32,ReadU32Error>
//! {
//!     use std::io::Read;
//! 
//!     let mut f = std::fs::File::open( filename ).map_dyn_err()?;
//! 
//!     let mut s = String::new();
//!     f.read_to_string( &mut s ).map_dyn_err()?;
//!     let number = s.trim().parse::<u32>()?;
//!     Ok( number )
//! }
//! ```

/// Short for `map_err()` + `into()`
pub trait MapErrTo<T,Src>
    where Self : Into<Result<T,Src>>
{
    fn map_err_to<Dest>( self ) -> Result<T,Dest>
        where Src : Into<Dest>
    {
        self.into().map_err( |err| err.into() )
    }
}

impl<Res,T,Src> MapErrTo<T,Src> for Res where Res: Into<Result<T,Src>> {}

/// Dynamic error type, using trait object.
/// Current implementation utilizes `failure::Error`.
pub use failure::Error as DynErr;

/// Map a plain error to a dynamic error.
pub trait MapDynErr<T,Src>
    where Self : Into<Result<T,Src>>
{
    fn map_dyn_err( self ) -> Result<T,DynErr>
        where Src : Into<DynErr>
    {
        self.into().map_err( |err| err.into() )
    }
}

impl<Res,T,Src> MapDynErr<T,Src> for Res where Res: Into<Result<T,Src>> {}
