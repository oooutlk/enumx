// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

//! Checked exception simulation in Rust.

pub use enumx::*;

pub type Logs = Vec<Log>;

/// Checked exception
#[must_use]
#[derive( Debug,PartialEq,Eq,PartialOrd,Ord )]
pub struct Cex<Enum> {
    pub error : Enum,
    pub logs  : Logs,
}

impl<Enum> Cex<Enum> {
    pub fn new<Error,Index>( err: Error, logs: Logs ) -> Self
        where Error: IntoEnum<Enum,Index>
    {
        Cex{ error: err.into_enum(), logs }
    }

    pub fn append( mut self, log: Log ) -> Self {
        self.logs.push( log );
        self
    }
}

impl<Src,Dest,Indices> IntoEnumX<Cex<Dest>,Indices> for Cex<Src>
    where Src : IntoEnumX<Dest,Indices>
{
    fn into_enumx( self ) -> Cex<Dest> {
        Cex {
            error : self.error.into_enumx(),
            logs  : self.logs,
        }
    }
}

impl<Src,Dest,Indices> ExchangeFrom<Cex<Src>,Indices> for Cex<Dest>
    where Dest: ExchangeFrom<Src,Indices>
{
    fn exchange_from( src: Cex<Src> ) -> Self {
        Cex {
            error  : <Dest as ExchangeFrom<Src,Indices>>::exchange_from( src.error ),
            logs : src.logs,
        }
    }
}

/// Converts a plain error to a checked exception
pub trait Throw<Enum> {
    #[inline( always )]
    fn throw<T,Index>( self ) -> Result<T,Cex<Enum>>
        where Self : Sized + IntoEnum<Enum,Index>
    {
        Err( Cex{ error: self.into_enum(), logs: Vec::new() })
    }

    #[inline( always )]
    fn throw_log<T,Index>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Self : Sized + IntoEnum<Enum,Index>
    {
        Err( Cex{ error: self.into_enum(), logs: vec![ log ]})
    }

    #[inline( always )]
    fn throw_ex<T,Indices>( self ) -> Result<T,Cex<Enum>>
        where Self : Sized + ExchangeInto<Enum,Indices>
    {
        Err( Cex{ error: self.exchange_into(), logs: Vec::new() })
    }

    #[inline( always )]
    fn throw_log_ex<T,Indices>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Self : Sized + ExchangeInto<Enum,Indices>
    {
        Err( Cex{ error: self.exchange_into(), logs: vec![ log ]})
    }
}

impl<E,Enum> Throw<Enum> for E {}

/// Converts a result containing a plain error to a result containing a checked exception.
pub trait MayThrow<T,E>
    where Self : Into<Result<T,E>>
{
    #[inline( always )]
    fn may_throw<Enum,Index>( self ) -> Result<T,Cex<Enum>>
        where E : IntoEnum<Enum,Index>
    {
        self.into().map_err( |err| Cex{ error: err.into_enum(), logs: Vec::new() })
    }

    #[inline( always )]
    fn may_throw_log<Enum,Index>( self, log: Log ) -> Result<T,Cex<Enum>>
        where E : IntoEnum<Enum,Index>
    {
        self.into().map_err( |err| Cex{ error: err.into_enum(), logs: vec![ log ]})
    }
}

impl<T,E> MayThrow<T,E> for Result<T,E> {}

/// Converts a checked exception to another one.
pub trait Rethrow<Enum> : Sized {
    #[inline( always )]
    fn rethrow<T,Indices>( self ) -> Result<T,Cex<Enum>>
        where Self : IntoEnumX<Cex<Enum>,Indices>
    {
        Err( self.into_enumx() )
    }

    #[inline( always )]
    fn rethrow_log<T,Indices>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Self : IntoEnumX<Cex<Enum>,Indices>
    {
        Err( self.into_enumx().append( log ))
    }

    #[inline( always )]
    fn rethrow_ex<T,Indices>( self ) -> Result<T,Cex<Enum>>
        where Self: ExchangeInto<Cex<Enum>,Indices>
    {
        Err( self.exchange_into() )
    }

    #[inline( always )]
    fn rethrow_log_ex<T,Indices>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Cex<Enum> : ExchangeFrom<Self,Indices>
    {
        Err( <Cex<Enum> as ExchangeFrom<Self,Indices>>::exchange_from( self ).append( log ))
    }

