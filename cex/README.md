# Summary

Introduce new combinators for `Result`, with perticular interest in recoverable
error-handling in favor of static dispatch.

The new constructs are:

* An `EnumX` proc-macro derive for `enum`s.

* New wrapper/combinator of `Result`: `error()` and `map_error()`, for error
conversions.

* New wrappers/combinators of `Result`: `err_to_log()`, `err_log()`,
`map_err_to_log()` and `map_err_log()`, for logging.

# Motivation and overview

Error-handling in manner of static dispatch has some fundamental advantages:

* Using type system to express design constraints at compile time.

* Be compitable with approaches using dynamic dispath, but not vice-versa.

This RFC discusses topics about error definition, must-have traits, error
 conversions and logging to conclude the general error-handling approach, in
spirit of static dispatch and orthogonality.

## Definition of recoverable error

In Rust, a recoverable error is a `Result::Err`. Neither panics nor exceptions
(stack unwinding) are considered as recoverable error-handling.

From now on, "error" is used as the synonym of "recoverable error", and "the
static approach" is short for "the proposed approach of error-handling in manner
of static dispatch".

## Traits that a plain error must have

The essence of plain error is to indicating branches for its caller, while
composite errors do more, e.g. conversion and logging.

In general, a plain error should have no mandatory trait. In specific cases,
errors may be expected to have certain traits. The static approach should allow
plain errors with no traits, meanwhile be capable of adding traits as needed.

## Library-provided error conversions

An error-handling library must provide error conversions in a systematical way.
The static approach must provide such mechanism in favor of `enum` rather than
trait object, because the latter will force users to implement mandatory traits.

An `enum` of errors for summarizing and converting is regarded as composite,
while its variant types are plain errors.

To support `enum` conversions systematically, an `EnumX` proc-macro derive will
be introduced and explained later.

## Library-provided logging

The static approach must provide logging mechanisms, which 

* is customizable

* does not force users to implement certain traits for plain errors.

* can be completely opt-out and substituted by user-defined logging.

## The example

Consider an artificial example. The task is to read 3 integer values a, b, and c
from 3 different files and check if they satisfied the equation `a * b == c`.

To read integers from files, a `read_u32()` function is defined. It returns a
`u32` if everything is ok, or some file operations fails or parsing integer from
text fails, resulting `std::io::Error` or `std::num::ParseIntError`.

To check the equation, an `a_mul_b_eq_c()` function is defined. It calls
`read_u32()` and do the calculating, which introduces a new possible error type
defined as `struct MulOverflow( u32, u32 )` to indicate a multiplication
overflow.

The three errors `std::io::Error`, `std::num::ParseIntError` and `MulOverflow`
are considered as plain errors. To do error convertion, some composite errors
should be defined with `EnumX` derived.

```rust
#[derive( EnumX )]
enum ReadU32Error {
    IO(    std::io::Error ),
    Parse( std::num::ParseIntError ),
}

fn read_u32( filename: &'static str ) -> Result<u32, ReadU32Error> {/**/}

#[derive( EnumX )]
enum AMulBEqCError {
    IO(       std::io::Error ),
    Parse(    std::num::ParseIntError ),
    Overflow( MulOverflow ),
}

fn a_mul_b_eq_c(
    file_a: &'static str,
    file_b: &'static str,
    file_c: &'static str
) -> Result<bool, AMulBEqCError>
{/**/}
```

`ReadU32Error` and `AMulBEqCError` are regarded as composite errors.

## EnumX derive

The `#[derive( EnumX )]` attributes will tell proc-macro to generate convertions
for `ReadU32Error` and `AmulBEqCError`:

* Other `EnumX`s can be constructed from them if the others has all variant
types defined in the source enum type.

* They can be constructed from other `EnumX` if they has all variant types of
the others.

* They can be constructed from any variant type.

## Error wrapper/combinator

* `error()`: plain/composite `error_type` to a composite `Err(error_type)`. 

