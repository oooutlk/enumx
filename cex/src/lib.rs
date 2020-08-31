// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>

//! Checked EXceptions for Rust.
//!
//! See the [enumx book](https://oooutlk.github.io/enumx/) for more.
//!
//! # Features
//!
//! 1. Use `Result!( Type throws A,B,.. )`, `ret!()`, `throw!()` to simulate
//! checked exceptions in Rust
//!
//! 2. `#[ty_pat] match` for "type as pattern matching" in match expressions.
//!
//! 3. Optional backtrace support.
//!
//! # Examples
//!
//! ```rust
//! use enumx::export::*;
//! use enumx::predefined::*;
//! use cex::*;
//!
//! // accepts even numbers; rejects odd ones and report an error `String`
//! #[cex]
//! fn check_even( a: u32 ) -> Result!( u32 throws String ) {
//!     if a % 2 == 1 {
//!         throw!( format!( "odd numbers not allowed: a == {}", a ));
//!     } else {
//!         ret!( a );
//!     }
//! }
//!
//! // accepts non-zero numbers; rejects zeros and report an error of `&'static str`
//! #[cex]
//! fn check_nonzero( b: u32 ) -> Result!( u32 throws &'static str ) {
//!     if b == 0 {
//!         throw!( "zero not allowed: b == 0" );
//!     } else {
//!         ret!( b )
//!     }
//! }
//!
//! struct Underflow;
//!
//! #[cex]
//! fn sub( a: u32, b: u32 ) -> Result!( u32 throws String, &'static str, Underflow ) {
//!     let a = check_even( a )?;
//!     let b = check_nonzero( b )?;
//!     ret!( a+b );
//! }
//!
//! #[cex]
//! fn distance( a: u32, b: u32 ) -> Result!( u32 throws String, &'static str ) {
//!     ret!( sub(a,b).or_else( |err| {#[ty_pat] match err {
//!         Underflow => ret!( b-a ),
//!         String(s) => throw!( s ),
//!         TyPat::<&'static str>(s) => throw!( s ),
//!     }}))
//! }
//!
//! #[cex]
//! fn distance2( a: u32, b: u32 ) -> Result!( u32 throws String, &'static str ) {
//!     ret!( sub(a,b).or_else( |err| #[ty_pat(gen_throws)] match err {
//!         Underflow => ret!( b-a ),
//!     }))
//! }
//!
//! #[cex]
//! fn distance3( a: u32, b: u32 ) -> Result!( u32 throws String, &'static str ) {
//!     ret!( sub(a,b).or_else( |err| #[ty_pat(gen &'static str, String )] match err {
//!         Underflow => ret!( b-a ),
//!     }))
//! }
//! ```

use enumx::export::*;

/// Enum exchange to wrap an `Err`.
///
/// ```rust
/// use enumx::export::*;
/// use enumx::predefined::*;
/// use cex::*;
///
/// let error: Result<(),Enum!(i32,bool)> = 42.error();
/// assert_eq!( error, Err( Enum2::_0(42) ));
/// ```
pub trait Error {
    fn error<T,Dest,Index>( self ) -> Result<T,Dest>
        where Self: Sized + ExchangeInto<Dest,Index>
    {
        Err( self.exchange_into() )
    }
}

impl<Enum> Error for Enum {}

/// Enum exchange for `Err` combinator.
///
/// ```rust
///
/// use enumx::export::*;
/// use enumx::predefined::*;
/// use cex::*;
///
/// let error: Result<(),i32> = Err( 42 );
/// let error: Result<(),Enum!(i32,bool)> = error.map_error();
/// assert_eq!( error, Err( Enum2::_0(42) ));
/// ```
pub trait MapError<T,Src>
    where Self : Into<Result<T,Src>>
{
    fn map_error<Dest,Indices>( self ) -> Result<T,Dest>
        where Src : Sized + ExchangeInto<Dest,Indices>
    {
        self.into().map_err( |src| src.exchange_into() )
    }
}

impl<Res,T,Src> MapError<T,Src> for Res where Res: Into<Result<T,Src>> {}

pub mod result;
pub use result::*;

pub mod log;
pub use self::log::*;

#[cfg( not( any( feature="log", feature="env_log" )))]
pub use cex_derive::cex;
#[cfg( not( any( feature="log", feature="env_log" )))]
pub use cex_derive::Result;

#[cfg( all( feature="log", not( feature="env_log" )))]
pub use cex_derive::cex_log as cex;
#[cfg( all( feature="log", not( feature="env_log" )))]
pub use cex_derive::ResultLog;

#[cfg( all( feature="env_log", not( feature="log" )))]
pub use cex_derive::cex_env_log as cex;
#[cfg( all( feature="env_log", not( feature="log" )))]
pub use cex_derive::ResultEnvLog;
