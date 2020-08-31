//! # Backtrace
//! Backtrace is disabled by default. When enabled, locations of error propagation
//! by `ret!()`, `throw!()` and `?` operator will be stored in the `Err` variant.
//!
//! ## Use `log` feature to enable backtrace.
//!
//! ```toml
//! [dependencies.cex]
//! version = "0.5"
//! features = ["log"]
//! ```
//!
//! ## Use `env_log` feature to enable backtrace if the envirnoment variable
//! `RUST_BACKTRACE` is 1 or "full".
//!
//! ```toml
//! [dependencies.cex]
//! version = "0.5"
//! features = ["env_log"]
//! ```
//!
//! ## Use `pretty_log` feature to pretty-print the frames, as if "{:#?}" were used.
//!
//! ```toml
//! [dependencies.cex]
//! version = "0.5"
//! features = ["log","pretty_log"]
//! # or features = ["env_log","pretty_log"]
//! ```
//!
//! ```rust,no_run
//! use enumx::export::*;
//! use enumx::predefined::*;
//! use cex::*;
//!
//! #[cex]
//! pub fn foo() -> Result!( () throws () ) {
//!     throw!( () );
//! }
//!
//! #[cex]
//! pub fn bar() -> Result!( () throws () ) {
//!     ret!( foo()? );
//! }
//!
//! fn main() {
//!     bar().unwrap();
//! }
//! ```
//!
//! The output is similar as follows:
//!
//! ```text
//! thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: _0(Log {
//!     error: (),
//!     agent: [
//!         Frame {
//!             module: "my_program",
//!             file: "src/main.rs",
//!             line: 5,
//!             column: 13,
//!             info: Some(
//!                 "throw!(())",
//!             ),
//!         },
//!         Frame {
//!             module: "my_program",
//!             file: "src/main.rs",
//!             line: 10,
//!             column: 11,
//!             info: Some(
//!                 "foo()",
//!             ),
//!         },
//!     ],
//! })', src/main.rs:14:5
//! ```
//!
//! ## Forward log features to cex crate
//!
//! ```toml
//! [features]
//! log = ["cex/log"]
//! env_log = ["cex/env_log"]
//! pretty_log = ["cex/pretty_log"]
//! ```
//!
//! ## `ret!()`/`throw!()` could have the second argument as a customized log item.
//!
//! ```text
//! ret!( expr, || frame!( "expect an ok value" ));
//! throw!( expr, || frame!( "oops" ));
//! ```
//!
//! Even if backtrace is disabled at compile time, these will compile. The second
//! argument just has no effect.

use std::{
    env,
    fmt::Debug,
    marker::PhantomData,
};

/// Log agent.
pub trait LogAgent {
    type Item;

    fn new() -> Self;
    fn create_log( item: Self::Item ) -> Self;
    fn append_log( &mut self, item: Self::Item );
}

impl<T> LogAgent for Vec<T> {
    type Item = T;

    fn new() -> Self { Vec::new() }
    fn create_log( item: T ) -> Self { vec![ item ] }
    fn append_log( &mut self, item: T ) { self.push( item ); }
}

impl<T> LogAgent for PhantomData<T> {
    type Item = T;

    fn new() -> Self { PhantomData }
    fn create_log( _item: T ) -> Self { PhantomData }
    fn append_log( &mut self, _item: T ) {}
}

impl LogAgent for String {
    type Item = String;

    fn new() -> Self { String::new() }
    fn create_log( item: String ) -> Self { item }
    fn append_log( &mut self, item: String ) { self.push_str( &format!( "\n{}", item )); }
}

/// A wrapper struct for logging error value.
#[derive( PartialEq,Eq )]
pub struct Log<Inner, Agent: LogAgent = Vec<Frame>> {
    pub error : Inner, // the error
    pub agent : Agent, // log agent
}

#[cfg( not( feature = "pretty_log" ))]
impl<Inner,Agent> Debug for Log<Inner,Agent>
    where Inner: Debug
        , Agent: Debug + LogAgent
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        f.debug_struct("Log")
         .field( "error", &self.error )
         .field( "agent", &self.agent )
         .finish()
    }
}

#[cfg( feature = "pretty_log" )]
impl<Inner,Agent> Debug for Log<Inner,Agent>
    where Inner: Debug
        , Agent: Debug + LogAgent
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("Log")
             .field( "error", &self.error )
             .field( "agent", &self.agent )
             .finish()
        } else {
            write!( f, "{:#?}", self )
        }
    }
}

/// A type alias for opt-out logging at compile time.
pub type NoLog<Inner,Item=Frame> = Log<Inner,PhantomData<Item>>;

/// Wraps a type with Log.
pub trait ToLog<Agent> : Sized
    where Agent : LogAgent
{
    fn new_log( self ) -> Log<Self,Agent>;
    fn to_log( self, item: Agent::Item ) -> Log<Self,Agent>;
}

impl<Inner,Agent> ToLog<Agent> for Inner
    where Agent : LogAgent
{
    fn new_log( self ) -> Log<Self,Agent> {
        Log{ error: self, agent: Agent::new() }
    }

    fn to_log( self, item: Agent::Item ) -> Log<Inner,Agent> {
        Log{ error: self, agent: Agent::create_log( item )}
    }
}

/// Appends a log item.
pub trait Logger<Agent> : Sized
    where Agent : LogAgent
{
    fn log( self, item: Agent::Item ) -> Self;
}

impl<Agent,E> Logger<Agent> for Log<E,Agent>
    where Agent : LogAgent
{
    fn log( mut self, item: Agent::Item ) -> Self {
        self.agent.append_log( item );
        self
    }
}

