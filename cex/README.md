# Summary

Introduce checked exception simulation in Rust, for refining the style of
putting all error types in a whole enum then using type alias `Result<T>` in
error-handling.

The new constructs are:

* `Cex` built upon _structural enum_ from `enumx` crate, and macros/traits for:

  - automatic error type convertion

  - throw point tracing and logging

* An optional `cex!{}` macro for syntax support:

  - `throws` in function signatures

  - shorthand notations `~`/`~~` for `.may_throw()`/`.may_rethrow()`.

# Motivation

A typical style of error-handling is so called ["wrapping errors"](https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html).

The procedure is as follows:

1. Collect all possible error types in a crate and putting them in an enum,
  perhaps named `Error` defined in `error.rs`, as "the crate's error type". Use
  `pub type Result<T> = Result<T,Error>` to simplify the function signatures.

2. Implement `From`s for the crate's error type, to do "up-casting" from actual
  error types; Maybe implement `std::error::Error` for those actual error types.

This method has some issues:

1. Using of type aliased `Result` effectively hide the actual error types,
  confusing programmers(including the author) when reading code or debugging.

2. Using of a fat enum as the `Err` for all functions adds unnecessary paths in
  error-handling, causing potentially inefficiencies.

3. Implementing `From`/`Error` trait brings boilerplate code.

# Features

The CeX project addresses all these issues listed above with features:

- Enumerating all the possible error types in function signatures.

- The users do not need to impl `From`s. If needed, a `#[derive(Exchange)]` is
  enough.

- No mandatory traits for actual error types. They could be or be not an
  `std::error::Error`, etc. If the user do want mandatory traits, trait bounds
  are up to the job.

- Integrated well with `Result` adaptors and `?`, though some sort of extra
  annotations may be requried, aka throw/rethrow, `~`/`~~`.

- `throws` syntax in principles of ergonomics, with fallbacks to vanilla Rust,
  to get better IDE support.

- Working with stable Rust.

# Overview

## Example: wrapping errors

We will see a program written in the "wrapping errors" style. It reads 3 `u32`
values a,b,c from 3 files respectively, then checks if they satisfied the
equation `a * b == c`.

```rust
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
```

Things worth noticing:

- The possible error types are hidden by `Result` type alias.

  ```rust
  fn read_u32( /**/ ) -> Result<u32> { /**/ }
  fn a_mul_b_eq_c( /**/ ) -> Result<bool> { /**/ }
  ```

  Programmers are not able to know the actual errors of a certain function by
  glancing over its signatures, unless they check the call chain recursively. In
  a real-word project, errors may be propagated through quite deep call stack,
  and manually checking the errors is infeasible for humans.

- The error types are not _accurate_

  Although `read_u32()` does not do calculating at all, we need to deal with
  the `Error::Calc` branch and write `unreachable!()` code.

  ```rust
  Err( error ) => match error {
      Error::IO(     _ ) => 0, // default to 0 if file is missing.
      Error::Parse(  _ ) => return Err( error ),
      Error::Calc( _,_ ) => {
          unreachable!(); // read_u32 does not do calculating at all!
      },
  },
  ```

  Even worse, any public API returning `Result<T>` will force the downstream
  users writing such code.

- Boilerplate code for trait `impl`s.

## Example: checked exception

We will rewrite this program in "checked exception" style. The `throws` syntax
and other syntatic sugar are utilized for demonstration purpose. However, **the
users are free to pick vanilla Rust equivalents as a fallback.**

- Introducing `throws` in function signatures:

```rust
fn read_u32( filename: &'static str ) -> u32
    throws IO(    std::io::Error )
         , Parse( std::num::ParseIntError )
{ /**/ }
```

```rust
#[derive( Debug, PartialEq, Eq )]
pub struct MulOverflow( pub u32, pub u32 );

fn a_mul_b_eq_c(
    file_a: &'static str,
    file_b: &'static str,
    file_c: &'static str )
    -> bool
    throws IO(    std::io::Error )
         , Parse( std::num::ParseIntError )
         , Calc(  MulOverflow )
{ /**/ }
```