* `map_error()`: plain/composite `Err` to a composite `Err`.

```rust
fn read_u32( filename: &'static str ) -> Result<u32,ReadU32Error> {
    use std::io::Read;

    let mut f = std::fs::File::open( filename ).map_error()?;
    let mut s = String::new();
    f.read_to_string( &mut s ).map_error()?;
    let number = s.trim().parse::<u32>().map_error()?;
    Ok( number )
}

fn a_mul_b_eq_c(
    file_a: &'static str,
    file_b: &'static str,
    file_c: &'static str
) -> Result<bool, AMulBEqCError>
{
    let a = read_u32( file_a ).map_error()?;

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
     .map( |result| result == c )
     .map_error()
}
```

## Logging

The static approach does not provide the ability to do backtracing via
predefined methods of plain errors. The users are free to do so, as long as they
want, using whatever traits they like.

Instead, it will ask users to change the definitions of the composite errors, to
turn on library-supported backtrace/logging.

```rust
#[derive( EnumX, Logger, Debug )]
enum ReadU32Error {
    IO(    Log<std::io::Error> ),
    Parse( Log<std::num::ParseIntError> ),
}

#[derive( EnumX, Logger, Debug )]
enum AMulBEqCError {
    IO(       Log<std::io::Error> ),
    Parse(    Log<std::num::ParseIntError> ),
    Overflow( Log<MulOverflow> ),
}
```

Note that a wrapper struct, `Log` and a derivable `Logger` trait are introduced.
The enum with `#[derive(Logger)]` must use the same log items among all of its
variants.

```rust
#[derive( EnumX, Logger, Debug )]
fn read_u32( filename: &'static str ) -> Result<u32,ReadU32Error> {
    use std::io::Read;

    let mut f = std::fs::File::open( filename )
        .map_err_to_log(frame!())
        .map_error()?;

    let mut s = String::new();
    f.read_to_string( &mut s )
        .map_err_to_log(frame!())
        .map_error()?;

    let number = s.trim().parse::<u32>()
        .map_err_to_log(frame!())
        .map_error()?;

    Ok( number )
}

fn a_mul_b_eq_c(
    file_a: &'static str,
    file_b: &'static str,
    file_c: &'static str
) -> Result<bool, AMulBEqCError>
{
    let a = read_u32( file_a ).map_err_log(frame!()).map_error()?;

    let b = match read_u32( file_b ) {
        Ok(  value ) => value,
        Err( err ) => {
            if a == 0 {
                0 // 0 * b == 0, no matter what b is.
            } else {
                return err.log(frame!()).error();
            }
        },
    };
 
    let c = match read_u32( file_c ) {
        Ok(  value ) => value,
        Err( err   ) => match err {
            ReadU32Error::IO(    _ ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( e ) => return e.log(frame!()).error(),
        },
    };

    a.checked_mul( b )
     .ok_or( MulOverflow(a,b) )
     .map( |result| result == c )
     .map_err_to_log( frame!() )
     .map_error()
}
```

## Log combinators

To summarize:

* `to_log(item)` converts an `E` to `Log<E>` with a logged `item`.

* `log(item)` appends an `item` to an existing `Log<E>`.

* `map_err_to_log(item)` is a combination of `map_err()` and `to_log(item)`

* `map_err_log(item)` is a combination of `map_err()` and `log(item)`

## Log agent and items

A log agent can be an in-memory `String`, `Vec`, a file on disk, or any thing
that the user is willing to use to collect log items. Both the log agent and its
items are generialized and fully customizable.

```rust
pub trait LogAgent {
    type Item;

    fn new() -> Self;
    fn create_log( item: Self::Item ) -> Self;
    fn append_log( &mut self, item: Self::Item );
}
```

Users can implement this trait to customize the behaviours on `to_log()` using
`create_log()`, and `log()` using `append_log()`. 

