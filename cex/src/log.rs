//! Extrusive logging and backtrace support
//!
//! # Example
//!
//! Change
//! 
//! ```rust
//! use enumx_derive::EnumX;
//! use enumx::prelude::*;
//!
//! #[derive( EnumX )]
//! enum Error {
//!     IO(    std::io::Error ),
//!     Parse( std::num::ParseIntError ),
//! }
//! ```
//! 
//! to
//! 
//! ```rust,no_run
//! use enumx_derive::EnumX;
//! use enumx::prelude::*;
//!
//! use cex_derive::Logger;
//! use cex::*;
//!
//! #[derive( EnumX, Logger )]
//! enum Error {
//!     IO(    Log<std::io::Error> ),
//!     Parse( Log<std::num::ParseIntError> ),
//! }
//! ```

use std::env;
use std::marker::PhantomData;

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

/// A wrapper struct for logging inner value.
#[derive( Debug,PartialEq,Eq )]
pub struct Log<Inner, Agent: LogAgent = Vec<Frame>> {
    pub inner : Inner, // the wrapped value
    pub agent : Agent, // log agent
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
        Log{ inner: self, agent: Agent::new() }
    }

    fn to_log( self, item: Agent::Item ) -> Log<Inner,Agent> {
        Log{ inner: self, agent: Agent::create_log( item )}
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

/// Environment variable controlled log level.
///
/// The variable is `CEX_LOG_LEVEL` which are parsed as `u32`, or in the
/// human readable text: Debug, Info, Warn, Error, Fatal, Nolog
///
/// # Usage
///
/// If the user decide to use log level control, the type definition can be
/// unchanged by using a type alias:
/// `type Log<E> = cex::Log<E, Env<Vec<Frame>>>;`.
///
/// Then put log levels in client code, e.g. change
/// `foo.map_err_to_log( frame!() )` to the following:
/// `foo.map_err_to_log(( LogLevel::Debug, frame!() ))`.
/// Note that `map_err_to_log()` accepts a tuple of two elements now.
#[derive( Debug, PartialEq, Eq )]
pub struct Env<Agent: LogAgent>( Agent );

/// Readable log level names
#[repr(u32)]
#[derive( Debug, PartialEq, Eq, PartialOrd, Ord )]
pub enum LogLevel {
    Nolog = 0,
    Fatal = 1,
    Error = 2,
    Warn  = 3,
    Info  = 4,
    Debug = 5,
}

impl<Agent> LogAgent for Env<Agent>
    where Agent : LogAgent
{
    type Item = ( LogLevel, <Agent as LogAgent>::Item );

    fn new() -> Self {
        Env( Agent::new() )
    }

    fn append_log( &mut self, item: Self::Item ) {
        if env_log_enabled( item.0 ) {
            self.0.append_log( item.1 );
        }
    }

    fn create_log( item: Self::Item ) -> Self {
        if env_log_enabled( item.0 ) {
            Env( Agent::create_log( item.1 ))
        } else {
            Env( Agent::new() )
        }
    }
}

/// Read from the environment variable `CEX_LOG_LEVEL` to get a log level value in `u32`.
/// Note that human readable names are also supported.
pub fn log_level() -> u32 {
    fn from_readable( name: &'static str ) -> u32 {
        let value = match name {
            "Debug" => LogLevel::Debug,
            "Info"  => LogLevel::Info ,
            "Warn"  => LogLevel::Warn ,
            "Error" => LogLevel::Error,
            "Fatal" => LogLevel::Fatal,
            "Nolog" => LogLevel::Nolog,
            _       => LogLevel::Nolog,
        };
        value as u32
    }

    let name = "CEX_LOG_LEVEL";

    env::var( name )
        .map( |var| var.parse::<u32>() )
        .unwrap_or( Ok( from_readable( name )))
        .expect( "either from CEX_LOG_LEVEL or default Nolog" )
}

fn env_log_enabled( level: LogLevel ) -> bool {
    let level = level as u32;

    level >  LogLevel::Nolog as u32 &&
    level <= log_level()
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

/// A struct for store one frame for tracing.
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
    ( $($arg:tt)+ ) => {
        Frame::new( module_path!(), file!(), line!(), column!(), Some( format!( $($arg)+ )))
    };
    () => {
        Frame::new( module_path!(), file!(), line!(), column!(), None )
    };
}