We consider `read_u32()` and `a_mul_b_eq_c()` as functions returning checked
exceptions, aka "cex functions", because they use `throws` in signatures. Those
who don't, such as `std::fs::File::open()`, are returning plain errors.

- Pattern matching on a cex function's result

As a cex function, `read_u32()` returns a `Result` of which `Err` is
`read_u32::Err`.

```rust
let c = match read_u32( file_c ) {
    Ok(  value ) => value,
    Err( cex   ) => match cex.error {
        read_u32::Err::IO( _      ) => 0, // default to 0 if file is missing.
        read_u32::Err::Parse( err ) => throw!( err ),
    },
};
```

- Annotations for distinguish between functions returning plain errors and ones
  returning checked exceptions.

  1. Use a postfix `~` to propagate a plain error to a cex function.
  For exameple, the statement `let mut f = std::fs::File::open( filename )?;`
  will be rewritten as `let mut f = std::fs::File::open( filename )~?;`

  2. Use a postfix `~~` to propagate checked exceptions to a cex function.
  For exameple, the statement `let a = read_u32( file_a )?;` will be rewritten
  as `let a = read_u32( file_a )~~?;`

- Unconditionally throw/rethrow

  1. Use `throw!()` to do early exit with a plain error in cex functions.
  Instead of `return Err( error )`, we will write `throw!( err )`.

  2. Use `rethrow!()` to do early exit with checked exceptions in cex functions.
  Instead of `return Err( error )`, we will write `rethrow!( err )`.

  3. Use `throw_log!()` and `rethrow_log!()` when you need to track the throw
  point or attach extra text in the log.

All the magic syntax support for `throws` and `~`/`~~` are provided by `cex!{}`
from `cex_derive` crate.

Put it all together:

```rust
use cex_derive::cex;

cex! {
    fn read_u32( filename: &'static str ) -> u32
        throws IO(    std::io::Error )
             , Parse( std::num::ParseIntError )
    {
        use std::io::Read;

        let mut f = std::fs::File::open( filename )~?;
        let mut s = String::new();
        f.read_to_string( &mut s )~?;
        let number = s.trim().parse::<u32>()
            .may_throw_log( log!( "fail in parsing {} to u32", s.trim() ))?;
        Ok( number )
    }
}

#[derive( Debug, PartialEq, Eq )]
pub struct MulOverflow( pub u32, pub u32 );

cex!{
    fn a_mul_b_eq_c(
        file_a: &'static str,
        file_b: &'static str,
        file_c: &'static str )
        -> bool
        throws IO(    std::io::Error )
             , Parse( std::num::ParseIntError )
             , Calc(  MulOverflow )
    {
        let a = read_u32( file_a )~~?;

        let b = match read_u32( file_b ) {
            Ok(  value ) => value,
            Err( cex   ) => {
                if a == 0 {
                    0 // 0 * b == 0, no matter what b is.
                } else {
                    rethrow_log!( cex );
                }
            },
        };
 
        let c = match read_u32( file_c ) {
            Ok(  value ) => value,
            Err( cex   ) => match cex.error {
                read_u32::Err::IO( _ ) => 0, // default to 0 if file is missing.
                read_u32::Err::Parse( err ) => throw!( err ),
            },
        };

        a.checked_mul( b )
         .ok_or( MulOverflow(a,b) )
         .may_throw_log( log!( "u32 overflow: {} * {}", a, b ))
         .map( |result| result == c )
    }
}
```

# Desugaring `~`/`~~`

- A `~` not followed by another `~`, is short for `.may_throw()`.

- A `~~` is short for `.may_rethrow()`.

- `may_throw_log()`/`may_rethrow_log()` are similar functions in addition to
  support logging.

# Desugaring `throws`

A function using `throws`

```rust
cex! {
    fn foo( /**/ ) -> Type
        throws SomeErr( SomeErrType )
             , AnotherErr( AnotherErrType )
             /*...*/
    { /**/ }
}
```