Note that log item is generalized and not restricted to strings. The static
approach provides a typed logging system that has the power to support both
common logging tasks and highly customized ones.

Some common log items could be predefined for convenience, e.g. A `Frame` to
store the source of the error with file name, module path, line/column numbers,
and an optional context info. A call similar to
`.log( frame!( "An unexpected {:?} was detect.", local_var ))` is up to the job.
The `frame!()` macro uses the same syntax with `format!()`, but results in a
`Frame` struct rather than a string.

Users can switch between type aliases to choose different log agents or items.

```rust
type Log<E> = cex::Log<E,Vec<Frame>>; // Vec agent, Frame item
```

```rust
type Log<E> = cex::Log<E,Vec<String>>; // Vec agent, String item
```

```rust
type Log<E> = cex::Log<E,String>; // String agent, String item
```

```rust
type Log<E> = cex::Log<E,MyLogAgent>; // User-defined agent and item
```

## Compile time opt-in logging

Using type aliases to turn on/off logging at compile time.

```rust
type Log<E> = cex::Log<E>; // do logging
```

```rust
type Log<E> = cex::NoLog<E>; // no logging
```

## Runtime opt-in logging: Logging-level

The type alias to turn on logging-level support.

```rust
type Log<E> = cex::Log<E, Env<Vec<Frame>>>;
```

The logging-level is defined by an environment variable `CEX_LOG_LEVEL`. Client
code providing a value no greater than it, is allowed to do logging.

```rust
let mut f = std::fs::File::open( filename )
    .map_err_to_log(( LogLevel::Debug, frame!() ))
    .map_error()?;
```

Note that the item is a tuple, the first field of which is the level.

## The ergonomic issue of extra annotations 

* Composite errors work with `?` at the cost of extra annotation `map_error()`.

* Logging requries explicit method calls of `map_err_to_log()`/`map_err_log()`.

To address the issue, users can tag the `fn` with an `#[cex]`, with optional
arguments. 

* A `#[cex]` will append `.map_error()`s to try expressions, unless they end
with it already.

* Besides, a `#[cex(log)]` will append `.map_err_log(frame!())`s to try
expressions.

* Besides, a `#[cex(log(expr))]` will append `.map_err_log(#expr)`s to try
expressions.

* A `#[cex(to_log)]` is similar with `#[cex(log)]`, but provides
`map_err_to_log` instead of `map_err_log`.

* A `#[cex(to_log(expr)]` is similar with `#[cex(log(expr))]`, but provides
`map_err_to_log` instead of `map_err_log`.

* Besides, a `#[cex(map_err(expr))]` will append `.map_err(#expr)`s to try
expressions.

* All the logging attributes will append nothing if the try expressions end with
`.map_err_to_log()`/`.map_err_log()`/`.map_err()` already.

* All the `#[cex]` tags will append nothing for functions, closures and try
expressions inside the `#[cex] fn` that are not tagged with `#[cex]` themselves.

The modified example:

```rust
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
fn a_mul_b_eq_c(
    file_a: &'static str,
    file_b: &'static str,
    file_c: &'static str
) -> Result<bool, AMulBEqCError>
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
        .map_err_to_log( frame!() ) // this is a plain error, needs `to_log`
    ? )
}
```

Example for nested `#[cex]`:

```rust
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

assert_eq!( misc(), Err( CexErr::Code( 42 )));
```

Example for log level support in `#[cex]`:

```rust
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
```

## Guidelines

Put plain errors in public API's composite `Err` type, as long as they are
part of the design constraints. Changing them may cause compile errors in client
code, which is guaranteed by type system. These errors are regarded as intrinsic
errors of the API.

If the API author wants to add new errors in the future without breaking
compatibility, he could tag the enum with `#[non_exhausted]`.

To treat extrinsic errors as extrinsic, some trait object could be utilized to
erase their types and treat them as one variant in the composite error. For
example, if the API author want erase `std::io::Error` in the API, the function
signatures can be written as:

```rust
#[derive( EnumX, Debug )]
enum ReadU32Error {
    Parse( std::num::ParseIntError ),
    Other( cex::DynErr ),
}

#[cex]
fn read_u32( filename: &'static str )
    -> Result<u32,ReadU32Error>
{
    use std::io::Read;

    let mut f = std::fs::File::open( filename ).map_dyn_err()?;

    let mut s = String::new();
    f.read_to_string( &mut s ).map_dyn_err()?;
    let number = s.trim().parse::<u32>()?;
    Ok( number )
}
```

The `cex::DynErr` is a wrapper of some trait object. Current library
implementation utilizes `failure::Error`.

This is an example how the static approach, as the foundamental error-handling
mechanism, is able to work with dynamic dispatch approaches as needed. If users
had picked up the dynamic approach as the foundamental, it is not possible to
adopt the static approach if needed.

## Fallback to fat enum

A classic way of using `enum`s in error-handling is "wrapping errors". It
collects all the possible errors into a fat `enum`, as "the crate error", and
every public API in the crate will return `Result<T>`, which is a type alias
`type Result<T> = Result<T,crate::Error>;`.

The main advantage of the static approach over it, is expressing errors excactly
in function signatures. However, if the users feel it is too weight to use the
static approach, they can simply define a fat enum and use the result type alias
again. The code can still be benefited from logging mechanisms provided by the
static approach.

This is a proof that "wrapping errors" is a special case of the static approach.

# Detailed design

## Minimum structural enum

The `EnumX` derive is the key technology to support error conversions. It is
essentially a library-implemented language feature: a minimum supported
structural enum, aka enum exchange.

We use "minimum supported" because it does not support:

* Uniquifing, duplicated variant types to be uniquified.

* Flattening, `EnumX` enum nested in another one to be decoupled, making all
its variants bubbling up.

It is obvious that these are not possible to implement with `enum` unless type
system changes.

To avoid overlapping `impl`s in generialized plain errors, a phantom generic
argument is utilized.

```rust
pub trait IntoEnumx<Dest,Index> {
    fn into_enumx( self ) -> Dest;
}

pub trait MapError<T,Src>
    where Self : Into<Result<T,Src>>
{
    fn map_error<Dest,Indices>( self ) -> Result<T,Dest>
        where Src : Sized + IntoEnumx<Dest,Indices>
    {
        self.into().map_err( |src| src.into_enumx() )
    }
}
```

Neither `std::convert::From` nor `std::ops::Try` provides such phantom argument.
It is the root cause that `map_error()` is required.

## Interaction with other features

The static approach is the nature result of applying `EnumX` on `Err`,
implemented in `Result` combinators. So it won't cause issues other than
ergonomic, such as asyn issues, thread safety issues, etc.

# Drawbacks

* Static dispatch may have negative impact on rapid-development/prototyping.

* Highly orthogonal APIs are complex for beginners and occasional users.

* The requirement of `#[cex]` to be ergonomic.

# Rationale and alternatives

The static approach is best effort in separating mechanisms from policies. Any
other error-handling approaches could be considered as special cases and
utilized as fallback.

* Any trait-object based approach is flexible at cost of losing compile time
constraints and runtime penalty. The static approach can use them as a
supplement, but not vice-versa.

* Any other approach in static dispatch, is not as concise/powerful as the
proposed one, unless they mimics "structural enum".

# Prior art

The key idea of enum exchange, is inspired by "union types" in Typed Racket.

Its original implementation with generics support is referenced to
`frunk_core::coproduct`.

# Unresolved questions

Intrusive backtrace via traits implemented by plain errors are unresolved by
design.

# Future possibilities

`Ok` combinators(available in library implementation) may find the usecases in
the future.

Fully language-supported "union types" will make the implementation trivial, and
get rid of `#[cex]`.

Changing in `std::ops::Try` traits may help getting rid of `#[cex]` too.

# License

Licensed under MIT.