    #[inline( always )]
    fn rethrow_named<T,Indices>( self ) -> Result<T,Cex<Enum>>
        where Enum                      : Exchange + From<<Enum as Exchange>::EnumX>
            , <Enum as Exchange>::EnumX : FromEnumX<Self,Indices>
    {
        Err( Cex{ error: <Enum as Exchange>::EnumX::from_enumx( self ).into(), logs: Vec::new() })
    }

    #[inline( always )]
    fn rethrow_log_named<T,Indices>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Enum                      : Exchange + From<<Enum as Exchange>::EnumX>
            , <Enum as Exchange>::EnumX : FromEnumX<Self,Indices>
    {
        Err( Cex{ error: <Enum as Exchange>::EnumX::from_enumx( self ).into(), logs: vec![ log ]})
    }
}

impl<SrcCex,Enum> Rethrow<Enum> for SrcCex {}

/// Converts a result containing a checked exception to a result containing another one.
pub trait MayRethrow<T,E>
    where Self : Into<Result<T,Cex<E>>>
{
    #[inline( always )]
    fn may_rethrow<Enum,Indices>( self ) -> Result<T,Cex<Enum>>
        where Cex<E> : IntoEnumX<Cex<Enum>,Indices>
    {
        self.into().map_err( |cex| cex.into_enumx() )
    }

    #[inline( always )]
    fn may_rethrow_log<Enum,Indices>( self, log: Log ) -> Result<T,Cex<Enum>>
        where Cex<E> : IntoEnumX<Cex<Enum>,Indices>
    {
        self.into().map_err( |mut cex| { cex.logs.push( log ); cex.into_enumx() })
    }

    #[inline( always )]
    fn may_rethrow_ex<Dest,Indices>( self ) -> Result<T,Cex<Dest>>
        where Cex<Dest> : ExchangeFrom<Cex<E>,Indices>
    {
        self.into().map_err( |cex| Cex::<Dest>::exchange_from( cex ))
    }

    #[inline( always )]
    fn may_rethrow_log_ex<Dest,Indices>( self, log: Log ) -> Result<T,Cex<Dest>>
        where Cex<Dest> : ExchangeFrom<Cex<E>,Indices>
    {
        self.into().map_err( |mut cex| { cex.logs.push( log ); Cex::<Dest>::exchange_from( cex )})
    }

    #[inline( always )]
    fn may_rethrow_named<Dest,Indices>( self ) -> Result<T,Cex<Dest>>
        where Dest                      : Exchange + From<<Dest as Exchange>::EnumX>
            , <Dest as Exchange>::EnumX : FromEnumX<E,Indices>
    {
        self.into().map_err( |cex| Cex{ error: <Dest as Exchange>::EnumX::from_enumx( cex.error ).into(), logs: cex.logs })
    }

    #[inline( always )]
    fn may_rethrow_log_named<Dest,Indices>( self, log: Log ) -> Result<T,Cex<Dest>>
        where Dest                      : Exchange + From<<Dest as Exchange>::EnumX>
            , <Dest as Exchange>::EnumX : FromEnumX<E,Indices>
    {
        self.into().map_err( |mut cex| { cex.logs.push( log ); Cex{ error: <Dest as Exchange>::EnumX::from_enumx( cex.error ).into(), logs: cex.logs }})
    }
}

impl<T,E> MayRethrow<T,E> for Result<T,Cex<E>> {}

/// A struct for tracing the propagation of the error.
#[derive( Debug,PartialEq,Eq,PartialOrd,Ord )]
pub struct Log {
    pub module   : &'static str,
    pub file     : &'static str,
    pub line     : u32,
    pub column   : u32,
    pub info     : Option<String>,
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
macro_rules! throw_ex {
    ( $expr:expr ) => { return $expr.throw_ex(); }
}

#[macro_export]
macro_rules! throw_log_ex {
    ( $expr:expr, $($arg:tt)+ ) => { return $expr.throw_log_ex( log!( $($arg)+ )); };
    ( $expr:expr ) => { return $expr.throw_log_ex( log!() ); };
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
macro_rules! rethrow_ex {
    ( $expr:expr ) => { return $expr.rethrow_ex(); }
}

#[macro_export]
macro_rules! rethrow_log_ex {
    ( $expr:expr, $($arg:tt)+ ) => { return $expr.rethrow_log_ex( log!( $($arg)+ )); };
    ( $expr:expr ) => { return $expr.rethrow_log_ex( log!() ); };
}

#[cfg( test )]
mod test;