is desugared as

```rust
mod foo {
    use super::*;
    use enumx::prelude::*;
    #[derive( enumx_derive::Exchange, Debug )]
    pub enum Err {
        SomeErr( SomeErrType ),
        AnotherErr( AnotherErrType ),
        /*...*/
    }
}

fn foo( /**/ ) -> Result<Type, Cex<foo::Err>> { /**/ }
```

Things worth noticing:

- `Debug` trait is mandatory for actural error types when using `cex!{}`.

- `throws` syntax is similar with enum variant definitions, with one limitation
  that all variant type should be "newtype form".

# Issues with `throws` syntax

- Assumes that a `mod` with the same name not defined.

- Potentially poor IDE support.

- Implicit `Result` type.

Alternatives will be discussed in the following sections, to address these
issues in situations that they really matter.

# Named checked exception

While `cex!{}` generates an enum definition for users, they have the chance to
define it themselves. For example, if the `mod` with the same name as the cex
function has already be defined, the user should avoid using `cex!{}`.

```rust
#[ derive( Exchange, Debug )]
enum ReadU32Error {
    IO( std::io::Error ),
    Parse( std::num::ParseIntError ),
}

fn read_u32( filename: &'static str )
    -> Result<u32, Cex<ReadU32Error>>
{ /**/ }
```

```rust
let c = match read_u32( file_c ) {
    Ok(  value ) => value,
    Err( cex   ) => match cex.error {
        ReadU32Error::IO( _ ) => 0, // default to 0 if file is missing.
        ReadU32Error::Parse( err ) => throw!( err ),
    },
};
```

However, the error types are not listed in signatures now. But the user can
still be able to check the corresponding enum definition to get them.

The complete code is in
["named" test case](https://github.com/oooutlk/enumx/blob/master/cex/src/test/named.rs).

# Unnamed checked exception

If the users are not willing to write an enum definition, they can use
predefined enums as an alternative.

```rust
fn read_u32( filename: &'static str )
    -> Result<u32, Throws!( std::io::Error, std::num::ParseIntError )>
{ /**/ }
```

However, the pattern matching is less ergonomics because the users have to
count the errors themselves. And pattern matching is subject to the order of
error definition.

```rust
let c = match read_u32( file_c ) {
    Ok(  value ) => value,
    Err( cex   ) => match cex.error {
        Enum2::_0( _   ) => 0, // default to 0 if file is missing.
        Enum2::_1( err ) => throw!( err ),
    },
};
```

The complete code is in
["adhoc" test case](https://github.com/oooutlk/enumx/blob/master/cex/src/test/adhoc.rs).

For users not willing to use any macro, `read_u32()` could be rewritten as:

```rust
fn read_u32( filename: &'static str )
    -> Result<u32, Cex<Enum2< std::io::Error, std::num::ParseIntError >>>
{ /**/ }
```

# Ok-wrapping

To address the issue of implicitly returning `Result`, some kind of ok-wrapping
mechanism could be implemented in library. However, I wonder if it is really
helpful for end users and worth writing hundreds lines of code to implement.

# Guildlines for interacting with other libraries

1. If other libraries do not use cex, their error types ar considered as plain
errors. Use `throw`/`~` and similar constructs to deal with them.

2. If other libraries use cex, their error types ar considered as checked
exceptions. Use `rethrow`/`~~` and similar constructs to deal with them.

3. Use checked exceptions as constriants in public API. Changes in checked
exceptions returned by your APIs must break the downstream's code to notice
the changes. If backward compatibility should be guaranteed, use
`#[non_exhausted]` with your enums to enforce an `_` match arm in client code.

# Future possibilities

A conservative syntax may be introduced as an alternative of `throws`.

```rust
#[cex]
fn foo( /**/ ) -> Result<Type, Throws!( Bar(BarType), Baz(BazType) )> { /**/ }
```

# License

Licensed under MIT.
