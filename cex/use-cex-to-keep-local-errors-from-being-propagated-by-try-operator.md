# Use cex to keep local errors from being propagated by `?` operator

## The issue of "global error type" with `?` operator

The sled project published
[Error Handling in a Correctness-Critical Rust Project](https://sled.rs/errors)
, introducing its error handling method. Summarize as follows:

1. Errors can be categorized into fatal and non-fatal(local). We must propagate
fatal errors and handle the local errors.

2. Most catastrophic system failures are the result of incorrect handling of
non-fatal errors.

3. There is a tendency in the Rust community to throw all errors into a single
global error type, which is a big enum that holds the various possible errors
that may have been encountered at any point anywhere in the program.

4. The `?` operator is convenient for propagating errors. As code changes over
time, a `may_throw_local_errors()?` may be added in the body of a
`throws_only_fatal_errors()` by accident. A "big enum" in the latter's
signature cannot help the compiler to catch this kind of mistakes.

## Example of propagating local errors by accident

```rust
pub struct FatalError1;
pub struct FatalError2;
pub struct LocalError1;
pub struct LocalError2;

// The global error type is a big enum
pub enum Error {
    A( FatalError1 ),
    B( FatalError2 ),
    C( LocalError1 ),
    D( LocalError2 ),
}

fn handle_c( c: LocalError1 );
fn handle_d( d: LocalError2 );

fn throws_only_fatal_errors_v1() -> Result<(), Error> {
    may_throw_a()?; // if returns Err, only Err::A is possible. The same below.
    may_throw_b()?;
    may_throw_ab()?;

    may_throw_abcd().map_err( |e| match e {
        Error::C(c) => handle_c(c),
        Error::D(d) => handle_d(d),
        _ => (),
    });
}

// All is fine until code changes
fn returns_only_fatal_errors_v2() -> Result<(), Error> {
    may_throw_a()?;
    may_throw_b()?;
    may_throw_c()?;   // <----- unhandled local error propagated
    may_throw_ab()?;

    may_throw_abcd().map_err( |e| match e {
        Error::C(c) => handle_c(c),
        Error::D(d) => handle_d(d),
        _ => (),
    });
}
```

## Don't put errors in `Err` variant that are not expected to be propagated

The sled's article suggested using nested `Result`s to separate local errors
from fatal errors:

```rust
pub struct A;
pub struct B;
pub struct C;
pub struct D;

pub enum FatalError {
    A( A ),
    B( A ),
}

pub enum LocalError {
    C( C ),
    D( D ),
}

fn handle_c( c: C );
fn handle_d( d: D );

fn may_throw_a() -> Result<(), A>;
fn may_throw_b() -> Result<(), B>;
fn may_throw_c() -> Result<(), C>;
fn may_throw_ab() -> Result<(), LocalError>;
fn may_throw_abcd() -> Result<Result<(),LocalError>, FatalError>; // nested!

fn returns_only_fatal_errors_v3() -> Result<(), FatalError> {
    may_throw_a()?;
    may_throw_b()?;
    // may_throw_c()?; // <----- compile error
    may_throw_ab()?;

    may_throw_abcd()?
    .or_else( |e| match e {
        Error::C(c) => Ok( handle_c( c )),
        Error::D(d) => Ok( handle_d( d )),
    })
}
```

## Use checked-exception syntax to make error types flatterned

The [cex crate](https://crates.io/crates/cex) emulates checked exception
handling in Rust. The possible error types are enumerated in function signature:

```rust
#[cex]
fn foo() -> Result!( Type throws ErrorA, ErrorB,.. );
```

Let's use cex to do code refactoring:

```rust
use cex::*;

pub struct A;
pub struct B;
pub struct C;
pub struct D;

fn handle_c( c: C );
fn handle_d( d: D );

#[cex] fn may_throw_a()    -> Result!( () throws A   );
#[cex] fn may_throw_b()    -> Result!( () throws B   );
#[cex] fn may_throw_c()    -> Result!( () throws C   );
#[cex] fn may_throw_ab()   -> Result!( () throws A,B );
#[cex] fn may_throw_abcd() -> Result!(
                 Result!( () throws C,D ) throws A,B );

#[cex]
fn returns_only_fatal_errors_v4() -> Result!( () throws A,B ) {
    may_throw_a()?;
    may_throw_b()?;
    // may_throw_c()?; // compile error, too
    may_throw_ab()?;

    may_throw_abcd()?
    .or_else( |e| #[ty_pat] match e { // ty_pat means type pattern match
        C(c) => Ok( handle_c( c )),
        D(d) => Ok( handle_d( d )),
    })
}
```

If you are not big fans of nested `Result`s, a `#[ty_pat(gen_throws)]` can be
used with a flat `Result` type.

```rust
#[cex] fn may_throw_abcd_v2() -> Result!( () throws A,B,C,D );

#[cex]
fn returns_only_fatal_errors_v5() -> Result!( () throws A,B ) {
    may_throw_a()?;
    may_throw_b()?;
    // may_throw_c()?; // compile error
    may_throw_ab()?;

    may_throw_abcd_v2()
    .or_else( |e| #[ty_pat(gen_throws)] match e { // generates arms to throw A,B
        C(c) => Ok( handle_c( c )),
        D(d) => Ok( handle_d( d )),
    })
}
```

The rules about fatal/local errors when using cex:

1. a `#[cex] fn` propagates fatal errors enumerated in its signature.

2. local errors not enumerated in `Result!()`'s throws type list are handled
inside the function body, or some compile errors/warnings will emit, such as
"patterns not covered" or "unused `std::result::Result` that must be used".
