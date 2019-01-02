// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! Checked exception simulation in Rust.

pub use enumx::Enum;
pub use enumx::prelude::*;

pub type Logs = Vec<Log>;

/// Checked exception
#[must_use]
#[derive( Debug,PartialEq,Eq,PartialOrd,Ord )]
pub struct Cex<Enum> {
    pub error : Enum,
    pub logs  : Logs,
}

impl<Enum> Cex<Enum> {
    pub fn new<Error,Index,Kind>( err: Error, logs: Logs ) -> Self
        where Error: IntoEnum<Enum,Index,Kind>
    {
        Cex{ error: err.into_enum(), logs }
    }

    pub fn rethrow<T,Dest,Indices,Kind>( self ) -> Result<T,Cex<Dest>>
        where Enum: ExchangeInto<Dest,Indices,Kind>
    {
        Err( Cex{ error: self.error.exchange_into(), logs: self.logs })
    }

    pub fn rethrow_log<T,Dest,Indices,Kind>( mut self, log: Log ) -> Result<T,Cex<Dest>>
        where Enum: ExchangeInto<Dest,Indices,Kind>
    {
        self.logs.push( log );
        Err( Cex{ error: self.error.exchange_into(), logs: self.logs })
    }
}

/// Converts a plain error to a checked exception
pub trait Throw<Enum> {
    #[inline( always )]
    fn throw<T,Index,Kind>( self ) -> Result<T,Cex<Enum>>
        where Self : Sized + IntoEnum<Enum,Index,Kind>
    {
        Err( Cex{ error: self.into_enum(), logs: Vec::new() })
    }

    #[inline( always )]
    fn throw_log<T,Index,Kind>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Self : Sized + IntoEnum<Enum,Index,Kind>
    {
        Err( Cex{ error: self.into_enum(), logs: vec![ log ]})
    }
}

impl<E,Enum> Throw<Enum> for E {}

/// Converts a result containing a plain error to a result containing a checked exception.
pub trait MayThrow<T,E>
    where Self : Into<Result<T,E>>
{
    #[inline( always )]
    fn may_throw<Enum,Index,Kind>( self ) -> Result<T,Cex<Enum>>
        where E : IntoEnum<Enum,Index,Kind>
    {
        self.into().map_err( |err| Cex{ error: err.into_enum(), logs: Vec::new() })
    }

    #[inline( always )]
    fn may_throw_log<Enum,Index,Kind>( self, log: Log ) -> Result<T,Cex<Enum>>
        where E : IntoEnum<Enum,Index,Kind>
    {
        self.into().map_err( |err| Cex{ error: err.into_enum(), logs: vec![ log ]})
    }
}

impl<Res,T,E> MayThrow<T,E> for Res where Res: Into<Result<T,E>> {}

/// Converts a result containing a checked exception to a result containing another one.
pub trait MayRethrow<T,Src>
    where Self : Into<Result<T,Cex<Src>>>
{
    #[inline( always )]
    fn may_rethrow<Dest,Indices,Kind>( self ) -> Result<T,Cex<Dest>>
        where Src : ExchangeInto<Dest,Indices,Kind>
    {
        self.into().map_err( |cex| Cex{ error: cex.error.exchange_into(), logs: cex.logs })
    }

    #[inline( always )]
    fn may_rethrow_log<Dest,Indices,Kind>( self, log: Log ) -> Result<T,Cex<Dest>>
        where Src : ExchangeInto<Dest,Indices,Kind>
    {
        self.into().map_err( |mut cex| {
            cex.logs.push( log );
            Cex{ error: cex.error.exchange_into(), logs: cex.logs }
        })
    }
}

impl<Res,T,Src> MayRethrow<T,Src> for Res where Res: Into<Result<T,Cex<Src>>> {}

/// A struct for tracing the propagation of the error.
#[derive( Debug,PartialEq,Eq,PartialOrd,Ord )]
pub struct Log {
    pub module : &'static str,
    pub file   : &'static str,
    pub line   : u32,
    pub column : u32,
    pub info   : Option<String>,
}

impl Log {
    pub fn new( module: &'static str, file: &'static str, line: u32, column: u32, info: Option<String> ) -> Self {
        Log{ module, file, line, column, info }
    }
}

#[macro_export]
macro_rules! log {
    ( $($arg:tt)+ ) => {
        Log::new( module_path!(), file!(), line!(), column!(), Some( format!( $($arg)+ )))
    };
    () => {
        Log::new( module_path!(), file!(), line!(), column!(), None )
    };
}

#[macro_export]
macro_rules! throw {
    ( $expr:expr ) => { return $expr.throw(); }
}

#[macro_export]
macro_rules! throw_log {
    ( $expr:expr, $($arg:tt)+ ) => { return $expr.throw_log( log!( $($arg)+ )); };
    ( $expr:expr ) => { return $expr.throw_log( log!() ); };
}

#[macro_export]
macro_rules! rethrow {
    ( $expr:expr ) => { return $expr.rethrow(); }
}

#[macro_export]
macro_rules! rethrow_log {
    ( $expr:expr, $($arg:tt)+ ) => { return $expr.rethrow_log( log!( $($arg)+ )); };
    ( $expr:expr ) => { return $expr.rethrow_log( log!() ); };
}

#[macro_export]
macro_rules! Throws {
    ( $($tt:tt)+ ) => { cex::Cex<Enum!($($tt)+)> }
}

#[cfg( test )]
mod test;