macro_rules! impl_logger_for_predefined_enumx {
    ($($enumx:ident => $($_index:ident $gen:ident)*;)+) => {
        use ::enumx::predefined::*;
        $(
            impl<Agent$(,$gen)*> Logger<Agent> for $enumx<$($gen),*>
                where Agent : LogAgent
                  $(, $gen  : Logger<Agent> )*
            {
                fn log( self, _item: Agent::Item ) -> Self {
                    match self {
                        $( $enumx::$_index( $_index ) => $enumx::$_index( Logger::<Agent>::log( $_index, _item )), )*
                    }
                }
            }
        )+
    };
}

impl_logger_for_predefined_enumx! {
    Enum0  => ;
    Enum1  => _0 T0;
    Enum2  => _0 T0 _1 T1;
    Enum3  => _0 T0 _1 T1 _2 T2;
    Enum4  => _0 T0 _1 T1 _2 T2 _3 T3;
    Enum5  => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4;
    Enum6  => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5;
    Enum7  => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6;
    Enum8  => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7;
    Enum9  => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8;
    Enum10 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9;
    Enum11 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9 _10 T10;
    Enum12 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9 _10 T10 _11 T11;
    Enum13 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9 _10 T10 _11 T11 _12 T12;
    Enum14 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9 _10 T10 _11 T11 _12 T12 _13 T13;
    Enum15 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9 _10 T10 _11 T11 _12 T12 _13 T13 _14 T14;
    Enum16 => _0 T0 _1 T1 _2 T2 _3 T3 _4 T4 _5 T5 _6 T6 _7 T7 _8 T8 _9 T9 _10 T10 _11 T11 _12 T12 _13 T13 _14 T14 _15 T15;
}

/// Environment variable `RUST_BACKTRACE` controlled log agent.
#[derive( Debug, PartialEq, Eq )]
pub struct Env<Agent: LogAgent>( Agent );

impl<Agent> LogAgent for Env<Agent>
    where Agent : LogAgent
{
    type Item = <Agent as LogAgent>::Item;

    fn new() -> Self {
        Env( Agent::new() )
    }

    fn append_log( &mut self, item: Self::Item ) {
        if env_log_enabled() {
            self.0.append_log( item );
        }
    }

    fn create_log( item: Self::Item ) -> Self {
        if env_log_enabled() {
            Env( Agent::create_log( item ))
        } else {
            Env( Agent::new() )
        }
    }
}

fn env_log_enabled() -> bool {
    env::var( "RUST_BACKTRACE" )
        .map( |value| value == "1" || value == "full" )
        .unwrap_or( false )
}

/// Wraps the `Ok` variant with Log
pub trait MapToLog<Agent> : Sized
    where Agent : LogAgent
{
    type Output;

    fn map_to_log( self, item: Agent::Item ) -> Self::Output;
}

impl<Agent,T,E> MapToLog<Agent> for Result<T,E>
    where E     : ToLog<Agent>
        , Agent : LogAgent
{
    type Output = Result<Log<T,Agent>, E>;

    fn map_to_log( self, item: Agent::Item ) -> Self::Output {
        self.map( |v| v.to_log( item ))
    }
}

/// Wraps the `Err` variant with Log
pub trait MapErrToLog<Agent> : Sized
    where Agent : LogAgent
{
    type Output;

    fn map_err_to_log( self, item: Agent::Item ) -> Self::Output;
}

impl<Agent,T,E> MapErrToLog<Agent> for Result<T,E>
    where E     : ToLog<Agent>
        , Agent : LogAgent
{
    type Output = Result<T, Log<E,Agent>>;

    fn map_err_to_log( self, item: Agent::Item ) -> Self::Output {
        self.map_err( |e| e.to_log( item ))
    }
}

/// Appends a log item to the `Ok` variant.
pub trait MapLog<Agent> : Sized
    where Agent : LogAgent
{
    type Output;

    fn map_log( self, item: Agent::Item ) -> Self::Output;
}

impl<Agent,T,E> MapLog<Agent> for Result<T,E>
    where T     : Logger<Agent>
        , Agent : LogAgent
{
    type Output = Self;

    fn map_log( self, item: Agent::Item ) -> Self::Output {
        self.map( |e| e.log( item ))
    }
}

/// Appends a log item to the `Err` variant.
pub trait MapErrLog<Agent> : Sized
    where Agent : LogAgent
{
    type Output;

    fn map_err_log( self, item: Agent::Item ) -> Self::Output;
}

impl<Agent,T,E> MapErrLog<Agent> for Result<T,E>
    where E     : Logger<Agent>
        , Agent : LogAgent
{
    type Output = Self;

    fn map_err_log( self, item: Agent::Item ) -> Self::Output {
        self.map_err( |e| e.log( item ))
    }
}

/// A struct for store one frame for backtrace.
#[derive( Debug,Default,PartialEq,Eq,PartialOrd,Ord )]
pub struct Frame {
    pub module : &'static str,
    pub file   : &'static str,
    pub line   : u32,
    pub column : u32,
    pub info   : Option<String>,
}

impl Frame {
    pub fn new( module: &'static str, file: &'static str, line: u32, column: u32, info: Option<String> ) -> Self {
        Frame{ module, file, line, column, info }
    }
}

/// A macro to generate a `Frame`, to store the source of the error with file
/// name, module path, line/column numbers, and an optional context info, using
/// the same syntax with `format!()`.
///
/// An example: `frame!( "An unexpected {:?} was detect.", local_var ))`
#[macro_export]
macro_rules! frame {
    ( $expr:expr ) => {
        Frame::new( module_path!(), file!(), line!(), column!(), Some( String::from( $expr )))
    };
    () => {
        Frame::new( module_path!(), file!(), line!(), column!(), None )
    };
}
